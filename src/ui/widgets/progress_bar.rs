use ratatui::{
    layout::Rect,
    style::{Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::ui::theme::Theme;

pub struct ProgressBar<'a> {
    progress: u8,
    label: &'a str,
    theme: &'a Theme,
}

impl<'a> ProgressBar<'a> {
    pub fn new(progress: u8, label: &'a str, theme: &'a Theme) -> Self {
        Self { progress, label, theme }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let gauge = Gauge::default()
            .block(Block::default().title(self.label).borders(Borders::ALL).border_style(Style::default().fg(self.theme.primary)))
            .gauge_style(Style::default().fg(self.theme.success))
            .percent(self.progress as u16);

        frame.render_widget(gauge, area);
    }
}
