use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};

use crate::core::models::Wallpaper;
use crate::ui::screens::resolution_select::{CropMode, ResolutionOption};

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub wallpaper: Wallpaper,
    pub status: DownloadStatus,
    pub progress: u8,
    pub save_path: PathBuf,
    pub cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    pub resolution: Option<ResolutionOption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Queued,
    Active,
    Completed,
    Failed(String),
}

impl DownloadTask {
    pub fn new(wallpaper: Wallpaper, save_path: PathBuf) -> Self {
        Self {
            wallpaper,
            status: DownloadStatus::Queued,
            progress: 0,
            save_path,
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            resolution: None,
        }
    }

    pub fn request_cancel(&self) {
        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_flag.load(std::sync::atomic::Ordering::SeqCst)
    }
}

pub struct DownloadManager {
    queue: Arc<Mutex<Vec<DownloadTask>>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn enqueue(&self, task: DownloadTask) {
        self.queue.lock().await.push(task);
    }

    pub async fn tasks(&self) -> Vec<DownloadTask> {
        self.queue.lock().await.clone()
    }

    pub async fn cancel(&self, index: usize) {
        let queue = self.queue.lock().await;
        if let Some(task) = queue.get(index) {
            task.request_cancel();
        }
    }

    pub async fn remove_completed(&self) {
        let mut queue = self.queue.lock().await;
        queue.retain(|t| t.status != DownloadStatus::Completed);
    }

    pub async fn retry(&self, index: usize) {
        let mut queue = self.queue.lock().await;
        if let Some(task) = queue.get_mut(index)
            && matches!(task.status, DownloadStatus::Failed(_))
        {
            task.status = DownloadStatus::Queued;
            task.progress = 0;
            task.cancel_flag
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub async fn len(&self) -> usize {
        self.queue.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.queue.lock().await.is_empty()
    }

    pub fn start_worker(self: &Arc<Self>, tx: mpsc::Sender<DownloadEvent>) {
        let manager = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                let task_idx = {
                    let queue = manager.queue.lock().await;
                    queue
                        .iter()
                        .position(|t| t.status == DownloadStatus::Queued)
                };

                if let Some(idx) = task_idx {
                    {
                        let mut queue = manager.queue.lock().await;
                        queue[idx].status = DownloadStatus::Active;
                    }

                    let (wallpaper, save_path, cancel_flag, resolution) = {
                        let queue = manager.queue.lock().await;
                        let task = &queue[idx];
                        (
                            task.wallpaper.clone(),
                            task.save_path.clone(),
                            Arc::clone(&task.cancel_flag),
                            task.resolution.clone(),
                        )
                    };

                    let result =
                        download_with_progress(&wallpaper, &save_path, &cancel_flag, &tx, idx, resolution.as_ref())
                            .await;

                    let mut queue = manager.queue.lock().await;
                    if let Some(task) = queue.get_mut(idx) {
                        match result {
                            Ok(()) => {
                                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                                    task.status = DownloadStatus::Failed("Cancelled".to_string());
                                } else {
                                    task.status = DownloadStatus::Completed;
                                    task.progress = 100;
                                    let _ = tx.send(DownloadEvent::Completed(idx)).await;
                                }
                            }
                            Err(e) => {
                                task.status = DownloadStatus::Failed(e);
                            }
                        }
                    }
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        });
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Progress(usize, u8),
    Completed(usize),
}

async fn download_with_progress(
    wallpaper: &Wallpaper,
    save_path: &std::path::Path,
    cancel_flag: &Arc<std::sync::atomic::AtomicBool>,
    tx: &mpsc::Sender<DownloadEvent>,
    task_idx: usize,
    resolution: Option<&ResolutionOption>,
) -> Result<(), String> {
    let url = &wallpaper.url;

    for attempt in 0..3 {
        if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }

        match try_download(url, save_path, cancel_flag, tx, task_idx, resolution).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                if attempt < 2 {
                    let delay = tokio::time::Duration::from_secs(2u64.pow(attempt));
                    tokio::time::sleep(delay).await;
                } else {
                    return Err(format!("Failed after 3 attempts: {e}"));
                }
            }
        }
    }
    Err("Max retries exceeded".to_string())
}

