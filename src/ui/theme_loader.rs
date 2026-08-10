use std::path::PathBuf;

use crate::ui::theme::{Theme, builtin_theme};

pub struct ThemeLoader;

impl ThemeLoader {
    pub fn themes_dir() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.join("walltui").join("themes")
    }

    pub fn load(name: &str) -> Theme {
        let custom_path = Self::themes_dir().join(format!("{name}.toml"));
        if custom_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&custom_path) {
                if let Ok(theme) = Theme::from_toml(&content) {
                    return theme;
                }
            }
        }
        builtin_theme(name).unwrap_or_default()
    }

}
