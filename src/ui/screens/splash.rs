use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::{StyleKey, Theme};
use crate::ui::widgets::help_bar::HelpBar;

pub struct SplashScreen<'a> {
    theme: &'a Theme,
    recent_downloads: &'a [String],
}

impl<'a> SplashScreen<'a> {
    pub fn new(theme: &'a Theme, recent_downloads: &'a [String]) -> Self {
        Self {
            theme,
            recent_downloads,
        }
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
        lines.push(Line::from(Span::raw(format!("v{}", env!("CARGO_PKG_VERSION")))).centered());
        lines.push(Line::from(""));
        lines.push(
            Line::from(vec![
                Span::styled(
                    "s",
                    self.theme
                        .resolve(StyleKey::Secondary)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" Search  "),
                Span::styled(
                    "g",
                    self.theme
                        .resolve(StyleKey::Secondary)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" Gallery  "),
                Span::styled(
                    "c",
                    self.theme
                        .resolve(StyleKey::Secondary)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" Settings  "),
                Span::styled(
                    "q",
                    self.theme
                        .resolve(StyleKey::Error)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::raw(" Quit"),
            ])
            .centered(),
        );

        if !self.recent_downloads.is_empty() {
            lines.push(Line::from(""));
            lines.push(
                Line::from(Span::styled(
                    "Recent downloads",
                    self.theme.resolve(StyleKey::Title),
                ))
                .centered(),
            );
            for title in self.recent_downloads.iter().take(5) {
                lines.push(
                    Line::from(Span::styled(
                        format!("• {title}"),
                        self.theme.resolve(StyleKey::Secondary),
                    ))
                    .centered(),
                );
            }
        }

        let splash = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(splash, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("s", "Search"),
            ("g", "Gallery"),
            ("c", "Settings"),
            ("q", "Quit"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
