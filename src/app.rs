use std::path::PathBuf;
use std::sync::Arc;

use image::GenericImageView;
use ratatui::Frame;
use tokio::sync::mpsc;

use crate::core::download::{DownloadEvent, DownloadManager, DownloadTask, generate_filename};
use crate::core::models::{Provider, SearchQuery, Wallpaper};
use crate::infrastructure::config_loader::AppConfig;
use crate::providers::create_provider;
use crate::ui::app_layout::AppLayout;
use crate::ui::screens::Screen;
use crate::ui::screens::config::ConfigScreen;
use crate::ui::screens::detail::DetailScreen;
use crate::ui::screens::download::DownloadScreen;
use crate::ui::screens::search::SearchScreen;
use crate::ui::screens::splash::SplashScreen;
use crate::ui::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct App {
    pub should_quit: bool,
    pub current_screen: Screen,
    pub previous_screen: Option<Screen>,
    pub active_provider: Provider,
    pub theme: Theme,
    pub theme_mode: ThemeMode,
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
    pub thumbnail_path: Option<std::path::PathBuf>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        let manager = Arc::new(DownloadManager::new());
        manager.start_worker(tx);

        let config = AppConfig::load();
        let theme_mode = if config.theme == "light" {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        let theme = if config.theme == "light" {
            Theme::light()
        } else {
            Theme::dark()
        };

        Self {
            should_quit: false,
            current_screen: Screen::Splash,
            previous_screen: None,
            active_provider: config.default_provider,
            theme,
            theme_mode,
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
            thumbnail_path: None,
        }
    }

    pub async fn tick(&mut self) {
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
                    }
                }
            }
        }

        self.download_tasks = self.download_manager.tasks().await;

        if self.current_screen == Screen::Detail {
            if let Some(wallpaper) = self.wallpapers.get(self.selected_index) {
                if let Some(path) = &self.thumbnail_path {
                    if !path.to_string_lossy().contains(&wallpaper.id) {
                        self.load_thumbnail().await;
                    }
                } else {
                    self.load_thumbnail().await;
                }
            }
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
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

    pub fn toggle_theme(&mut self) {
        match self.theme_mode {
            ThemeMode::Dark => {
                self.theme_mode = ThemeMode::Light;
                self.theme = Theme::light();
            }
            ThemeMode::Light => {
                self.theme_mode = ThemeMode::Dark;
                self.theme = Theme::dark();
            }
        }
        let mut config = AppConfig::load();
        config.theme = if self.theme_mode == ThemeMode::Light {
            "light".to_string()
        } else {
            "dark".to_string()
        };
        let _ = config.save();
    }

    pub fn switch_provider(&mut self, provider: Provider) {
        self.active_provider = provider;
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
            Provider::Wallhaven => config.wallhaven_api_key,
            Provider::Pixiv => config.pixiv_api_key,
        };

        let adapter = create_provider(self.active_provider, api_key);
        let query = SearchQuery::builder(&self.search_query)
            .page(self.search_page)
            .build();

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

    pub fn enqueue_download(&mut self, wallpaper: &Wallpaper) {
        let save_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
        let filename = generate_filename(wallpaper);
        let save_path = save_dir.join(filename);
        let task = DownloadTask::new(wallpaper.clone(), save_path);
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

    pub async fn load_thumbnail(&mut self) {
        if let Some(wallpaper) = self.wallpapers.get(self.selected_index) {
            let url = &wallpaper.thumb_url;
            let client = reqwest::Client::new();

            match client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.bytes().await {
                            Ok(bytes) => {
                                let temp_dir = std::env::temp_dir();
                                let filename = format!("walltui_thumb_{}.tmp", wallpaper.id);
                                let path = temp_dir.join(&filename);

                                if let Ok(()) = tokio::fs::write(&path, &bytes).await {
                                    self.thumbnail_path = Some(path);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to read thumbnail bytes: {e}");
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to download thumbnail: {e}");
                }
            }
        }
    }

    pub fn get_thumbnail_ascii(&self, width: usize, height: usize) -> Vec<String> {
        if let Some(path) = &self.thumbnail_path {
            if let Ok(img) = image::open(path) {
                let resized = img.resize_exact(
                    width as u32,
                    height as u32,
                    image::imageops::FilterType::Nearest,
                );
                let mut lines = Vec::new();
                for y in 0..height {
                    let mut line = String::new();
                    for x in 0..width {
                        let pixel = resized.get_pixel(x as u32, y as u32);
                        let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
                        let ch = match brightness {
                            0..=63 => ' ',
                            64..=127 => '.',
                            128..=191 => ':',
                            192..=255 => '#',
                            _ => ' ',
                        };
                        line.push(ch);
                    }
                    lines.push(line);
                }
                return lines;
            }
        }
        vec![String::new(); height]
    }

    pub fn draw(&self, frame: &mut Frame) {
        let body_area = AppLayout::new(
            &self.theme,
            self.current_screen,
            &self.active_provider.to_string(),
        )
        .render(frame);

        match self.current_screen {
            Screen::Splash => {
                SplashScreen::new(&self.theme).render(frame, body_area);
            }
            Screen::Search => {
                SearchScreen::new(
                    &self.search_query,
                    self.cursor_pos,
                    self.search_focused,
                    self.selected_index,
                    &self.wallpapers,
                    self.active_provider,
                    self.search_page,
                    &self.theme,
                )
                .render(frame, body_area);
            }
            Screen::Detail => {
                if let Some(wallpaper) = self.wallpapers.get(self.selected_index) {
                    let thumb_lines = self.get_thumbnail_ascii(40, 15);
                    DetailScreen::new(wallpaper, &self.theme, &thumb_lines)
                        .render(frame, body_area);
                }
            }
            Screen::Download => {
                DownloadScreen::new(&self.download_tasks, self.selected_index, &self.theme)
                    .render(frame, body_area);
            }
            Screen::Config => {
                ConfigScreen::new(&self.theme).render(frame, body_area);
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
