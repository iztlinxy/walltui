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
            self.render_table(frame, chunks[0]);
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
                    format!("{} {}", mini_progress_bar(task.progress), task.progress),
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
            let title = truncate(&task.wallpaper.title, 28);
            let res = truncate(&res_str, 16);
            Row::new(vec![
                Cell::from(Span::styled(status_str, status_style)),
                Cell::from(Span::raw(title)),
                Cell::from(Span::styled(res, Style::default().fg(self.theme.fg_secondary))),
            ])
            .height(1)
        });

        let mut state = TableState::default().with_selected(Some(self.selected));
        let table = Table::new(rows, [
            Constraint::Length(14),
            Constraint::Percentage(55),
            Constraint::Percentage(30),
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

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let items = [
            ("x", "Cancel"),
            ("r", "Retry"),
            ("c", "Clear"),
            ("o", "Open folder"),
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

fn mini_progress_bar(progress: u8) -> String {
    let width = 8;
    let filled = ((progress as usize * width) / 100).min(width);
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn truncate(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        format!("{}...", text.chars().take(max_len.saturating_sub(3)).collect::<String>())
    }
}
