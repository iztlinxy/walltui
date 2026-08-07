use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;

pub struct ConfigScreen<'a> {
    theme: &'a Theme,
}

impl<'a> ConfigScreen<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let config_lines = vec![
            Line::from(Span::styled(" Configuration", Style::default().fg(self.theme.primary).bold())),
            Line::from(""),
            Line::from(Span::styled(" API Keys:", Style::default().fg(self.theme.secondary).bold())),
            Line::from("   Wallhaven: (not set)"),
            Line::from("   Pixiv: (not set)"),
            Line::from(""),
            Line::from(Span::styled(" Download Directory:", Style::default().fg(self.theme.secondary).bold())),
            Line::from("   ~/Downloads/walltui"),
            Line::from(""),
            Line::from(Span::styled(" Theme:", Style::default().fg(self.theme.secondary).bold())),
            Line::from("   Dark (press 't' to toggle)"),
        ];

        let config = Paragraph::new(config_lines)
            .block(Block::default().title(" Settings ").borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)));

        frame.render_widget(config, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("t", "Toggle theme"),
            ("Ctrl+S", "Save"),
            ("Esc", "Back"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
