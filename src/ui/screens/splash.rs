use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;

pub struct SplashScreen<'a> {
    theme: &'a Theme,
}

impl<'a> SplashScreen<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = ratatui::layout::Layout::default()
            .constraints([
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(1),
            ])
            .split(area);

        let ascii_logo: Vec<Line> = vec![
            Line::from(r"  _    _   ___   _     _     _____  _   _  _____      ").centered(),
            Line::from(r" | |  | | / _ \\| |   | |   |_   _|| | | ||_   _|     ").centered(),
            Line::from(r" | |/\| || |_| || |   | |     | |  | | | |  | |       ").centered(),
            Line::from(r" |  /\  ||  _  || |__ | |__   | |  | |_| | _| |_      ").centered(),
            Line::from(r" \_/ \_/|_| |_||____||____|  |_|  |_____| |____|     ").centered(),
        ];

        let mut lines = ascii_logo;
        lines.push(Line::from(""));
        lines.push(
            Line::from(Span::styled(
                "Terminal Wallpaper Manager",
                Style::default().fg(self.theme.primary).bold(),
            ))
            .centered(),
        );
        lines.push(Line::from(""));
        lines.push(Line::from(Span::raw("v0.1.0")).centered());
        lines.push(Line::from(""));
        lines.push(
            Line::from(vec![
                Span::styled("s", Style::default().fg(self.theme.secondary).bold()),
                Span::raw(" Search  "),
                Span::styled("c", Style::default().fg(self.theme.secondary).bold()),
                Span::raw(" Settings  "),
                Span::styled("q", Style::default().fg(self.theme.error).bold()),
                Span::raw(" Quit"),
            ])
            .centered(),
        );

        let splash = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(splash, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> =
            vec![("s", "Search"), ("c", "Settings"), ("q", "Quit")];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