async fn try_download(
    url: &str,
    save_path: &std::path::Path,
    cancel_flag: &Arc<std::sync::atomic::AtomicBool>,
    tx: &mpsc::Sender<DownloadEvent>,
    task_idx: usize,
    resolution: Option<&ResolutionOption>,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    if let Some(parent) = save_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }

    let mut file = tokio::fs::File::create(save_path)
        .await
        .map_err(|e| e.to_string())?;
    use tokio::io::AsyncWriteExt;

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    let mut buffer = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;

        if let Some(progress) = downloaded
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_size))
        {
            let _ = tx
                .send(DownloadEvent::Progress(task_idx, progress as u8))
                .await;
        }

        if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            let _ = tokio::fs::remove_file(save_path).await;
            return Err("Cancelled".to_string());
        }
    }

    if let Some(res_opt) = resolution {
        let img = image::load_from_memory(&buffer).map_err(|e| e.to_string())?;
        let (target_w, target_h) = res_opt.dimensions();
        let processed = match res_opt {
            ResolutionOption::Original => {
                file.write_all(&buffer).await.map_err(|e| e.to_string())?;
                file.flush().await.map_err(|e| e.to_string())?;
                return Ok(());
            }
            ResolutionOption::Custom(_, _, _) |
            ResolutionOption::HD720(_, _) |
            ResolutionOption::FHD1080(_, _) |
            ResolutionOption::QHD1440(_, _) |
            ResolutionOption::UHD2160(_, _) |
            ResolutionOption::Ultrawide2560(_, _) |
            ResolutionOption::Ultrawide3440(_, _) |
            ResolutionOption::MacBook16(_, _) |
            ResolutionOption::Phone1080(_, _) => {
                match res_opt.crop_mode() {
                    CropMode::Scale => {
                        img.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3)
                    }
                    CropMode::CropCenter => {
                        let resized = img.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3);
                        resized
                    }
                    CropMode::Fit => {
                        img.resize(target_w, target_h, image::imageops::FilterType::Lanczos3)
                    }
                }
            }
        };
        let mut out_buf = Vec::new();
        let ext = save_path.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
        match ext {
            "png" => {
                processed.write_to(&mut std::io::Cursor::new(&mut out_buf), image::ImageFormat::Png).map_err(|e| e.to_string())?;
            }
            "webp" => {
                processed.write_to(&mut std::io::Cursor::new(&mut out_buf), image::ImageFormat::WebP).map_err(|e| e.to_string())?;
            }
            _ => {
                processed.write_to(&mut std::io::Cursor::new(&mut out_buf), image::ImageFormat::Jpeg).map_err(|e| e.to_string())?;
            }
        }
        file.write_all(&out_buf).await.map_err(|e| e.to_string())?;
    } else {
        file.write_all(&buffer).await.map_err(|e| e.to_string())?;
    }

    file.flush().await.map_err(|e| e.to_string())?;
    Ok(())
}

pub fn generate_filename(wallpaper: &Wallpaper) -> String {
    let provider = wallpaper.provider.to_string().to_lowercase();
    let id = sanitize_id(&wallpaper.id);
    let dimensions = match (wallpaper.width, wallpaper.height) {
        (Some(w), Some(h)) => format!("{w}x{h}"),
        _ => "unknown".to_string(),
    };

    let extension = get_extension(wallpaper);

    format!("{provider}_{id}_{dimensions}.{extension}")
}

fn get_extension(wallpaper: &Wallpaper) -> &'static str {
    if let Some(file_type) = &wallpaper.file_type {
        match file_type.as_str() {
            "image/jpeg" => return "jpg",
            "image/png" => return "png",
            "image/webp" => return "webp",
            "image/gif" => return "gif",
            _ => {}
        }
    }

    if let Some(idx) = wallpaper.url.rfind('.') {
        let ext = wallpaper.url[idx + 1..].to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" => return "jpg",
            "png" => return "png",
            "webp" => return "webp",
            "gif" => return "gif",
            _ => {}
        }
    }

    "jpg"
}

fn sanitize_id(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}
