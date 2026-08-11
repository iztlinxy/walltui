use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Stdio;
use std::future::Future;
use tokio::sync::mpsc;
use tokio::process::Command;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use serde::Deserialize;

use crate::core::errors::AppError;
use crate::core::models::{Provider, SearchQuery, Wallpaper};

#[derive(Debug, Deserialize)]
struct YtDlpInfo {
    id: String,
    title: String,
    duration: Option<f64>,
    uploader: Option<String>,
    thumbnail: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

pub struct YtDlpAdapter {
    pub output_dir: PathBuf,
}

impl YtDlpAdapter {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    pub async fn fetch_metadata(&self, url: &str) -> Result<Wallpaper, String> {
        ensure_ytdlp().await?;

        let output = Command::new("yt-dlp")
            .args([
                "--dump-json",
                "--no-download",
                "--no-warnings",
                url,
            ])
            .output()
            .await
            .map_err(|e| format!("yt-dlp not found. Install it: pip install yt-dlp ({e})"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return parse_ytdlp_error(&stderr);
        }

        let json = String::from_utf8_lossy(&output.stdout);
        let info: YtDlpInfo = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse yt-dlp output: {e}"))?;

        let duration_secs = info.duration.map(|d| d as u64);
        let _duration_display = duration_secs
            .map(|d| format!("{}s", d))
            .unwrap_or_else(|| "unknown".to_string());

        let id = sanitize_id(&info.id);
        let wallpaper = Wallpaper {
            id: id.clone(),
            provider: Provider::YtDlp,
            url: url.to_string(),
            thumb_url: info.thumbnail.clone().unwrap_or_default(),
            title: info.title.clone(),
            photographer: info.uploader.clone().unwrap_or_else(|| "Unknown".to_string()),
            width: info.width,
            height: info.height,
            avg_color: None,
            attribution: None,
            file_type: Some("video/mp4".to_string()),
            web_url: Some(url.to_string()),
            tags: Vec::new(),
            category: None,
            purity: None,
            views: None,
            favorites: None,
            is_video: true,
            duration_secs,
            clip_start_secs: None,
            clip_end_secs: None,
        };

        Ok(wallpaper)
    }

    pub async fn download_clip(
        &self,
        url: &str,
        start_secs: u64,
        duration_secs: u64,
        progress_tx: mpsc::Sender<(u8, String)>,
    ) -> Result<PathBuf, String> {
        ensure_ytdlp().await?;

        let video_dir = self.output_dir.join("videos");
        std::fs::create_dir_all(&video_dir)
            .map_err(|e| format!("Failed to create video directory: {e}"))?;

        let temp_file = video_dir.join(format!("yt_dlp_{}.mp4", sanitize_id(url)));

        let _ = progress_tx.send((2, "Resolving URL...".to_string())).await;

        let mut child = Command::new("yt-dlp")
            .args([
                "--format",
                "bestvideo[ext=mp4][height<=1080]+bestaudio[ext=m4a]/best[ext=mp4]/best",
                "--newline",
                "--no-playlist",
                "--no-warnings",
                "--merge-output-format",
                "mp4",
                "--retries",
                "3",
                "--socket-timeout",
                "30",
                "--output",
                temp_file.to_str().unwrap(),
                url,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start yt-dlp: {e}"))?;

        let _ = progress_tx.send((5, "Downloading video...".to_string())).await;

        let stderr = child.stderr.take();
        if let Some(stderr) = stderr {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        parse_download_progress(&line, &progress_tx).await;
                    }
                    Err(_) => break,
                }
            }
        }

        let status = child.wait().await.map_err(|e| format!("yt-dlp failed: {e}"))?;
        if !status.success() {
            return Err("yt-dlp download failed".to_string());
        }

        if !temp_file.exists() {
            return Err("Downloaded file not found".to_string());
        }

        let _ = progress_tx.send((55, "Extracting clip...".to_string())).await;

        let clipped_file = video_dir.join(format!("yt_dlp_clipped_{}.mp4", sanitize_id(url)));
        ffmpeg_extract_clip(&temp_file, &clipped_file, start_secs, duration_secs, &progress_tx).await?;

        let _ = progress_tx.send((75, "Converting to H.264 1080p...".to_string())).await;

        let output_file = video_dir.join(format!(
            "yt_dlp_{}_{}s_1920x1080.mp4",
            sanitize_id(url),
            duration_secs
        ));

        ffmpeg_convert(&clipped_file, &output_file, &progress_tx).await?;

        let _ = std::fs::remove_file(&temp_file);
        let _ = std::fs::remove_file(&clipped_file);

