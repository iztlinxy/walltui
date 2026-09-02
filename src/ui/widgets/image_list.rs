use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use ratatui_image::{Resize, StatefulImage, thread::ThreadProtocol};

use crate::core::models::Wallpaper;
use crate::ui::theme::{StyleKey, Theme};

pub struct ImageList<'a> {
    wallpapers: &'a [Wallpaper],
    selected: usize,
    thumbnail_image: &'a mut Option<ThreadProtocol>,
    theme: &'a Theme,
    fullscreen: bool,
}

impl<'a> ImageList<'a> {
    pub fn new(
        wallpapers: &'a [Wallpaper],
        selected: usize,
        thumbnail_image: &'a mut Option<ThreadProtocol>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            wallpapers,
            selected,
            thumbnail_image,
            theme,
            fullscreen: false,
        }
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.fullscreen {
            self.render_image(frame, area);
            return;
        }
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        self.render_table(frame, chunks[0]);
        self.render_preview(frame, chunks[1]);
    }

    fn render_image(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(image) = self.thumbnail_image.as_mut() {
            frame.render_stateful_widget(
                StatefulImage::default().resize(Resize::Crop(None)),
                area,
                image,
            );
        } else {
            let thumb = Paragraph::new(vec![
                Line::from(Span::styled(
                    "Loading...",
                    Style::default().fg(self.theme.fg_secondary),
                ))
                .alignment(Alignment::Center),
            ])
            .block(
                Block::default()
                    .title(" Preview ")
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(thumb, area);
        }
    }

    fn render_table(&self, frame: &mut Frame, area: Rect) {
        let header_cells = ["ID / TITLE", "RES", "RATIO", "FAV"].iter().map(|h| {
            Cell::from(Span::styled(
                *h,
                Style::default()
                    .fg(self.theme.fg_primary)
                    .add_modifier(Modifier::BOLD),
            ))
        });
        let header = Row::new(header_cells)
            .style(Style::default().bg(self.theme.bg_surface))
            .height(1);

        let rows = self.wallpapers.iter().map(|w| {
            let dims = match (w.width, w.height) {
                (Some(w), Some(h)) => format!("{w}x{h}"),
                _ => "?".to_string(),
            };
            let ratio = w.ratio.as_deref().unwrap_or("-");
            let fav = format!("★ {}", format_number(w.favorites.unwrap_or(0)));
            let id_title = format!("{}  {}", w.id, w.title);
            Row::new(vec![
                Cell::from(Span::raw(id_title)),
                Cell::from(Span::styled(dims, Style::default().fg(self.theme.fg_secondary))),
                Cell::from(Span::styled(ratio, Style::default().fg(self.theme.fg_secondary))),
                Cell::from(Span::styled(fav, Style::default().fg(self.theme.secondary))),
            ])
            .height(1)
        });

        let widths = [
            Constraint::Percentage(45),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ];

        let mut state = TableState::default().with_selected(Some(self.selected));
        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .title(" Results ")
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            )
            .row_highlight_style(
                Style::default()
                    .bg(self.theme.bg_selected)
                    .fg(self.theme.fg_contrast)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut state);
    }

    fn render_preview(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(4)])
            .split(area);

        self.render_image(frame, chunks[0]);

        // Info panel below thumbnail.
        if let Some(w) = self.wallpapers.get(self.selected) {
            let dims = match (w.width, w.height) {
                (Some(wi), Some(hi)) => format!("{wi}x{hi}"),
                _ => "Unknown".to_string(),
            };
            let size = format_size(w.file_size.unwrap_or(0));
            let tags = if w.tags.is_empty() {
                "-".to_string()
            } else {
                w.tags.join("  ")
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled("Title: ", self.theme.resolve(StyleKey::Title)),
                    Span::raw(&w.title),
                ]),
                Line::from(vec![
                    Span::styled("Res: ", self.theme.resolve(StyleKey::Title)),
                    Span::raw(dims),
                    Span::raw(" | "),
                    Span::styled("Size: ", self.theme.resolve(StyleKey::Title)),
                    Span::raw(size),
                ]),
                Line::from(vec![
                    Span::styled("Tags: ", self.theme.resolve(StyleKey::Title)),
                    Span::styled(tags, Style::default().fg(self.theme.fg_secondary)),
                ]),
            ];

            let info = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(info, chunks[1]);
        }
    }
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}m", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_size(bytes: u64) -> String {
    if bytes == 0 {
        "-".to_string()
    } else if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{bytes} B")
    }
}
