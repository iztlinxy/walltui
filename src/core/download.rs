use std::path::PathBuf;

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
