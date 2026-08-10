use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use image::{DynamicImage, GenericImageView};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
};
use tokio::sync::mpsc;

use crate::core::download::{DownloadEvent, DownloadManager, DownloadTask, generate_filename};
use crate::core::models::{Provider, SearchQuery, Wallpaper};
use crate::infrastructure::config_loader::AppConfig;
use crate::infrastructure::gallery_index::GalleryIndex;
use crate::providers::create_provider;
use crate::ui::app_layout::AppLayout;
use crate::ui::screens::Screen;
use crate::ui::screens::config::{ConfigField, ConfigScreen};
use crate::ui::screens::detail::DetailScreen;
use crate::ui::screens::download::DownloadScreen;
use crate::ui::screens::gallery::GalleryScreen;
use crate::ui::screens::resolution_select::{ResolutionOption, ResolutionSelectScreen};
use crate::ui::screens::search::SearchScreen;
use crate::ui::screens::splash::SplashScreen;
use crate::ui::theme::Theme;
use crate::ui::theme_loader::ThemeLoader;
use crate::ui::widgets::dialog::ConfirmDialog;
use crate::ui::widgets::toast::ToastManager;

pub struct App {
    pub should_quit: bool,
    pub current_screen: Screen,
    pub previous_screen: Option<Screen>,
    pub active_provider: Provider,
    pub theme: Theme,
    pub search_query: String,
    pub cursor_pos: usize,
    pub search_focused: bool,
    pub search_page: u32,
    pub wallpapers: Vec<Wallpaper>,
    pub selected_index: usize,
    pub download_tasks: Vec<DownloadTask>,
    pub download_manager: Arc<DownloadManager>,
    pub download_rx: mpsc::Receiver<DownloadEvent>,
    pub notification: Option<String>,
    pub thumbnail_lines: Vec<Line<'static>>,
    pub thumbnail_rx: mpsc::Receiver<Option<DynamicImage>>,
    pub thumbnail_tx: mpsc::Sender<Option<DynamicImage>>,
    pub thumbnail_loading_id: Option<String>,
    pub download_dir: PathBuf,
    pub config: AppConfig,
    pub config_fields: Vec<ConfigField>,
    pub config_selected_index: usize,
    pub config_input: String,
    pub config_cursor: usize,
    pub config_editing: bool,
    pub gallery_wallpapers: Vec<Wallpaper>,
    pub gallery_selected_index: usize,
    pub gallery_thumbnail_lines: Vec<Line<'static>>,
    pub gallery_thumbnail_rx: mpsc::Receiver<Option<DynamicImage>>,
    pub gallery_thumbnail_tx: mpsc::Sender<Option<DynamicImage>>,
    pub gallery_thumbnail_loading_id: Option<String>,
    pub gallery_thumbnail_cache: std::collections::HashMap<String, Vec<Line<'static>>>,
    pub pending_download_wallpaper: Option<Wallpaper>,
    pub resolution_selected_index: usize,
    pub resolution_options: Vec<ResolutionOption>,
    // UI polish state
    pub cursor_visible: bool,
    pub last_blink: Instant,
    pub toast_manager: ToastManager,
    pub recent_downloads: Vec<String>,
    pub confirm_dialog: Option<ConfirmDialog>,
    // Gallery rename / metadata
    pub gallery_index: GalleryIndex,
    pub gallery_editing: bool,
    pub gallery_edit_input: String,
    pub gallery_edit_cursor: usize,
    pub gallery_cursor_visible: bool,
    pub gallery_scan_rx: mpsc::Receiver<Vec<Wallpaper>>,
    pub gallery_scan_tx: mpsc::Sender<Vec<Wallpaper>>,
    pub gallery_loading: bool,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        let (thumb_tx, thumb_rx) = mpsc::channel(10);
        let (gallery_thumb_tx, gallery_thumb_rx) = mpsc::channel(10);
        let (gallery_scan_tx, gallery_scan_rx) = mpsc::channel(1);
        let manager = Arc::new(DownloadManager::new());
        manager.start_worker(tx);

        let config = AppConfig::load();
        let theme = ThemeLoader::load(&config.theme_name);
        let download_dir = config.download_dir.clone();
        let config_input = download_dir.to_string_lossy().to_string();
        let gallery_index = GalleryIndex::load();

