use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_image::thread::ThreadProtocol;

use crate::core::models::Wallpaper;
use crate::infrastructure::config_loader::AppConfig;
use crate::ui::theme::{StyleKey, Theme};
use crate::ui::widgets::image_list::ImageList;
use crate::ui::widgets::search_bar::SearchBar;

pub struct SearchScreen<'a> {
    query: &'a str,
    cursor_pos: usize,
    search_focused: bool,
    cursor_visible: bool,
    cursor_style: &'a str,
    selected: usize,
    wallpapers: &'a [Wallpaper],
    thumbnail_image: &'a mut Option<ThreadProtocol>,
    theme: &'a Theme,
    config: &'a AppConfig,
    fullscreen: bool,
}

impl<'a> SearchScreen<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        query: &'a str,
        cursor_pos: usize,
        search_focused: bool,
        cursor_visible: bool,
        cursor_style: &'a str,
        selected: usize,
        wallpapers: &'a [Wallpaper],
        thumbnail_image: &'a mut Option<ThreadProtocol>,
        theme: &'a Theme,
        config: &'a AppConfig,
    ) -> Self {
        Self {
            query,
            cursor_pos,
            search_focused,
            cursor_visible,
            cursor_style,
            selected,
            wallpapers,
            thumbnail_image,
            theme,
            config,
            fullscreen: false,
        }
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn render(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(area);

        SearchBar::new(
            self.query,
            self.cursor_pos,
            self.search_focused,
            self.cursor_visible,
            self.cursor_style,
            &self.config.purity_label(),
            &self.config.category_label(),
            "Top",
            self.theme,
        )
        .render(frame, chunks[0]);

        if self.wallpapers.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No results. Press / to search.",
                self.theme.resolve(StyleKey::Secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(empty, chunks[1]);
        } else {
            ImageList::new(
                self.wallpapers,
                self.selected,
                self.thumbnail_image,
                self.theme,
            )
            .fullscreen(self.fullscreen)
            .render(frame, chunks[1]);
        }

        self.render_footer(frame, chunks[2]);
    }

    fn render_footer(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let items = [
            ("/", "Edit Search"),
            ("↑/↓", "Navigate"),
            ("Enter", "Apply"),
            ("d", "Save"),
            ("f", "Fullscreen"),
            ("Esc", "Back"),
        ];
        let spans: Vec<Span> = items
            .iter()
            .enumerate()
            .flat_map(|(i, (key, desc))| {
                let mut parts = vec![
                    Span::styled(
                        format!("[ {key} ]"),
                        self.theme
                            .resolve(StyleKey::Secondary)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    ),
                    Span::raw(format!(" {desc}")),
                ];
                if i < items.len() - 1 {
                    parts.push(Span::raw(" "));
                }
                parts
            })
            .collect();

        let footer = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(self.theme.resolve(StyleKey::Border)),
            );
        frame.render_widget(footer, area);
    }
}
