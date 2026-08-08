use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::core::download::{DownloadStatus, DownloadTask};
use crate::ui::screens::resolution_select::ResolutionOption;
use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;

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
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        if self.tasks.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No downloads. Press 'd' on an image to start downloading.",
                Style::default().fg(self.theme.secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
            frame.render_widget(empty, chunks[0]);
        } else {
            let items: Vec<ListItem> = self
                .tasks
                .iter()
                .map(|task| {
                    let status_str = match &task.status {
                        DownloadStatus::Queued => "Queued".to_string(),
                        DownloadStatus::Active => format!("Downloading {}%", task.progress),
                        DownloadStatus::Completed => "Completed".to_string(),
                        DownloadStatus::Failed(e) => format!("Failed: {e}"),
                    };
                    let status_style = match &task.status {
                        DownloadStatus::Queued => Style::default().fg(self.theme.secondary),
                        DownloadStatus::Active => Style::default().fg(self.theme.primary),
                        DownloadStatus::Completed => Style::default().fg(self.theme.success),
                        DownloadStatus::Failed(_) => Style::default().fg(self.theme.error),
                    };
                    let res_str = match &task.resolution {
                        Some(ResolutionOption::Original) => String::new(),
                        Some(opt) => format!(" [{}]", opt.dimensions().0) ,
                        None => String::new(),
                    };
                    let line = Line::from(vec![
                        Span::styled(format!("[{status_str}] "), status_style),
                        Span::raw(&task.wallpaper.title),
                        Span::styled(res_str, Style::default().fg(self.theme.secondary)),
                    ]);
                    ListItem::new(line)
                })
                .collect();

            let mut state =
                ratatui::widgets::ListState::default().with_selected(Some(self.selected));
            let list = List::new(items)
                .block(
                    Block::default()
                        .title(" Downloads ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(self.theme.primary)),
                )
                .highlight_style(
                    Style::default()
                        .bg(self.theme.primary)
                        .fg(self.theme.background),
                )
                .highlight_symbol("▶ ");

            frame.render_stateful_widget(list, chunks[0], &mut state);
        }

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("x", "Cancel"),
            ("r", "Retry"),
            ("c", "Clear"),
            ("Esc", "Back"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
