use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};
use ratatui_image::thread::ThreadProtocol;

use crate::core::models::Wallpaper;
use crate::ui::theme::{StyleKey, Theme};
use crate::ui::widgets::dialog::ConfirmDialog;
use crate::ui::widgets::help_bar::HelpBar;
use crate::ui::widgets::image_list::ImageList;

pub struct GalleryScreen<'a> {
    wallpapers: &'a [Wallpaper],
    selected: usize,
    thumbnail_image: &'a mut Option<ThreadProtocol>,
    theme: &'a Theme,
    download_dir: &'a PathBuf,
    editing: bool,
    edit_input: &'a str,
    edit_cursor: usize,
    cursor_visible: bool,
    cursor_style: &'a str,
    confirm_dialog: &'a Option<ConfirmDialog>,
    loading: bool,
}

impl<'a> GalleryScreen<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        wallpapers: &'a [Wallpaper],
        selected: usize,
        thumbnail_image: &'a mut Option<ThreadProtocol>,
        theme: &'a Theme,
        download_dir: &'a PathBuf,
        editing: bool,
        edit_input: &'a str,
        edit_cursor: usize,
        cursor_visible: bool,
        cursor_style: &'a str,
        confirm_dialog: &'a Option<ConfirmDialog>,
        loading: bool,
    ) -> Self {
        Self {
            wallpapers,
            selected,
            thumbnail_image,
            theme,
            download_dir,
            editing,
            edit_input,
            edit_cursor,
            cursor_visible,
            cursor_style,
            confirm_dialog,
            loading,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        if self.editing {
            self.render_rename_header(frame, chunks[0]);
        } else {
            self.render_header(frame, chunks[0]);
        }

        if self.loading {
            let loading = Paragraph::new(Line::from(Span::styled(
                "Loading gallery...",
                self.theme.resolve(StyleKey::Secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(loading, chunks[1]);
        } else if self.wallpapers.is_empty() {
            let empty = Paragraph::new(Line::from(Span::styled(
                "No wallpapers found in download folder.",
                self.theme.resolve(StyleKey::Secondary),
            )))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            );
            frame.render_widget(empty, chunks[1]);
        } else {
            ImageList::new(
                self.wallpapers,
                self.selected,
                self.thumbnail_image,
                self.theme,
            )
            .render(frame, chunks[1]);
        }

        let help_shortcuts: Vec<(&str, &str)> = if self.editing {
            vec![("Enter", "Confirm"), ("Esc", "Cancel")]
        } else {
            vec![
                ("↑/↓", "Navigate"),
                ("Enter", "Open"),
                ("w", "Set wallpaper"),
                ("r", "Rename"),
                ("d", "Delete"),
                ("Esc", "Back"),
            ]
        };
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[2]);

        if let Some(dialog) = self.confirm_dialog {
            self.render_confirm_dialog(frame, area, dialog);
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header = Paragraph::new(Line::from(vec![
            Span::styled(" Gallery ", self.theme.resolve(StyleKey::Title)),
            Span::raw(" | Folder: "),
            Span::styled(
                self.download_dir.to_string_lossy().to_string(),
                self.theme.resolve(StyleKey::Secondary),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.resolve(StyleKey::BorderFocused)),
        );
        frame.render_widget(header, area);
    }

    fn render_rename_header(&self, frame: &mut Frame, area: Rect) {
        let before: String = self.edit_input.chars().take(self.edit_cursor).collect();
        let iter = self.edit_input.chars().skip(self.edit_cursor);
        let rest: String = iter.collect();
        let cursor_str = if self.cursor_visible {
            match self.cursor_style {
                "line" => "▏",
                "underline" => "_",
                _ => "█",
            }
        } else {
            ""
        };
        let (cursor_char, after) = if cursor_str.is_empty() {
            (String::new(), rest)
        } else {
            let mut chars = rest.chars();
            let current = chars.next().unwrap_or(' ').to_string();
            (current, chars.collect())
        };

        let line = Line::from(vec![
            Span::styled(" Rename: ", self.theme.resolve(StyleKey::Title)),
            Span::raw(before),
            Span::styled(
                cursor_char,
                if self.cursor_visible {
                    self.theme.resolve(StyleKey::Cursor)
                } else {
                    Style::default()
                },
            ),
            Span::raw(cursor_str),
            Span::raw(after),
        ]);
        let header = Paragraph::new(line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.resolve(StyleKey::BorderFocused)),
        );
        frame.render_widget(header, area);
    }

    fn render_confirm_dialog(&self, frame: &mut Frame, area: Rect, dialog: &ConfirmDialog) {
        let width = (dialog.message.len() as u16 + 14)
            .min(area.width - 4)
            .max(30);
        let height = 5u16;
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let dialog_area = Rect::new(x, y, width, height);
        frame.render_widget(Clear, dialog_area);
        let paragraph = Paragraph::new(Line::from(vec![
            Span::raw(dialog.message.clone()),
            Span::raw(" [y/N]"),
        ]))
        .block(
            Block::default()
                .title(" Confirm ")
                .borders(Borders::ALL)
                .border_style(self.theme.resolve(StyleKey::Error)),
        );
        frame.render_widget(paragraph, dialog_area);
    }
}
