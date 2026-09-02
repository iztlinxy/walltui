use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::Theme;

pub struct SearchBar<'a> {
    query: &'a str,
    cursor_pos: usize,
    focused: bool,
    cursor_visible: bool,
    cursor_style: &'a str,
    theme: &'a Theme,
}

impl<'a> SearchBar<'a> {
    pub fn new(
        query: &'a str,
        cursor_pos: usize,
        focused: bool,
        cursor_visible: bool,
        cursor_style: &'a str,
        theme: &'a Theme,
    ) -> Self {
        Self {
            query,
            cursor_pos,
            focused,
            cursor_visible,
            cursor_style,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        use crate::ui::theme::StyleKey;
        let border_style = if self.focused {
            self.theme.resolve(StyleKey::BorderFocused)
        } else {
            self.theme.resolve(StyleKey::Border)
        };

        let cursor = if self.focused && self.cursor_visible {
            match self.cursor_style {
                "line" => "▏",
                "underline" => "_",
                _ => "█",
            }
        } else {
            ""
        };
        let (before, after) = self.query.split_at(self.cursor_pos.min(self.query.len()));

        let input_line = Line::from(vec![
            Span::raw(" "),
            Span::raw(before),
            Span::styled(cursor, self.theme.resolve(StyleKey::Cursor)),
            Span::raw(after),
        ]);

        let search_bar = Paragraph::new(input_line).block(
            Block::default()
                .title(" Search ")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

        frame.render_widget(search_bar, area);
    }
}
