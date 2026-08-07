use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

pub struct ImageList<'a> {
    wallpapers: &'a [Wallpaper],
    selected: usize,
    thumbnail_lines: &'a [String],
    theme: &'a Theme,
}

impl<'a> ImageList<'a> {
    pub fn new(
        wallpapers: &'a [Wallpaper],
        selected: usize,
        thumbnail_lines: &'a [String],
        theme: &'a Theme,
    ) -> Self {
        Self {
            wallpapers,
            selected,
            thumbnail_lines,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        self.render_list(frame, chunks[0]);
        self.render_thumbnail(frame, chunks[1]);
    }

    fn render_list(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .wallpapers
            .iter()
            .map(|w| {
                let dims = match (w.width, w.height) {
                    (Some(w), Some(h)) => format!("{w}x{h}"),
                    _ => "?".to_string(),
                };
                let category = w.category.as_deref().unwrap_or("");
                let purity = w.purity.as_deref().unwrap_or("");
                let tags_preview = if w.tags.is_empty() {
                    String::new()
                } else {
                    let tags: Vec<&str> = w.tags.iter().take(3).map(|s| s.as_str()).collect();
                    format!(" [{}]", tags.join(", "))
                };

                let line = Line::from(vec![
                    Span::styled(
                        format!(" {} ", w.provider),
                        Style::default().fg(self.theme.primary),
                    ),
                    Span::raw(format!("{} ", w.title)),
                    Span::styled(format!("[{}]", dims), Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!(" {} {}", category, purity),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(tags_preview),
                ]);
                ListItem::new(line)
            })
            .collect();

        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Results ")
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

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn render_thumbnail(&self, frame: &mut Frame, area: Rect) {
        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(
                "Thumbnail",
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
        ];

        lines.push(Line::from(""));

        if self.thumbnail_lines.is_empty() || self.thumbnail_lines == ["Loading..."] {
            lines.push(
                Line::from(Span::styled(
                    "Loading...",
                    Style::default().fg(Color::DarkGray),
                ))
                .alignment(Alignment::Center),
            );
        } else {
            for line in self.thumbnail_lines {
                lines.push(Line::from(Span::styled(
                    line.as_str(),
                    Style::default().fg(Color::Green),
                )));
            }
        }

        let thumb = Paragraph::new(lines).block(
            Block::default()
                .title(" Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );

        frame.render_widget(thumb, area);
    }
}