        Ok(output_file)
    }

    pub fn validate_duration(duration_secs: Option<u64>) -> Result<(), String> {
        match duration_secs {
            Some(d) if d < 20 => Err(format!("Video too short: {}s (minimum 20s)", d)),
            Some(d) if d > 60 => Err(format!("Video too long: {}s (maximum 60s)", d)),
            Some(_) => Ok(()),
            None => Err("Unknown video duration".to_string()),
        }
    }
}

async fn ensure_ytdlp() -> Result<(), String> {
    let result = Command::new("yt-dlp")
        .arg("--version")
        .output()
        .await;

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(
            "yt-dlp not found. Install it with:\n  pip install yt-dlp\n  or download from https://github.com/yt-dlp/yt-dlp".to_string()
        ),
    }
}

async fn ensure_ffmpeg() -> Result<(), String> {
    let result = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await;

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(
            "ffmpeg not found. Install it from https://ffmpeg.org/download.html".to_string()
        ),
    }
}

async fn ffmpeg_extract_clip(
    input: &Path,
    output: &Path,
    start_secs: u64,
    duration_secs: u64,
    _progress_tx: &mpsc::Sender<(u8, String)>,
) -> Result<(), String> {
    ensure_ffmpeg().await?;

    let ffmpeg_output = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            &start_secs.to_string(),
            "-t",
            &duration_secs.to_string(),
            "-i",
            input.to_str().unwrap(),
            "-c",
            "copy",
            output.to_str().unwrap(),
        ])
        .output()
        .await
        .map_err(|e| format!("ffmpeg clip extract failed: {e}"))?;

    if !ffmpeg_output.status.success() {
        let stderr = String::from_utf8_lossy(&ffmpeg_output.stderr);
        return Err(format!("ffmpeg clip extract failed: {}", stderr.lines().take(2).collect::<Vec<_>>().join(" ")));
    }

    if !output.exists() {
        return Err("Clipped file not created".to_string());
    }

    Ok(())
}

async fn ffmpeg_convert(
    input: &Path,
    output: &Path,
    progress_tx: &mpsc::Sender<(u8, String)>,
) -> Result<(), String> {
    let total_duration = get_video_duration_ms(input).await.unwrap_or(0);

    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            input.to_str().unwrap(),
            "-c:v",
            "libx264",
            "-preset",
            "medium",
            "-crf",
            "23",
            "-vf",
            "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2",
            "-an",
            "-movflags",
            "+faststart",
            "-progress",
            "pipe:1",
            output.to_str().unwrap(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("ffmpeg spawn failed: {e}"))?;

    let stdout = child.stdout.take();
    if let Some(stdout) = stdout {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    parse_ffmpeg_progress_with_total(&line, total_duration, progress_tx).await;
                }
                Err(_) => break,
            }
        }
    }

    let status = child.wait().await.map_err(|e| format!("ffmpeg failed: {e}"))?;
    if !status.success() {
        let stderr = child.stderr.take();
        if let Some(mut stderr) = stderr {
            let mut buf = String::new();
            let _ = stderr.read_to_string(&mut buf).await;
            if !buf.is_empty() {
                return Err(format!("ffmpeg conversion failed: {}", buf.lines().take(2).collect::<Vec<_>>().join(" ")));
            }
        }
        return Err("ffmpeg conversion failed".to_string());
    }

    if !output.exists() {
        return Err("Converted file not created".to_string());
    }

    let _ = progress_tx.send((100, "Complete!".to_string())).await;

    Ok(())
}

fn parse_ytdlp_error(stderr: &str) -> Result<Wallpaper, String> {
    let lower = stderr.to_lowercase();
    if lower.contains("private") || lower.contains("members-only") {
        Err("Video is private or members-only".to_string())
    } else if lower.contains("age") || lower.contains("sign in") {
        Err("Video is age-restricted. yt-dlp may need --cookies".to_string())
    } else if lower.contains("geo") || lower.contains("country") {
        Err("Video is geo-blocked in your region".to_string())
    } else if lower.contains("not available") || lower.contains("unavailable") {
        Err("Video is not available".to_string())
    } else {
        let msg = stderr.lines().next().unwrap_or("Unknown yt-dlp error");
        Err(msg.to_string())
    }
}

