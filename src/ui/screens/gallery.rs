use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;
use crate::ui::widgets::image_list::ImageList;

pub struct GalleryScreen<'a> {
    wallpapers: &'a [Wallpaper],
    selected: usize,
    thumbnail_lines: &'a [Line<'static>],
    theme: &'a Theme,
    download_dir: &'a PathBuf,
}

impl<'a> GalleryScreen<'a> {
    pub fn new(
        wallpapers: &'a [Wallpaper],
        selected: usize,
        thumbnail_lines: &'a [Line<'static>],
        theme: &'a Theme,
        download_dir: &'a PathBuf,
    ) -> Self {
        Self {
            wallpapers,
            selected,
            thumbnail_lines,
            theme,
            download_dir,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let header = Paragraph::new(Line::from(vec![
            Span::styled(" Gallery ", Style::default().fg(self.theme.primary).bold()),
            Span::raw(" | Folder: "),
            Span::styled(
                self.download_dir.to_string_lossy().to_string(),
                Style::default().fg(self.theme.secondary),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );
        frame.render_widget(header, chunks[0]);

        if self.wallpapers.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No wallpapers found in download folder.",
                Style::default().fg(self.theme.secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            );
            frame.render_widget(empty, chunks[1]);
        } else {
            ImageList::new(self.wallpapers, self.selected, self.thumbnail_lines, self.theme)
                .render(frame, chunks[1]);
        }

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("↑/↓", "Navigate"),
            ("Enter", "Open"),
            ("r", "Reload"),
            ("Esc", "Back"),
        ];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[2]);
    }
}
