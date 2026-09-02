use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use ratatui_image::{Resize, StatefulImage, thread::ThreadProtocol};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

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
            self.render_thumbnail(frame, area);
            return;
        }
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        self.render_table(frame, chunks[0]);
        self.render_thumbnail(frame, chunks[1]);
    }

    fn render_table(&self, frame: &mut Frame, area: Rect) {
        let header_cells = ["ID", "Resolution", "Category", "Purity"].iter().map(|h| {
            Cell::from(Span::styled(
                *h,
                Style::default()
                    .fg(self.theme.primary)
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
            let category = w.category.as_deref().unwrap_or("-");
            let purity = w.purity.as_deref().unwrap_or("-");
            Row::new(vec![
                Cell::from(Span::raw(w.id.clone())),
                Cell::from(Span::styled(dims, Style::default().fg(Color::DarkGray))),
                Cell::from(Span::raw(category)),
                Cell::from(Span::raw(purity)),
            ])
            .height(1)
        });

        let widths = [
            Constraint::Percentage(25),
            Constraint::Percentage(30),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
        ];

        let mut state = TableState::default().with_selected(Some(self.selected));
        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .title(" Results ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            )
            .row_highlight_style(
                Style::default()
                    .bg(self.theme.bg_selected)
                    .fg(self.theme.fg_primary)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut state);
    }

    fn render_thumbnail(&mut self, frame: &mut Frame, area: Rect) {
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
                    Style::default().fg(Color::DarkGray),
                ))
                .alignment(Alignment::Center),
            ])
            .block(
                Block::default()
                    .title(" Preview ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
            frame.render_widget(thumb, area);
        }
    }
}
