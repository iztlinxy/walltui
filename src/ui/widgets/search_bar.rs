use ratatui::{
    layout::Rect,
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::theme::Theme;

pub struct SearchBar<'a> {
    query: &'a str,
    cursor_pos: usize,
    focused: bool,
    theme: &'a Theme,
}

impl<'a> SearchBar<'a> {
    pub fn new(query: &'a str, cursor_pos: usize, focused: bool, theme: &'a Theme) -> Self {
        Self {
            query,
            cursor_pos,
            focused,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_style = if self.focused {
            Style::default().fg(self.theme.primary)
        } else {
            Style::default().fg(self.theme.secondary)
        };

        let cursor = if self.focused { "▌" } else { "" };
        let (before, after) = self.query.split_at(self.cursor_pos.min(self.query.len()));

        let input_line = Line::from(vec![
            Span::raw(before),
            Span::styled(cursor, Style::default().fg(self.theme.foreground)),
            Span::raw(after),
        ]);

        let search_bar = Paragraph::new(input_line)
            .block(Block::default().title(" Search ").borders(Borders::ALL).border_style(border_style));

        frame.render_widget(search_bar, area);
    }
}
