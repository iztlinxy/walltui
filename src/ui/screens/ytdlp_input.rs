use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::Theme;

pub struct YtDlpInputScreen<'a> {
    url: &'a str,
    cursor_pos: usize,
    cursor_visible: bool,
    cursor_style: &'a str,
    error: Option<&'a str>,
    theme: &'a Theme,
}

impl<'a> YtDlpInputScreen<'a> {
    pub fn new(
        url: &'a str,
        cursor_pos: usize,
        cursor_visible: bool,
        cursor_style: &'a str,
        error: Option<&'a str>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            url,
            cursor_pos,
            cursor_visible,
            cursor_style,
            error,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Min(1),
        ])
        .split(area);

        let title = Paragraph::new(Line::from(Span::styled(
            "yt-dlp - Animated Wallpaper",
            Style::default()
                .fg(self.theme.primary)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);

        let url_display = if self.cursor_visible {
            let before = &self.url[..self.cursor_pos.min(self.url.len())];
            let after = &self.url[self.cursor_pos.min(self.url.len())..];
            let cursor_char = match self.cursor_style {
                "line" => "|",
                "underline" => "\u{2581}",
                _ => "\u{2588}",
            };
            format!("{}{}{}", before, cursor_char, after)
        } else {
            self.url.to_string()
        };

        let url_input = Paragraph::new(Line::from(Span::styled(
            format!("URL: {}", url_display),
            Style::default().fg(self.theme.foreground),
        )))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary))
                .title("Video URL"),
        );
        frame.render_widget(url_input, chunks[1]);

        let hint = Paragraph::new(Line::from(Span::styled(
            "Paste URL from YouTube, TikTok, Reddit, Vimeo, Twitter/X...",
            Style::default().fg(self.theme.fg_disabled),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[2]);

        let supported = vec![
            Line::from(Span::styled(
                "Supported sites:",
                Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "YouTube, TikTok, Vimeo, Reddit, Twitter/X, Bilibili, Instagram",
                Style::default().fg(self.theme.foreground),
            )),
            Line::from(Span::styled(
                "And 1000+ more via yt-dlp",
            Style::default().fg(self.theme.fg_secondary),
            )),
        ];
        let sites = Paragraph::new(supported)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary))
                    .title("Supported Sites"),
            );
        frame.render_widget(sites, chunks[3]);

        let keys = vec![
            Span::styled("[Enter] ", Style::default().fg(self.theme.primary)),
            Span::styled("Fetch metadata  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[Ctrl+V] ", Style::default().fg(self.theme.primary)),
            Span::styled("Paste  ", Style::default().fg(self.theme.foreground)),
            Span::styled("[Esc] ", Style::default().fg(self.theme.primary)),
            Span::styled("Back", Style::default().fg(self.theme.foreground)),
        ];
        let keybar = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
        frame.render_widget(keybar, chunks[4]);

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
}
