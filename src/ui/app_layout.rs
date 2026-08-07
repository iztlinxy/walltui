use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::theme::Theme;
use crate::ui::screens::Screen;

pub struct AppLayout<'a> {
    theme: &'a Theme,
    active_screen: Screen,
    active_provider: &'a str,
}

impl<'a> AppLayout<'a> {
    pub fn new(theme: &'a Theme, active_screen: Screen, active_provider: &'a str) -> Self {
        Self {
            theme,
            active_screen,
            active_provider,
        }
    }

    pub fn render(&self, frame: &mut Frame) -> ratatui::layout::Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let header = Paragraph::new(Line::from(vec![
            Span::styled(" WallTUI ", Style::default().fg(self.theme.primary).bold()),
            Span::raw(" | Provider: "),
            Span::styled(self.active_provider, Style::default().fg(self.theme.secondary)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)));
        frame.render_widget(header, chunks[0]);

        let screen_name = match self.active_screen {
            Screen::Splash => "Splash",
            Screen::Search => "Search",
            Screen::Detail => "Detail",
            Screen::Download => "Downloads",
            Screen::Config => "Settings",
        };
        let footer_text = format!(" {screen_name} ");
        let footer = Paragraph::new(Line::from(vec![
            Span::styled(&footer_text, Style::default().fg(self.theme.foreground)),
            Span::raw(" | "),
            Span::styled(" ? help ", Style::default().fg(self.theme.secondary)),
            Span::raw(" | "),
            Span::styled(" q quit ", Style::default().fg(self.theme.error)),
        ]))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)));
        frame.render_widget(footer, chunks[2]);

        chunks[1]
    }
}