        Self {
            should_quit: false,
            current_screen: Screen::Splash,
            previous_screen: None,
            active_provider: config.default_provider,
            theme,
            search_query: String::new(),
            cursor_pos: 0,
            search_focused: false,
            search_page: 1,
            wallpapers: Vec::new(),
            selected_index: 0,
            download_tasks: Vec::new(),
            download_manager: manager,
            download_rx: rx,
            notification: None,
            thumbnail_lines: Vec::new(),
            thumbnail_rx: thumb_rx,
            thumbnail_tx: thumb_tx,
            thumbnail_loading_id: None,
            download_dir,
            config: config.clone(),
            config_fields: ConfigField::all(),
            config_selected_index: 0,
            config_input,
            config_cursor: 0,
            config_editing: false,
            gallery_wallpapers: Vec::new(),
            gallery_selected_index: 0,
            gallery_thumbnail_lines: Vec::new(),
            gallery_thumbnail_rx: gallery_thumb_rx,
            gallery_thumbnail_tx: gallery_thumb_tx,
            gallery_thumbnail_loading_id: None,
            gallery_thumbnail_cache: std::collections::HashMap::new(),
            pending_download_wallpaper: None,
            resolution_selected_index: 0,
            resolution_options: ResolutionOption::presets(),
            cursor_visible: true,
            last_blink: Instant::now(),
            toast_manager: ToastManager::default(),
            recent_downloads: Vec::new(),
            confirm_dialog: None,
            gallery_index,
            gallery_editing: false,
            gallery_edit_input: String::new(),
            gallery_edit_cursor: 0,
            gallery_cursor_visible: true,
            gallery_scan_rx,
            gallery_scan_tx,
            gallery_loading: false,
        }
    }

    pub async fn tick(&mut self) {
        // Cursor blink tick (530ms default).
        if self.last_blink.elapsed() >= self.theme.cursor_blink_interval() {
            self.cursor_visible = !self.cursor_visible;
            self.gallery_cursor_visible = !self.gallery_cursor_visible;
            self.last_blink = Instant::now();
        }

        // Toast auto-dismiss.
        self.toast_manager.tick();

        while let Ok(event) = self.download_rx.try_recv() {
            match event {
                DownloadEvent::Progress(idx, progress) => {
                    if let Some(task) = self.download_tasks.get_mut(idx) {
                        task.progress = progress;
                    }
                }
                DownloadEvent::Completed(idx) => {
                    if let Some(task) = self.download_tasks.get(idx) {
                        let title = task.wallpaper.title.clone();
                        self.notification = Some(format!("Downloaded: {title}"));
                        self.toast_manager.show(format!("Downloaded: {title}"));
                        self.add_recent_download(&title);
                    }
                }
            }
        }

        self.download_tasks = self.download_manager.tasks().await;

        while let Ok(img) = self.thumbnail_rx.try_recv() {
            if let Some(img) = img {
                self.thumbnail_lines = image_to_lines(&img, 42, 18);
            } else {
                self.thumbnail_lines = Vec::new();
            }
        }

        while let Ok(img) = self.gallery_thumbnail_rx.try_recv() {
            if let Some(img) = img {
                let lines = image_to_lines(&img, 60, 25);
                if let Some(w) = self.gallery_wallpapers.get(self.gallery_selected_index) {
                    self.gallery_thumbnail_cache.insert(w.id.clone(), lines.clone());
                }
                self.gallery_thumbnail_lines = lines;
            } else {
                self.gallery_thumbnail_lines = Vec::new();
            }
        }

        while let Ok(wallpapers) = self.gallery_scan_rx.try_recv() {
            self.gallery_wallpapers = wallpapers;
            self.gallery_loading = false;
        }

        if (self.current_screen == Screen::Detail || self.current_screen == Screen::Search)
            && let Some(wallpaper) = self.wallpapers.get(self.selected_index)
            && self.thumbnail_loading_id.as_deref() != Some(&wallpaper.id)
        {
            self.thumbnail_loading_id = Some(wallpaper.id.clone());
            self.thumbnail_lines = Vec::new();
            let tx = self.thumbnail_tx.clone();
            let wallpaper = wallpaper.clone();
            tokio::spawn(async move {
                load_thumbnail_async(wallpaper, tx).await;
            });
        }

        if self.current_screen == Screen::Gallery
            && let Some(wallpaper) = self.gallery_wallpapers.get(self.gallery_selected_index)
            && self.gallery_thumbnail_loading_id.as_deref() != Some(&wallpaper.id)
        {
            if let Some(cached) = self.gallery_thumbnail_cache.get(&wallpaper.id) {
                self.gallery_thumbnail_lines = cached.clone();
                self.gallery_thumbnail_loading_id = Some(wallpaper.id.clone());
            } else {
                self.gallery_thumbnail_loading_id = Some(wallpaper.id.clone());
                self.gallery_thumbnail_lines = Vec::new();
                let tx = self.gallery_thumbnail_tx.clone();
                let path = PathBuf::from(&wallpaper.url);
                tokio::spawn(async move {
                    load_local_thumbnail_async(path, tx).await;
                });
            }
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn add_recent_download(&mut self, title: &str) {
        self.recent_downloads.retain(|t| t != title);
        self.recent_downloads.insert(0, title.to_string());
        if self.recent_downloads.len() > 5 {
            self.recent_downloads.pop();
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.previous_screen = Some(self.current_screen);
        self.current_screen = screen;
    }

    pub fn go_back(&mut self) {
        if let Some(prev) = self.previous_screen {
            self.current_screen = prev;
            self.previous_screen = None;
        } else {
            self.current_screen = Screen::Splash;
        }
    }

    pub fn switch_provider(&mut self, provider: Provider) {
        self.active_provider = provider;
    }

    pub fn cycle_theme(&mut self) {
        let next = crate::ui::theme::cycle_theme_name(&self.config.theme_name);
        self.config.theme_name = next.to_string();
        self.theme = ThemeLoader::load(next);
        let _ = self.config.save();
        self.toast_manager.show(format!("Theme: {next}"));
    }

    pub fn handle_search_input(&mut self, c: char) {
        self.search_query.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn handle_search_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.search_query.remove(self.cursor_pos);
        }
    }

    pub fn handle_search_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn handle_search_right(&mut self) {
        if self.cursor_pos < self.search_query.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.cursor_pos = 0;
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index + 1 < self.wallpapers.len() {
            self.selected_index += 1;
        }
    }

    pub async fn execute_search(&mut self) {
        if self.search_query.is_empty() {
            return;
        }
        if let Err(e) = self.execute_search_inner().await {
            tracing::error!("Search failed: {e}");
        }
    }

    async fn execute_search_inner(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig::load();
        let api_key = match self.active_provider {
            Provider::Wallhaven => config.wallhaven_api_key.clone(),
        };

        let purity = build_purity_string(&config);
        let categories = build_category_string(&config);

        if config.purity_nsfw && api_key.is_none() {
            self.notification = Some("NSFW requires a Wallhaven API key. Set it in Settings.".to_string());
        }

        let adapter = create_provider(self.active_provider, api_key);
        let mut builder = SearchQuery::builder(&self.search_query)
            .page(self.search_page);
        if !purity.is_empty() {
            builder = builder.purity(purity);
        }
        if !categories.is_empty() {
            builder = builder.categories(categories);
        }
        let query = builder.build();

        let results = adapter.search(&query).await.map_err(|e| {
            let msg = e.to_string();
            self.wallpapers.clear();
            self.selected_index = 0;
            msg
        })?;

        self.wallpapers = results;
        self.selected_index = 0;
        Ok(())
    }

    pub async fn search_next_page(&mut self) {
        if self.search_query.is_empty() {
            return;
        }
        self.search_page += 1;
        if let Err(e) = self.execute_search_inner().await {
            tracing::error!("Next page failed: {e}");
        }
    }

    pub async fn search_prev_page(&mut self) {
        if self.search_query.is_empty() || self.search_page <= 1 {
            return;
        }
        self.search_page -= 1;
        if let Err(e) = self.execute_search_inner().await {
            tracing::error!("Prev page failed: {e}");
        }
    }

    pub fn reset_search_page(&mut self) {
        self.search_page = 1;
    }

    pub fn next_page(&mut self) {
        self.search_page += 1;
    }

    pub fn prev_page(&mut self) {
        if self.search_page > 1 {
            self.search_page -= 1;
        }
    }

    pub fn enqueue_download(&mut self, wallpaper: &Wallpaper, resolution: Option<ResolutionOption>) {
        let save_dir = self.download_dir.clone();
        let _ = std::fs::create_dir_all(&save_dir);
        let filename = generate_filename(wallpaper);
        let save_path = save_dir.join(filename);
        let mut task = DownloadTask::new(wallpaper.clone(), save_path);
        if let Some(res) = resolution {
            task.resolution = Some(res);
        }
        self.download_tasks.push(task.clone());
        tokio::spawn({
            let manager = Arc::clone(&self.download_manager);
            async move {
                manager.enqueue(task).await;
            }
        });
    }

    pub async fn cancel_download(&mut self, index: usize) {
        self.download_manager.cancel(index).await;
    }

    pub async fn retry_download(&mut self, index: usize) {
        self.download_manager.retry(index).await;
    }

    pub async fn clear_completed_downloads(&mut self) {
        self.download_manager.remove_completed().await;
    }
    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    pub fn load_gallery(&mut self) {
        self.gallery_loading = true;
        self.gallery_wallpapers.clear();
        self.gallery_selected_index = 0;
        self.gallery_thumbnail_lines = Vec::new();
        self.gallery_thumbnail_loading_id = None;
        let dir = self.download_dir.clone();
        let tx = self.gallery_scan_tx.clone();
        let index = self.gallery_index.clone();
        tokio::spawn(async move {
            let wallpapers = tokio::task::spawn_blocking(move || scan_local_wallpapers(&dir))
                .await
                .unwrap_or_default();
            let wallpapers: Vec<Wallpaper> = wallpapers
                .into_iter()
                .map(|mut wp| {
                    wp.title = index.display_name(&wp);
                    wp
                })
                .collect();
            let _ = tx.send(wallpapers).await;
        });
    }

    pub fn move_gallery_selection_up(&mut self) {
        if self.gallery_selected_index > 0 {
            self.gallery_selected_index -= 1;
        }
    }

    pub fn move_gallery_selection_down(&mut self) {
        if self.gallery_selected_index + 1 < self.gallery_wallpapers.len() {
            self.gallery_selected_index += 1;
        }
    }

    pub fn start_gallery_rename(&mut self) {
        if let Some(wp) = self.gallery_wallpapers.get(self.gallery_selected_index) {
            self.gallery_editing = true;
            self.gallery_edit_input = self.gallery_index.display_name(wp).to_string();
            self.gallery_edit_cursor = self.gallery_edit_input.len();
        }
    }

    pub fn cancel_gallery_rename(&mut self) {
        self.gallery_editing = false;
        self.gallery_edit_input.clear();
        self.gallery_edit_cursor = 0;
    }

    pub fn handle_gallery_edit_input(&mut self, c: char) {
        self.gallery_edit_input.insert(self.gallery_edit_cursor, c);
        self.gallery_edit_cursor += 1;
    }

    pub fn handle_gallery_edit_backspace(&mut self) {
        if self.gallery_edit_cursor > 0 {
            self.gallery_edit_cursor -= 1;
            self.gallery_edit_input.remove(self.gallery_edit_cursor);
        }
    }

    pub fn handle_gallery_edit_left(&mut self) {
        if self.gallery_edit_cursor > 0 {
            self.gallery_edit_cursor -= 1;
        }
    }

    pub fn handle_gallery_edit_right(&mut self) {
        if self.gallery_edit_cursor < self.gallery_edit_input.len() {
            self.gallery_edit_cursor += 1;
        }
    }

    pub fn prompt_delete_gallery_selected(&mut self) {
        if let Some(wp) = self.gallery_wallpapers.get(self.gallery_selected_index) {
            self.confirm_dialog = Some(crate::ui::widgets::dialog::ConfirmDialog::new(
                format!("Delete '{}' ?", wp.title),
                crate::ui::widgets::dialog::ConfirmAction::DeleteGalleryItem,
            ));
        }
    }

    pub fn confirm_gallery_delete(&mut self) {
        if self.gallery_selected_index >= self.gallery_wallpapers.len() {
            return;
        }
        let wp = self.gallery_wallpapers[self.gallery_selected_index].clone();
        let path = PathBuf::from(&wp.url);
        if let Err(e) = std::fs::remove_file(&path) {
            self.toast_manager.show(format!("Delete failed: {e}"));
        } else {
            self.gallery_index.entries.remove(&wp.id);
            let _ = self.gallery_index.save();
            self.toast_manager.show(format!("Deleted: {}", wp.title));
        }
        self.confirm_dialog = None;
        self.load_gallery();
    }

    pub fn confirm_gallery_rename(&mut self) {
        let Some(wp) = self.gallery_wallpapers.get(self.gallery_selected_index).cloned() else {
            return;
        };
        let new_name = self.gallery_edit_input.trim();
        if new_name.is_empty() {
            self.toast_manager.show("Rename cancelled: empty name");
            self.cancel_gallery_rename();
            return;
        }
        let old_path = PathBuf::from(&wp.url);
        let Some(ext) = old_path.extension().and_then(|e| e.to_str()) else {
            self.toast_manager.show("Rename failed: no extension");
            self.cancel_gallery_rename();
            return;
        };
        let new_filename = format!("{new_name}.{ext}");
        let new_path = old_path.with_file_name(&new_filename);
        if new_path.exists() {
            self.toast_manager.show("Rename failed: name already exists");
            self.cancel_gallery_rename();
            return;
        }
        if let Err(e) = std::fs::rename(&old_path, &new_path) {
            self.toast_manager.show(format!("Rename failed: {e}"));
            self.cancel_gallery_rename();
            return;
        }
        if let Err(e) = self.gallery_index.rename(&wp.id, &new_filename, new_path) {
            self.toast_manager.show(format!("Index update failed: {e}"));
        } else {
            self.toast_manager.show(format!("Renamed to '{new_filename}'"));
        }
        self.cancel_gallery_rename();
        self.load_gallery();
    }

    pub fn start_config_edit(&mut self) {
        let field = self.config_fields[self.config_selected_index];
        match field {
            ConfigField::ApiKey => {
                self.config_editing = true;
                self.config_input = self.config.wallhaven_api_key.clone().unwrap_or_default();
                self.config_cursor = self.config_input.len();
            }
            ConfigField::DownloadDir => {
                self.config_editing = true;
                self.config_input = self.download_dir.to_string_lossy().to_string();
                self.config_cursor = self.config_input.len();
            }
            ConfigField::ThemeName => {
                self.config_editing = true;
                self.config_input = self.config.theme_name.clone();
                self.config_cursor = self.config_input.len();
            }
            ConfigField::CursorStyle => {
                self.config_editing = true;
                self.config_input = self.config.cursor_style.clone();
                self.config_cursor = self.config_input.len();
            }
            _ => {
                self.toggle_config_field();
            }
        }
    }

    pub fn toggle_config_field(&mut self) {
        let field = self.config_fields[self.config_selected_index];
        match field {
            ConfigField::PuritySfw => self.config.purity_sfw = !self.config.purity_sfw,
            ConfigField::PuritySketchy => self.config.purity_sketchy = !self.config.purity_sketchy,
            ConfigField::PurityNsfw => self.config.purity_nsfw = !self.config.purity_nsfw,
            ConfigField::CategoryGeneral => self.config.category_general = !self.config.category_general,
            ConfigField::CategoryAnime => self.config.category_anime = !self.config.category_anime,
            ConfigField::CategoryPeople => self.config.category_people = !self.config.category_people,
            _ => return,
        }
        self.save_config();
    }

    pub fn move_config_selection_up(&mut self) {
        if self.config_selected_index > 0 {
            self.config_selected_index -= 1;
        }
    }

    pub fn move_config_selection_down(&mut self) {
        if self.config_selected_index + 1 < self.config_fields.len() {
            self.config_selected_index += 1;
        }
    }

    pub fn handle_config_input(&mut self, c: char) {
        self.config_input.insert(self.config_cursor, c);
        self.config_cursor += 1;
    }

    pub fn handle_config_backspace(&mut self) {
        if self.config_cursor > 0 {
            self.config_cursor -= 1;
            self.config_input.remove(self.config_cursor);
        }
    }

    pub fn handle_config_left(&mut self) {
        if self.config_cursor > 0 {
            self.config_cursor -= 1;
        }
    }

    pub fn handle_config_right(&mut self) {
        if self.config_cursor < self.config_input.len() {
            self.config_cursor += 1;
        }
    }

    pub fn cancel_config_edit(&mut self) {
        self.config_editing = false;
    }

    pub fn confirm_config_edit(&mut self) {
        let field = self.config_fields[self.config_selected_index];
        match field {
            ConfigField::ApiKey => {
                let val = if self.config_input.is_empty() { None } else { Some(self.config_input.clone()) };
                self.config.wallhaven_api_key = val.clone();
                self.save_config();
                self.notification = match val {
                    Some(_) => Some("API key saved".to_string()),
                    None => Some("API key removed".to_string()),
                };
            }
            ConfigField::DownloadDir => {
                let expanded = expand_tilde(&self.config_input);
                if expanded.as_os_str().is_empty() {
                    self.notification = Some("Download directory cannot be empty".to_string());
                    return;
                }
                if let Err(e) = std::fs::create_dir_all(&expanded) {
                    self.notification = Some(format!("Failed to create directory: {e}"));
                    return;
                }
                self.download_dir = expanded.clone();
                self.config.download_dir = expanded;
                self.save_config();
                self.notification = Some(format!("Download dir saved: {}", self.download_dir.display()));
            }
            ConfigField::ThemeName => {
                let name = self.config_input.trim();
                if name.is_empty() {
                    self.toast_manager.show("Theme name cannot be empty");
                    return;
                }
                let new_theme = crate::ui::theme_loader::ThemeLoader::load(name);
                self.config.theme_name = name.to_string();
                self.theme = new_theme;
                self.save_config();
                self.toast_manager.show(format!("Theme: {name}"));
            }
            ConfigField::CursorStyle => {
                let style = self.config_input.trim();
                if matches!(style, "block" | "line" | "underline") {
                    self.config.cursor_style = style.to_string();
                    self.save_config();
                    self.toast_manager.show(format!("Cursor style: {style}"));
                } else {
                    self.toast_manager.show("Cursor style must be block, line or underline");
                    return;
                }
            }
            _ => {}
        }
        self.config_editing = false;
    }

    fn save_config(&self) {
        let _ = self.config.save();
    }

    pub fn draw(&self, frame: &mut Frame) {
        let toast = self
            .toast_manager
            .message()
            .or(self.notification.as_deref());
        let body_area = AppLayout::new(
            &self.theme,
            self.current_screen,
            &self.active_provider.to_string(),
            toast,
        )
        .render(frame);

        match self.current_screen {
            Screen::Splash => {
                SplashScreen::new(&self.theme, &self.recent_downloads)
                    .render(frame, body_area);
            }
            Screen::Search => {
                SearchScreen::new(
                    &self.search_query,
                    self.cursor_pos,
                    self.search_focused,
                    self.cursor_visible,
                    &self.config.cursor_style,
                    self.selected_index,
                    &self.wallpapers,
                    self.search_page,
                    &self.thumbnail_lines,
                    &self.theme,
                )
                .render(frame, body_area);
            }
            Screen::Detail => {
                if let Some(wallpaper) = self.wallpapers.get(self.selected_index) {
                    DetailScreen::new(wallpaper, &self.theme, &self.thumbnail_lines)
                        .render(frame, body_area);
                }
            }
            Screen::Download => {
                DownloadScreen::new(&self.download_tasks, self.selected_index, &self.theme)
                    .render(frame, body_area);
            }
            Screen::Config => {
                ConfigScreen::new(
                    &self.theme,
                    &self.config,
                    self.config_selected_index,
                    self.config_fields[self.config_selected_index],
                    self.config_editing,
                    &self.config_input,
                    self.config_cursor,
                    self.cursor_visible,
                    &self.config.cursor_style,
                )
                .render(frame, body_area);
            }
            Screen::Gallery => {
                GalleryScreen::new(
                    &self.gallery_wallpapers,
                    self.gallery_selected_index,
                    &self.gallery_thumbnail_lines,
                    &self.theme,
                    &self.download_dir,
                    self.gallery_editing,
                    &self.gallery_edit_input,
                    self.gallery_edit_cursor,
                    self.gallery_cursor_visible,
                    &self.config.cursor_style,
                    &self.confirm_dialog,
                    self.gallery_loading,
                )
                .render(frame, body_area);
            }
            Screen::ResolutionSelect => {
                if let Some(wallpaper) = &self.pending_download_wallpaper {
                    let dims = match (wallpaper.width, wallpaper.height) {
                        (Some(w), Some(h)) => Some((w, h)),
                        _ => None,
                    };
                    ResolutionSelectScreen::new(
                        &self.resolution_options,
                        self.resolution_selected_index,
                        &wallpaper.title,
                        dims,
                        &self.theme,
                    )
                    .render(frame, body_area);
                }
            }
        }
    }
}

fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from(path));
    }
    if let Some(rest) = path.strip_prefix("~/") {
        return dirs::home_dir()
            .map(|home| home.join(rest))
            .unwrap_or_else(|| PathBuf::from(path));
    }
    PathBuf::from(path)
}

