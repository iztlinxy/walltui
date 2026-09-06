use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect, Size},
    style::{Color, Modifier, Style},
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
    show_favorites: bool,
    show_preview: bool,
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
            show_favorites: true,
            show_preview: true,
        }
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn show_favorites(mut self, show: bool) -> Self {
        self.show_favorites = show;
        self
    }

    pub fn show_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.fullscreen {
            self.render_image(frame, area);
            return;
        }
        if !self.show_preview {
            self.render_table(frame, area);
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
        let headers: Vec<&str> = if self.show_favorites {
            vec!["ID / TITLE", "RES", "RATIO", "FAV"]
        } else {
            vec!["ID / TITLE", "RES", "RATIO"]
        };
        let header_cells = headers.iter().map(|h| {
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
            let id_title = format!("{}  {}", w.id, w.title);
            let mut cells = vec![
                Cell::from(Span::raw(id_title)),
                Cell::from(Span::styled(dims, Style::default().fg(self.theme.fg_secondary))),
                Cell::from(Span::styled(ratio, Style::default().fg(self.theme.fg_secondary))),
            ];
            if self.show_favorites {
                let fav = format!("★ {}", format_number(w.favorites.unwrap_or(0)));
                cells.push(Cell::from(Span::styled(
                    fav,
                    Style::default().fg(self.theme.secondary),
                )));
            }
            Row::new(cells).height(1)
        });

        let widths = if self.show_favorites {
            vec![
                Constraint::Percentage(45),
                Constraint::Percentage(25),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]
        } else {
            vec![
                Constraint::Percentage(50),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
        };

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
        let block = Block::default()
            .title(" Preview ")
            .borders(Borders::ALL)
            .border_style(self.theme.resolve(StyleKey::BorderFocused));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(6)])
            .split(inner);

        let image_area = chunks[0];
        let fit_size = self
            .thumbnail_image
            .as_ref()
            .and_then(|i| i.size_for(Resize::Fit(None), Size::new(image_area.width, image_area.height)));
        if let Some(image) = self.thumbnail_image.as_mut() {
            let render_area = fit_size
                .map(|s| center_rect(image_area, s.width, s.height))
                .unwrap_or(image_area);
            frame.render_stateful_widget(
                StatefulImage::default().resize(Resize::Fit(None)),
                render_area,
                image,
            );
        } else {
            let loading = Paragraph::new(
                Line::from(Span::styled(
                    "Loading...",
                    Style::default().fg(self.theme.fg_secondary),
                ))
                .alignment(Alignment::Center),
            );
            frame.render_widget(loading, chunks[0]);
        }

        if let Some(w) = self.wallpapers.get(self.selected) {
            let dims = match (w.width, w.height) {
                (Some(wi), Some(hi)) => format!("{wi}x{hi}"),
                _ => "Unknown".to_string(),
            };
            let size = format_size(w.file_size.unwrap_or(0));
            let tags = if w.tags.is_empty() {
                "-".to_string()
            } else {
                w.tags.iter().map(|t| format!("[{t}]")).collect::<Vec<_>>().join(" ")
            };

            let color_spans = if w.colors.is_empty() {
                vec![Span::styled("-", Style::default().fg(self.theme.fg_secondary))]
            } else {
                let mut spans = Vec::new();
                for c in w.colors.iter().take(5) {
                    if let Some(color) = parse_hex_color(c) {
                        spans.push(Span::raw("["));
                        spans.push(Span::styled("██", Style::default().fg(color)));
                        spans.push(Span::raw("] "));
                    }
                }
                if spans.is_empty() {
                    spans.push(Span::styled("-", Style::default().fg(self.theme.fg_secondary)));
                }
                spans
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
                Line::from(Span::styled(
                    tags,
                    Style::default().fg(self.theme.fg_secondary),
                )),
                Line::from(color_spans),
            ];

            let info = Paragraph::new(lines).alignment(Alignment::Center);
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

fn parse_hex_color(s: &str) -> Option<Color> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