async fn parse_download_progress(
    output: &str,
    progress_tx: &mpsc::Sender<(u8, String)>,
) {
    let line = output.trim();
    if line.is_empty() {
        return;
    }

    if line.contains("[download]") && line.contains('%') {
        if let Some(pct) = extract_percentage(line) {
            let mapped = 5 + (pct as u64 * 45 / 100) as u8;
            let speed = extract_speed(line);
            let eta = extract_eta(line);
            let status = match (speed, eta) {
                (Some(s), Some(e)) => format!("Downloading: {}% ({}, {})", pct, s, e),
                (Some(s), None) => format!("Downloading: {}% ({})", pct, s),
                _ => format!("Downloading: {}%", pct),
            };
            let _ = progress_tx.send((mapped, status)).await;
        }
    } else if line.contains("[download]") || line.contains("Downloading") {
        let _ = progress_tx.send((3, "Starting download...".to_string())).await;
    } else if line.contains("[ExtractAudio]") || line.contains("Merger") {
        let _ = progress_tx.send((50, "Merging video and audio...".to_string())).await;
    } else if line.contains("[Fixup") || line.contains("Fixing") {
        let _ = progress_tx.send((52, "Finalizing...".to_string())).await;
    }
}

fn extract_speed(line: &str) -> Option<String> {
    for part in line.split_whitespace() {
        if part.contains("MiB/s") || part.contains("KiB/s") || part.contains("GiB/s") {
            return Some(part.to_string());
        }
    }
    None
}

fn extract_eta(line: &str) -> Option<String> {
    for part in line.split_whitespace() {
        if part.contains(':') && (part.contains("ETA") || line.contains("ETA")) {
            let idx = line.find("ETA")?;
            return Some(line[idx..].split_whitespace().take(2).collect::<Vec<_>>().join(" "));
        }
    }
    None
}

fn extract_percentage(line: &str) -> Option<u8> {
    for part in line.split_whitespace() {
        if part.ends_with('%') {
            if let Ok(val) = part.trim_end_matches('%').parse::<u8>() {
                return Some(val);
            }
        }
    }
    None
}

async fn get_video_duration_ms(path: &Path) -> Option<u64> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "csv=p=0",
            path.to_str().unwrap(),
        ])
        .output()
        .await
        .ok()?;

    if output.status.success() {
        let dur_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Ok(dur) = dur_str.parse::<f64>() {
            return Some((dur * 1000.0) as u64);
        }
    }
    None
}

async fn parse_ffmpeg_progress_with_total(
    line: &str,
    total_duration_ms: u64,
    progress_tx: &mpsc::Sender<(u8, String)>,
) {
    let line = line.trim();
    if line.starts_with("out_time_ms=") {
        if let Ok(ms) = line.strip_prefix("out_time_ms=").unwrap_or("0").parse::<u64>() {
            let pct = if total_duration_ms > 0 {
                (ms as u64 * 25 / total_duration_ms) as u8
            } else {
                0
            };
            let mapped = 75 + pct.min(24);
            let secs = ms / 1000;
            let status = format!("Converting: {}s processed", secs);
            let _ = progress_tx.send((mapped, status)).await;
        }
    } else if line.starts_with("frame=") {
        if let Some(frame_str) = line.strip_prefix("frame=") {
            if let Ok(frame) = frame_str.trim().split_whitespace().next().unwrap_or("0").parse::<u64>() {
                let status = format!("Converting: frame {}...", frame);
                let _ = progress_tx.send((75, status)).await;
            }
        }
    } else if line.starts_with("speed=") {
        let speed = line.strip_prefix("speed=").unwrap_or("");
        let status = format!("Converting ({})...", speed);
        let _ = progress_tx.send((75, status)).await;
    }
}

pub fn sanitize_id(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .take(50)
        .collect()
}

pub struct YtDlpProviderAdapter {
    adapter: YtDlpAdapter,
}

impl YtDlpProviderAdapter {
    pub fn new() -> Self {
        let output_dir = dirs::home_dir()
            .map(|h| h.join("Pictures").join("walltui"))
            .unwrap_or_else(|| PathBuf::from("."));
        Self {
            adapter: YtDlpAdapter::new(output_dir),
        }
    }
}

impl Default for YtDlpProviderAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::providers::ProviderAdapter for YtDlpProviderAdapter {
    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Wallpaper>, AppError>> + Send + 'a>> {
        Box::pin(async move {
            let url = &query.query;
            match self.adapter.fetch_metadata(url).await {
                Ok(wallpaper) => Ok(vec![wallpaper]),
                Err(e) => Err(AppError::Provider(e)),
            }
        })
    }

    fn download_url(&self, wallpaper: &Wallpaper) -> Result<String, AppError> {
        Ok(wallpaper.url.clone())
    }

    fn name(&self) -> &'static str {
        "yt-dlp"
    }
}