fn is_image_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|ext| matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp"))
        .unwrap_or(false)
}

fn scan_local_wallpapers(dir: &std::path::Path) -> Vec<Wallpaper> {
    let mut entries: Vec<Wallpaper> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() || !is_image_file(&path) {
                return None;
            }
            let filename = path.file_name()?.to_string_lossy().to_string();
            let path_str = path.to_string_lossy().to_string();
            let (width, height) = image::image_dimensions(&path).unwrap_or((0, 0));
            Some(Wallpaper {
                id: filename.clone(),
                provider: crate::core::models::Provider::Wallhaven,
                url: path_str.clone(),
                thumb_url: path_str,
                title: filename,
                photographer: String::new(),
                width: if width > 0 { Some(width) } else { None },
                height: if height > 0 { Some(height) } else { None },
                avg_color: None,
                attribution: None,
                file_type: None,
                web_url: None,
                tags: Vec::new(),
                category: None,
                purity: None,
                views: None,
                favorites: None,
            })
        })
        .collect();
    entries.sort_by(|a, b| a.title.cmp(&b.title));
    entries
}

async fn load_local_thumbnail_async(path: PathBuf, tx: mpsc::Sender<Option<DynamicImage>>) {
    let img = tokio::task::spawn_blocking(move || image::open(&path).ok())
        .await
        .ok()
        .flatten();
    let _ = tx.send(img).await;
}

