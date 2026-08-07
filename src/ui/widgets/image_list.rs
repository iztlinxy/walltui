use ratatui::{
    layout::Rect,
    style::{Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;

pub struct ImageList<'a> {
    wallpapers: &'a [Wallpaper],
    selected: usize,
    theme: &'a Theme,
}

impl<'a> ImageList<'a> {
    pub fn new(wallpapers: &'a [Wallpaper], selected: usize, theme: &'a Theme) -> Self {
        Self {
            wallpapers,
            selected,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .wallpapers
            .iter()
            .map(|w| {
                let title = &w.title;
                let dims = match (w.width, w.height) {
                    (Some(w), Some(h)) => format!("{w}x{h}"),
                    _ => "?".to_string(),
                };
                let line = Line::from(vec![
                    Span::styled(format!(" {} ", w.provider), Style::default().fg(self.theme.secondary)),
                    Span::raw(format!("{title} ")),
                    Span::styled(format!("[{dims}]"), Style::default().fg(self.theme.secondary)),
                ]);
                ListItem::new(line)
            })
            .collect();

        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(items)
            .block(Block::default().title(" Results ").borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)))
            .highlight_style(Style::default().bg(self.theme.primary).fg(self.theme.background))
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, area, &mut state);
    }
}
