use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::models::Provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub wallhaven_api_key: Option<String>,
    pub download_dir: PathBuf,
    pub default_provider: Provider,
    pub purity_sfw: bool,
    pub purity_sketchy: bool,
    pub purity_nsfw: bool,
    pub category_general: bool,
    pub category_anime: bool,
    pub category_people: bool,
    pub theme_name: String,
    pub cursor_style: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            wallhaven_api_key: None,
            download_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
            default_provider: Provider::Wallhaven,
            purity_sfw: true,
            purity_sketchy: false,
            purity_nsfw: false,
            category_general: true,
            category_anime: true,
            category_people: false,
            theme_name: "dark".to_string(),
            cursor_style: "block".to_string(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("walltui").join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        tracing::warn!("Failed to parse config: {e}");
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to read config: {e}");
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
