use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::models::{GalleryEntry, Wallpaper};
use crate::infrastructure::config_loader::data_path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GalleryIndex {
    pub version: u32,
    #[serde(default)]
    pub entries: HashMap<String, GalleryEntry>,
}

fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl GalleryIndex {
    pub fn load() -> Self {
        let path = data_path();
        if path.exists()
            && let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(index) = serde_json::from_str(&content)
        {
            return index;
        }
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = data_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(&path, content)
    }

    pub fn rename(
        &mut self,
        old_filename: &str,
        new_filename: &str,
        new_path: PathBuf,
    ) -> Result<(), String> {
        let mut entry = self
            .entries
            .remove(old_filename)
            .ok_or_else(|| "Entry not found".to_string())?;
        entry.custom_name = Some(strip_extension(new_filename).to_string());
        entry.date_modified = now_ts();
        entry.wallpaper.id = new_filename.to_string();
        entry.wallpaper.url = new_path.to_string_lossy().to_string();
        entry.wallpaper.thumb_url = entry.wallpaper.url.clone();
        entry.wallpaper.title = new_filename.to_string();
        self.entries.insert(new_filename.to_string(), entry);
        self.save().map_err(|e| e.to_string())
    }

    pub fn display_name(&self, wallpaper: &Wallpaper) -> String {
        self.entries
            .get(&wallpaper.id)
            .map(|e| e.display_name().to_string())
            .unwrap_or_else(|| wallpaper.title.clone())
    }
}

fn strip_extension(name: &str) -> &str {
    Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name)
}
