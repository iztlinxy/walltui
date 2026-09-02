use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_image::{Resize, StatefulImage, thread::ThreadProtocol};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

pub struct ImageCard<'a> {
    wallpaper: &'a Wallpaper,
    theme: &'a Theme,
    thumbnail_image: &'a mut Option<ThreadProtocol>,
}

impl<'a> ImageCard<'a> {
    pub fn new(
        wallpaper: &'a Wallpaper,
        theme: &'a Theme,
        thumbnail_image: &'a mut Option<ThreadProtocol>,
    ) -> Self {
        Self {
            wallpaper,
            theme,
            thumbnail_image,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        self.render_info(frame, chunks[0]);
        self.render_preview(frame, chunks[1]);
    }

    fn render_info(&self, frame: &mut Frame, area: Rect) {
        let w = self.wallpaper;
        let dims = match (w.width, w.height) {
            (Some(w), Some(h)) => format!("{w}x{h}"),
            _ => "Unknown".to_string(),
        };

        let views_str = w.views.map(|v| format!("Views: {v}")).unwrap_or_default();
        let favorites_str = w
            .favorites
            .map(|v| format!("Favorites: {v}"))
            .unwrap_or_default();
        let category_str = w.category.as_deref().unwrap_or("N/A");
        let purity_str = w.purity.as_deref().unwrap_or("N/A");

        let mut lines = vec![
            Line::from(Span::styled(
                "Title: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(w.title.as_str()),
            Line::from(""),
            Line::from(Span::styled(
                "Photographer: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(w.photographer.as_str()),
            Line::from(""),
            Line::from(Span::styled(
                "Dimensions: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(dims),
            Line::from(""),
            Line::from(Span::styled(
                "Category: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(category_str),
            Line::from(""),
            Line::from(Span::styled(
                "Purity: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(purity_str),
        ];

        if !views_str.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Views: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(views_str));
        }

        if !favorites_str.is_empty() {
            lines.push(Line::from(Span::styled(
                "Favorites: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(favorites_str));
        }

        if let Some(color) = &w.avg_color {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Avg Color: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(color.as_str()));
        }

        if let Some(web_url) = &w.web_url {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Link: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(web_url.as_str()));
        }

        if !w.tags.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Tags: ",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )));
            let tags_line = w.tags.join(", ");
            lines.push(Line::from(tags_line));
        }

        let card = Paragraph::new(lines).block(
            Block::default()
                .title(" Image Detail ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );

        frame.render_widget(card, area);
    }

    fn render_preview(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(image) = self.thumbnail_image.as_mut() {
            frame.render_stateful_widget(
                StatefulImage::default().resize(Resize::Crop(None)),
                area,
                image,
            );
        } else {
            let preview = Paragraph::new(vec![
                Line::from(Span::styled(
                    "Loading...",
                    Style::default().fg(Color::DarkGray),
                ))
                .centered(),
            ])
            .block(
                Block::default()
                    .title(" Preview ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
            frame.render_widget(preview, area);
        }
    }
}
