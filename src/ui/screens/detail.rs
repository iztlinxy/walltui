use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::core::models::Wallpaper;
use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;
use crate::ui::widgets::image_card::ImageCard;

pub struct DetailScreen<'a> {
    wallpaper: &'a Wallpaper,
    theme: &'a Theme,
}

impl<'a> DetailScreen<'a> {
    pub fn new(wallpaper: &'a Wallpaper, theme: &'a Theme) -> Self {
        Self { wallpaper, theme }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        ImageCard::new(self.wallpaper, self.theme).render(frame, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> =
            vec![("d", "Download"), ("o", "Open URL"), ("Esc", "Back")];
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}
