use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::models::Provider;

fn walltui_dir() -> PathBuf {
    // ponytail: project is Windows-only; keep a single .config dir under home.
    dirs::home_dir()
        .map(|home| home.join(".config").join("walltui"))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn config_path() -> PathBuf {
    walltui_dir().join("config.toml")
}

pub fn data_path() -> PathBuf {
    walltui_dir().join("gallery.json")
}

pub fn themes_dir() -> PathBuf {
    walltui_dir().join("themes")
}

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
    pub fn load() -> Self {
        let path = config_path();
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
        let path = config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn purity_label(&self) -> String {
        let mut labels = Vec::new();
        if self.purity_sfw {
            labels.push("SFW");
        }
        if self.purity_sketchy {
            labels.push("Sketchy");
        }
        if self.purity_nsfw {
            labels.push("NSFW");
        }
        if labels.is_empty() {
            labels.push("None");
        }
        labels.join("+")
    }

    pub fn category_label(&self) -> String {
        let mut labels = Vec::new();
        if self.category_general {
            labels.push("General");
        }
        if self.category_anime {
            labels.push("Anime");
        }
        if self.category_people {
            labels.push("People");
        }
        if labels.is_empty() {
            labels.push("None");
        }
        labels.join("+")
    }
}
