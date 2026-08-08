use std::path::PathBuf;
use std::sync::Arc;

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
use crate::providers::create_provider;
use crate::ui::app_layout::AppLayout;
use crate::ui::screens::Screen;
use crate::ui::screens::config::ConfigScreen;
use crate::ui::screens::detail::DetailScreen;
use crate::ui::screens::download::DownloadScreen;
use crate::ui::screens::search::SearchScreen;
use crate::ui::screens::splash::SplashScreen;
use crate::ui::theme::Theme;

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
    pub config_input: String,
    pub config_cursor: usize,
    pub config_editing: bool,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        let (thumb_tx, thumb_rx) = mpsc::channel(10);
        let manager = Arc::new(DownloadManager::new());
        manager.start_worker(tx);

        let config = AppConfig::load();
        let theme = Theme::default();
        let download_dir = config.download_dir.clone();
        let config_input = download_dir.to_string_lossy().to_string();

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
            config_input,
            config_cursor: 0,
            config_editing: false,
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

        while let Ok(img) = self.thumbnail_rx.try_recv() {
            if let Some(img) = img {
                self.thumbnail_lines = image_to_lines(&img, 42, 18);
            } else {
                self.thumbnail_lines = Vec::new();
            }
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
        let save_dir = self.download_dir.clone();
        let _ = std::fs::create_dir_all(&save_dir);
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

    pub fn start_config_edit(&mut self) {
        self.config_editing = true;
        self.config_input = self.download_dir.to_string_lossy().to_string();
        self.config_cursor = self.config_input.len();
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
        self.config_input = self.download_dir.to_string_lossy().to_string();
        self.config_cursor = 0;
    }

    pub fn confirm_config_download_dir(&mut self) {
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
        self.config_editing = false;
        self.config_cursor = 0;
        let mut config = AppConfig::load();
        config.download_dir = expanded;
        match config.save() {
            Ok(_) => self.notification = Some(format!(
                "Download dir saved: {}",
                self.download_dir.display()
            )),
            Err(e) => self.notification = Some(format!("Failed to save config: {e}")),
        }
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
                    &self.download_dir.to_string_lossy(),
                    &self.config_input,
                    self.config_editing,
                    self.config_cursor,
                )
                .render(frame, body_area);
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
