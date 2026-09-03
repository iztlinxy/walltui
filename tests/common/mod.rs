use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use ratatui_image::picker::Picker;
use tokio::sync::mpsc;

use walltui::app::App;
use walltui::core::download::DownloadManager;
use walltui::core::models::{Provider, Wallpaper};
use walltui::infrastructure::config_loader::AppConfig;
use walltui::infrastructure::gallery_index::GalleryIndex;
use walltui::ui::screens::Screen;
use walltui::ui::screens::config::ConfigField;
use walltui::ui::screens::resolution_select::ResolutionOption;
use walltui::ui::widgets::toast::ToastManager;

#[allow(dead_code)]
pub fn make_app() -> App {
    let (_tx, rx) = mpsc::channel(100);
    let (thumb_tx, thumb_rx) = mpsc::channel(10);
    let (gallery_tx, gallery_rx) = mpsc::channel(10);
    let (gallery_scan_tx, gallery_scan_rx) = mpsc::channel(1);
    let manager = Arc::new(DownloadManager::new());

    let config = AppConfig {
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
    };

    App {
        should_quit: false,
        current_screen: Screen::Splash,
        previous_screen: None,
        active_provider: Provider::Wallhaven,
        theme: walltui::ui::theme::Theme::default(),
        search_query: String::new(),
        cursor_pos: 0,
        search_focused: false,
        search_page: 1,
        search_total_pages: 1,
        wallpapers: Vec::new(),
        selected_index: 0,
        search_fullscreen: false,
        download_tasks: Vec::new(),
        download_manager: manager,
        download_rx: rx,
        picker: Picker::halfblocks(),
        thumbnail_image: None,
        thumbnail_response_rx: None,
        thumbnail_rx: thumb_rx,
        thumbnail_tx: thumb_tx,
        thumbnail_loading_id: None,
        download_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
        config: config.clone(),
        config_fields: ConfigField::all(),
        config_selected_index: 0,
        config_input: String::new(),
        config_cursor: 0,
        config_editing: false,
        gallery_wallpapers: Vec::new(),
        gallery_selected_index: 0,
        gallery_thumbnail_image: None,
        gallery_thumbnail_response_rx: None,
        gallery_thumbnail_rx: gallery_rx,
        gallery_thumbnail_tx: gallery_tx,
        gallery_thumbnail_loading_id: None,
        pending_download_wallpaper: None,
        resolution_selected_index: 0,
        resolution_options: ResolutionOption::presets(),
        cursor_visible: true,
        last_blink: Instant::now(),
        toast_manager: ToastManager::default(),
        recent_downloads: Vec::new(),
        confirm_dialog: None,
        gallery_index: GalleryIndex::default(),
        gallery_editing: false,
        gallery_edit_input: String::new(),
        gallery_edit_cursor: 0,
        gallery_cursor_visible: true,
        gallery_scan_rx,
        gallery_scan_tx,
        gallery_loading: false,
        splash_selected_index: 0,
    }
}

pub fn sample_wallpaper(id: &str) -> Wallpaper {
    Wallpaper {
        id: id.to_string(),
        provider: Provider::Wallhaven,
        url: format!("https://example.com/{id}.jpg"),
        thumb_url: format!("https://example.com/thumb_{id}.jpg"),
        title: format!("Wallpaper {id}"),
        photographer: "Unknown".to_string(),
        width: Some(1920),
        height: Some(1080),
        ratio: Some("16x9".to_string()),
        file_size: Some(1_800_000),
        avg_color: None,
        colors: Vec::new(),
        attribution: None,
        file_type: Some("image/jpeg".to_string()),
        web_url: None,
        tags: Vec::new(),
        category: None,
        purity: None,
        views: None,
        favorites: None,
    }
}
