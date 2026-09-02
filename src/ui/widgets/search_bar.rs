use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::{StyleKey, Theme};

pub struct SearchBar<'a> {
    query: &'a str,
    cursor_pos: usize,
    focused: bool,
    cursor_visible: bool,
    cursor_style: &'a str,
    purity_label: &'a str,
    category_label: &'a str,
    sort_label: &'a str,
    theme: &'a Theme,
}

impl<'a> SearchBar<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        query: &'a str,
        cursor_pos: usize,
        focused: bool,
        cursor_visible: bool,
        cursor_style: &'a str,
        purity_label: &'a str,
        category_label: &'a str,
        sort_label: &'a str,
        theme: &'a Theme,
    ) -> Self {
        Self {
            query,
            cursor_pos,
            focused,
            cursor_visible,
            cursor_style,
            purity_label,
            category_label,
            sort_label,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
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

        let badge_style = self.theme.resolve(StyleKey::Secondary);
        let badge_spans: Vec<Span> = [
            self.purity_label,
            self.category_label,
            self.sort_label,
        ]
        .iter()
        .filter(|s| !s.is_empty())
        .flat_map(|label| {
            vec![
                Span::styled("[", badge_style),
                Span::styled(*label, badge_style.add_modifier(ratatui::style::Modifier::BOLD)),
                Span::styled("]", badge_style),
                Span::raw(" "),
            ]
        })
        .collect();

        let search_bar = Paragraph::new(input_line).block(
            Block::default()
                .title_top(Line::from(Span::styled(" Search ", self.theme.resolve(StyleKey::Title))).left_aligned())
                .title_top(Line::from(badge_spans).right_aligned())
                .borders(Borders::ALL)
                .border_style(border_style),
        );

        frame.render_widget(search_bar, area);
    }
}
