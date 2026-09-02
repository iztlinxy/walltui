use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::screens::Screen;
use crate::ui::theme::Theme;

pub struct AppLayout<'a> {
    theme: &'a Theme,
    #[allow(dead_code)]
    active_screen: Screen,
    active_provider: &'a str,
    toast: Option<&'a str>,
}

impl<'a> AppLayout<'a> {
    pub fn new(
        theme: &'a Theme,
        active_screen: Screen,
        active_provider: &'a str,
        toast: Option<&'a str>,
    ) -> Self {
        Self {
            theme,
            active_screen,
            active_provider,
            toast,
        }
    }

    pub fn render(&self, frame: &mut Frame) -> ratatui::layout::Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(frame.area());

        let mut header_spans = vec![
            Span::styled(
                format!("WALLTUI v{}", env!("CARGO_PKG_VERSION")),
                Style::default().fg(self.theme.fg_primary).bold(),
            ),
            Span::raw(" | [Provider: "),
            Span::styled(
                self.active_provider,
                Style::default().fg(self.theme.success),
            ),
            Span::raw("]"),
        ];
        if let Some(toast) = self.toast {
            header_spans.push(Span::raw(" | "));
            header_spans.push(Span::styled(toast, Style::default().fg(self.theme.success)));
        }

        let header = Paragraph::new(Line::from(header_spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
        frame.render_widget(header, chunks[0]);

        chunks[1]
    }
}
