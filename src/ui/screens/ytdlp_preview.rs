use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

pub struct YtDlpPreviewScreen<'a> {
    wallpaper: &'a Wallpaper,
    clip_start: u64,
    clip_end: u64,
    downloading: bool,
    download_progress: u8,
    download_status: &'a str,
    error: Option<&'a str>,
    theme: &'a Theme,
}

impl<'a> YtDlpPreviewScreen<'a> {
    pub fn new(
        wallpaper: &'a Wallpaper,
        clip_start: u64,
        clip_end: u64,
        downloading: bool,
        download_progress: u8,
        download_status: &'a str,
        error: Option<&'a str>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            wallpaper,
            clip_start,
            clip_end,
            downloading,
            download_progress,
            download_status,
            error,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

        let title = Paragraph::new(Line::from(Span::styled(
            format!("Preview: \"{}\"", self.wallpaper.title),
            Style::default()
                .fg(self.theme.primary)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);

        let duration = self.wallpaper.duration_secs.unwrap_or(0);
        let channel = &self.wallpaper.photographer;
        let dims = match (self.wallpaper.width, self.wallpaper.height) {
            (Some(w), Some(h)) => format!("{}x{}", w, h),
            _ => "unknown".to_string(),
        };
        let meta = vec![
            Line::from(Span::styled(
                format!("Duration: {}s | Channel: {}", duration, channel),
                Style::default().fg(self.theme.foreground),
            )),
            Line::from(Span::styled(
                format!("Resolution: {}", dims),
                Style::default().fg(self.theme.foreground),
            )),
        ];
        let meta_para = Paragraph::new(meta).alignment(Alignment::Center);
        frame.render_widget(meta_para, chunks[1]);

        self.render_clip_range(frame, chunks[2], duration);

        if self.downloading {
            self.render_download_progress(frame, chunks[3]);
        } else {
            let keys = self.render_action_keys();
            let keybar = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
            frame.render_widget(keybar, chunks[3]);
        }

        if let Some(err) = self.error {
            let error_para = Paragraph::new(Line::from(Span::styled(
                format!("Error: {}", err),
                Style::default().fg(self.theme.error),
            )))
            .alignment(Alignment::Center);
            let error_area = Rect {
                x: area.x,
                y: area.y + area.height - 2,
                width: area.width,
                height: 1,
            };
            frame.render_widget(error_para, error_area);
        }
    }

    fn render_clip_range(&self, frame: &mut Frame, area: Rect, total_duration: u64) {
        let clip_duration = self.clip_end.saturating_sub(self.clip_start);
        let bar_width = area.width.saturating_sub(4) as usize;
        if bar_width < 10 {
            return;
        }

        let start_pos = if total_duration > 0 {
            (self.clip_start as f64 / total_duration as f64 * bar_width as f64) as usize
        } else {
            0
        };
        let end_pos = if total_duration > 0 {
            (self.clip_end as f64 / total_duration as f64 * bar_width as f64) as usize
        } else {
            bar_width
        };

        let mut bar = String::from("  ");
        for i in 0..bar_width {
            if i >= start_pos && i <= end_pos {
                bar.push('\u{2588}');
            } else {
                bar.push('\u{2591}');
            }
        }
        bar.push_str("  ");

        let time_marks = if total_duration > 0 {
            format!(
                "  0s{:>width$}{}s{:>width$}{}s",
                "",
                self.clip_start,
                "",
                total_duration,
                width = (bar_width / 3).saturating_sub(2)
            )
        } else {
            String::new()
        };

        let clip_info = format!(
            "Clip: {}s-{}s ({}s clip)",
            self.clip_start,
            self.clip_end,
            clip_duration
        );

        let lines = vec![
            Line::from(Span::styled(
                "Clip range:",
                Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                bar,
                Style::default().fg(self.theme.success),
            )),
            Line::from(Span::styled(
                time_marks,
                Style::default().fg(self.theme.fg_disabled),
            )),
            Line::from(Span::styled(
                clip_info,
                Style::default().fg(self.theme.foreground),
            )),
        ];

        let para = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary))
                    .title("Clip Selection"),
            );
        frame.render_widget(para, area);
    }

    fn render_download_progress(&self, frame: &mut Frame, area: Rect) {
        let bar_width = area.width.saturating_sub(4) as usize;
        let filled = (bar_width as u64 * self.download_progress as u64 / 100) as usize;
        let empty = bar_width.saturating_sub(filled);

        let mut bar = String::from("[");
        for _ in 0..filled {
            bar.push('\u{2588}');
        }
        for _ in 0..empty {
            bar.push('\u{2591}');
        }
        bar.push(']');

        let lines = vec![
            Line::from(Span::styled(
                self.download_status,
                Style::default().fg(self.theme.primary),
            )),
            Line::from(Span::styled(
                format!("{} {}%", bar, self.download_progress),
                Style::default().fg(self.theme.success),
            )),
            Line::from(Span::styled(
                "[x] Cancel",
                Style::default().fg(self.theme.error),
            )),
        ];

        let para = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary))
                    .title("Downloading"),
            );
        frame.render_widget(para, area);
    }

    fn render_action_keys(&self) -> Vec<Span<'a>> {
        vec![
            Span::styled("[d] ", Style::default().fg(self.theme.primary)),
            Span::styled("Download clip  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[Esc] ", Style::default().fg(self.theme.primary)),
            Span::styled("Back  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[\u{2190}/\u{2192}] ", Style::default().fg(self.theme.primary)),
            Span::styled("Adjust start  ", Style::default().fg(self.theme.foreground)),
            Span::styled("Shift+[\u{2190}/\u{2192}] ", Style::default().fg(self.theme.primary)),
            Span::styled("Adjust end", Style::default().fg(self.theme.foreground)),
        ]
    }
}
