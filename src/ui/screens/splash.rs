use ratatui::{
    layout::Rect,
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
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

        let ascii_logo = r#"
 __        __   _   _           _____  _    _ _____ 
 \ \      / /__| | | |_      __  ( _ )| |  | |_   _|
  \ \ /\ / / _ \ | | \ \ /\ / / / _ \| |  | | | |  
   \ V  V /  __/ |_| |\ V  V / | (_) | |__| |_| |_ 
    \_/\_/ \___|\___/  \_/\_/   \___/ \____/|_____|
        "#;

        let splash = Paragraph::new(vec![
            Line::from(ascii_logo).centered(),
            Line::from(""),
            Line::from(Span::styled("Terminal Wallpaper Manager", Style::default().fg(self.theme.primary).bold())).centered(),
            Line::from(""),
            Line::from(Span::raw("v0.1.0")).centered(),
            Line::from(""),
            Line::from(vec![
                Span::styled("s", Style::default().fg(self.theme.secondary).bold()),
                Span::raw(" Search  "),
                Span::styled("c", Style::default().fg(self.theme.secondary).bold()),
                Span::raw(" Settings  "),
                Span::styled("q", Style::default().fg(self.theme.error).bold()),
                Span::raw(" Quit"),
            ]).centered(),
        ])
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)));

        frame.render_widget(splash, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("s", "Search"),
            ("c", "Settings"),
            ("q", "Quit"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
