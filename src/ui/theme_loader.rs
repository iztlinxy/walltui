use crate::infrastructure::config_loader::themes_dir;
use crate::ui::theme::{Theme, builtin_theme};

pub struct ThemeLoader;

impl ThemeLoader {
    pub fn load(name: &str) -> Theme {
        let custom_path = themes_dir().join(format!("{name}.toml"));
        if custom_path.exists()
            && let Ok(content) = std::fs::read_to_string(&custom_path)
            && let Ok(theme) = Theme::from_toml(&content)
        {
            return theme;
        }
        builtin_theme(name).unwrap_or_default()
    }
}
