use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;
use crate::ui::widgets::image_list::ImageList;
use crate::ui::widgets::search_bar::SearchBar;

pub struct SearchScreen<'a> {
    query: &'a str,
    cursor_pos: usize,
    search_focused: bool,
    selected: usize,
    wallpapers: &'a [Wallpaper],
    search_page: u32,
    thumbnail_lines: &'a [Line<'static>],
    theme: &'a Theme,
}

impl<'a> SearchScreen<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        query: &'a str,
        cursor_pos: usize,
        search_focused: bool,
        selected: usize,
        wallpapers: &'a [Wallpaper],
        search_page: u32,
        thumbnail_lines: &'a [Line<'static>],
        theme: &'a Theme,
    ) -> Self {
        Self {
            query,
            cursor_pos,
            search_focused,
            selected,
            wallpapers,
            search_page,
            thumbnail_lines,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        SearchBar::new(self.query, self.cursor_pos, self.search_focused, self.theme)
            .render(frame, chunks[0]);

        if self.wallpapers.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No results. Press / to search.",
                Style::default().fg(self.theme.secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
            frame.render_widget(empty, chunks[1]);
        } else {
            ImageList::new(
                self.wallpapers,
                self.selected,
                self.thumbnail_lines,
                self.theme,
            )
            .render(frame, chunks[1]);
        }

        let page_text = format!("Page: {}", self.search_page);
        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("/", "Search"),
            ("↑/↓", "Navigate"),
            ("n/p", &page_text),
            ("Enter", "View"),
            ("d", "Download"),
            ("Esc", "Back"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[2]);
    }
}
