use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::core::download::{DownloadStatus, DownloadTask};
use crate::ui::screens::resolution_select::ResolutionOption;
use crate::ui::theme::{StyleKey, Theme};

pub struct DownloadScreen<'a> {
    tasks: &'a [DownloadTask],
    selected: usize,
    theme: &'a Theme,
}

impl<'a> DownloadScreen<'a> {
    pub fn new(tasks: &'a [DownloadTask], selected: usize, theme: &'a Theme) -> Self {
        Self {
            tasks,
            selected,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(2)])
            .split(area);

        if self.tasks.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No downloads. Press 'd' on an image to start downloading.",
                self.theme.resolve(StyleKey::Secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(empty, chunks[0]);
        } else {
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
                .split(chunks[0]);

            self.render_table(frame, body[0]);
            self.render_preview(frame, body[1]);
        }

        self.render_footer(frame, chunks[1]);
    }

    fn render_table(&self, frame: &mut Frame, area: Rect) {
        let header_cells = ["STATUS", "TITLE", "RES"].iter().map(|h| {
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

        let rows = self.tasks.iter().map(|task| {
            let (status_str, status_style) = match &task.status {
                DownloadStatus::Queued => ("Queued".to_string(), self.theme.resolve(StyleKey::Secondary)),
                DownloadStatus::Active => (
                    format!("Downloading {}%", task.progress),
                    self.theme.resolve(StyleKey::Primary),
                ),
                DownloadStatus::Completed => ("Completed".to_string(), self.theme.resolve(StyleKey::Success)),
                DownloadStatus::Failed(e) => (format!("Failed: {e}"), self.theme.resolve(StyleKey::Error)),
            };
            let res_str = match &task.resolution {
                Some(ResolutionOption::Original) => "Original".to_string(),
                Some(opt) => opt.label().to_string(),
                None => "Original".to_string(),
            };
            Row::new(vec![
                Cell::from(Span::styled(status_str, status_style)),
                Cell::from(Span::raw(&task.wallpaper.title)),
                Cell::from(Span::styled(res_str, Style::default().fg(self.theme.fg_secondary))),
            ])
            .height(1)
        });

        let mut state = TableState::default().with_selected(Some(self.selected));
        let table = Table::new(rows, [
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ])
        .header(header)
        .block(
            Block::default()
                .title(" Downloads ")
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

    fn render_preview(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(5)])
            .split(area);

        // Placeholder for the image preview.
        let task = self.tasks.get(self.selected);
        let placeholder_text = match task {
            Some(t) => match &t.status {
                DownloadStatus::Queued => "Queued",
                DownloadStatus::Active => &format!("Downloading {}%", t.progress),
                DownloadStatus::Completed => "Download completed",
                DownloadStatus::Failed(_) => "Download failed",
            },
            None => "No preview",
        };
        let image_placeholder = Paragraph::new(vec![
            Line::from(Span::styled(
                placeholder_text,
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
        frame.render_widget(image_placeholder, chunks[0]);

        // Info panel.
        if let Some(task) = task {
            let w = &task.wallpaper;
            let dims = match (w.width, w.height) {
                (Some(wi), Some(hi)) => format!("{wi}x{hi}"),
                _ => "Unknown".to_string(),
            };
            let size = format_size(w.file_size.unwrap_or(0));
            let status = match &task.status {
                DownloadStatus::Queued => "Queued".to_string(),
                DownloadStatus::Active => format!("{}%", task.progress),
                DownloadStatus::Completed => "Completed".to_string(),
                DownloadStatus::Failed(e) => format!("Failed: {e}"),
            };
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
                    Span::styled("Status: ", self.theme.resolve(StyleKey::Title)),
                    Span::raw(status),
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

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let items = vec![
            ("x", "Cancel"),
            ("r", "Retry"),
            ("c", "Clear"),
            ("Esc", "Back"),
        ];
        let spans: Vec<Span> = items
            .iter()
            .enumerate()
            .flat_map(|(i, (key, desc))| {
                let mut parts = vec![
                    Span::styled(
                        format!("[ {key} ]"),
                        self.theme
                            .resolve(StyleKey::Secondary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" {desc}")),
                ];
                if i < items.len() - 1 {
                    parts.push(Span::raw("  "));
                }
                parts
            })
            .collect();

        let footer = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(self.theme.resolve(StyleKey::Border)),
            );
        frame.render_widget(footer, area);
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
