use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::core::models::Wallpaper;

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub wallpaper: Wallpaper,
    pub status: DownloadStatus,
    pub progress: u8,
    pub save_path: PathBuf,
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
        }
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
        let mut queue = self.queue.lock().await;
        if index < queue.len() {
            queue.remove(index);
        }
    }

    pub async fn len(&self) -> usize {
        self.queue.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.queue.lock().await.is_empty()
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
