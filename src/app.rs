use ratatui::Frame;

use crate::core::download::{DownloadManager, DownloadTask};
use crate::core::models::{Provider, Wallpaper};
use crate::ui::app_layout::AppLayout;
use crate::ui::screens::config::ConfigScreen;
use crate::ui::screens::detail::DetailScreen;
use crate::ui::screens::download::DownloadScreen;
use crate::ui::screens::search::SearchScreen;
use crate::ui::screens::splash::SplashScreen;
use crate::ui::screens::Screen;
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
    pub wallpapers: Vec<Wallpaper>,
    pub selected_index: usize,
    pub download_tasks: Vec<DownloadTask>,
    pub download_manager: DownloadManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: Screen::Splash,
            previous_screen: None,
            active_provider: Provider::Wallhaven,
            theme: Theme::dark(),
            theme_mode: ThemeMode::Dark,
            search_query: String::new(),
            cursor_pos: 0,
            search_focused: false,
            wallpapers: Vec::new(),
            selected_index: 0,
            download_tasks: Vec::new(),
            download_manager: DownloadManager::new(),
        }
    }

    pub fn tick(&mut self) {}

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

    pub fn draw(&self, frame: &mut Frame) {
        let body_area = AppLayout::new(&self.theme, self.current_screen, &self.active_provider.to_string()).render(frame);

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
                    &self.theme,
                )
                .render(frame, body_area);
            }
            Screen::Detail => {
                if let Some(wallpaper) = self.wallpapers.get(self.selected_index) {
                    DetailScreen::new(wallpaper, &self.theme).render(frame, body_area);
                }
            }
            Screen::Download => {
                DownloadScreen::new(&self.download_tasks, self.selected_index, &self.theme).render(frame, body_area);
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
