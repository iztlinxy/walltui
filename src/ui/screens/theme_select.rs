use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::ui::theme::{Theme, builtin_theme_names};

pub struct ThemeSelectScreen<'a> {
    themes: Vec<&'static str>,
    selected: usize,
    current_theme: &'a str,
    theme: &'a Theme,
}

impl<'a> ThemeSelectScreen<'a> {
    pub fn new(current_theme: &'a str, theme: &'a Theme, selected: usize) -> Self {
        let themes = builtin_theme_names();
        Self {
            themes,
            selected,
            current_theme,
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

        let title = Paragraph::new(Line::from(Span::styled(
            "Select Theme",
            Style::default()
                .fg(self.theme.primary)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = self.themes
            .iter()
            .map(|name| {
                let is_current = *name == self.current_theme;
                let is_selected = *name == self.themes[self.selected];
                let prefix = if is_current { "✓ " } else { "  " };
                let style = if is_selected {
                    Style::default()
                        .bg(self.theme.primary)
                        .fg(self.theme.background)
                        .add_modifier(Modifier::BOLD)
                } else if is_current {
                    Style::default().fg(self.theme.success)
                } else {
                    Style::default().fg(self.theme.foreground)
                };
                ListItem::new(Line::from(Span::styled(format!("{}{}", prefix, name), style)))
            })
            .collect();

        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Themes ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.primary)
                    .fg(self.theme.background)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, chunks[1], &mut state);

        let keys = vec![
            Span::styled("[↑/↓] ", Style::default().fg(self.theme.primary)),
            Span::styled("Navigate  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[Enter] ", Style::default().fg(self.theme.primary)),
            Span::styled("Apply  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[Esc] ", Style::default().fg(self.theme.primary)),
            Span::styled("Back", Style::default().fg(self.theme.foreground)),
        ];
        let keybar = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
        frame.render_widget(keybar, chunks[2]);
    }

    pub fn selected_theme(&self) -> &'static str {
        self.themes[self.selected]
    }
}
