use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::models::{Provider, Wallpaper};
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
    active_provider: Provider,
    theme: &'a Theme,
}

impl<'a> SearchScreen<'a> {
    pub fn new(
        query: &'a str,
        cursor_pos: usize,
        search_focused: bool,
        selected: usize,
        wallpapers: &'a [Wallpaper],
        active_provider: Provider,
        theme: &'a Theme,
    ) -> Self {
        Self {
            query,
            cursor_pos,
            search_focused,
            selected,
            wallpapers,
            active_provider,
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
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)));
            frame.render_widget(empty, chunks[1]);
        } else {
            ImageList::new(self.wallpapers, self.selected, self.theme).render(frame, chunks[1]);
        }

        let provider_label = self.active_provider.to_string();
        let provider_text = format!("Provider: {provider_label}");
        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("/", "Search"),
            ("1/2", &provider_text),
            ("↑/↓", "Navigate"),
            ("Enter", "View"),
            ("d", "Download"),
            ("Esc", "Back"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[2]);
    }
}