async fn load_thumbnail_async(wallpaper: Wallpaper, tx: mpsc::Sender<Option<DynamicImage>>) {
    let client = reqwest::Client::new();
    let img = match client.get(&wallpaper.thumb_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => image::load_from_memory(&bytes).ok(),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    };

    let _ = tx.send(img).await;
}

fn image_to_lines(img: &DynamicImage, width: u32, height: u32) -> Vec<Line<'static>> {
    let resized = img.resize_exact(width, height * 2, image::imageops::FilterType::Lanczos3);
    let mut lines = Vec::with_capacity(height as usize);

    for y in 0..height {
        let mut spans = Vec::with_capacity(width as usize);
        for x in 0..width {
            let top = resized.get_pixel(x, y * 2);
            let bottom = resized.get_pixel(x, y * 2 + 1);
            let fg = Color::Rgb(top[0], top[1], top[2]);
            let bg = Color::Rgb(bottom[0], bottom[1], bottom[2]);
            spans.push(Span::styled("▀", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
    }

    lines
}

fn build_purity_string(config: &AppConfig) -> String {
    let s = if config.purity_sfw { "1" } else { "0" };
    let k = if config.purity_sketchy { "1" } else { "0" };
    let n = if config.purity_nsfw { "1" } else { "0" };
    format!("{s}{k}{n}")
}

fn build_category_string(config: &AppConfig) -> String {
    let g = if config.category_general { "1" } else { "0" };
    let a = if config.category_anime { "1" } else { "0" };
    let p = if config.category_people { "1" } else { "0" };
    format!("{g}{a}{p}")
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
