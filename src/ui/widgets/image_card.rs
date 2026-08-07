use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

pub struct ImageCard<'a> {
    wallpaper: &'a Wallpaper,
    theme: &'a Theme,
}

impl<'a> ImageCard<'a> {
    pub fn new(wallpaper: &'a Wallpaper, theme: &'a Theme) -> Self {
        Self { wallpaper, theme }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let w = self.wallpaper;
        let dims = match (w.width, w.height) {
            (Some(w), Some(h)) => format!("{w}x{h}"),
            _ => "Unknown".to_string(),
        };

        let color_info = w.avg_color.as_deref().unwrap_or("N/A");

        let lines = vec![
            Line::from(Span::styled(
                "Title: ",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from(w.title.as_str()),
            Line::from(""),
            Line::from(Span::styled(
                "Photographer: ",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from(w.photographer.as_str()),
            Line::from(""),
            Line::from(Span::styled(
                "Dimensions: ",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from(dims),
            Line::from(""),
            Line::from(Span::styled(
                "Avg Color: ",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from(color_info),
            Line::from(""),
            Line::from(Span::styled(
                "URL: ",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from(w.url.as_str()),
        ];

        let card = Paragraph::new(lines).block(
            Block::default()
                .title(" Image Detail ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );

        frame.render_widget(card, area);
    }
}
