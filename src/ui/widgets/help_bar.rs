use ratatui::{
    Frame,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::ui::theme::Theme;

pub struct HelpBar<'a> {
    shortcuts: &'a [(&'a str, &'a str)],
    theme: &'a Theme,
}

impl<'a> HelpBar<'a> {
    pub fn new(shortcuts: &'a [(&'a str, &'a str)], theme: &'a Theme) -> Self {
        Self { shortcuts, theme }
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans: Vec<Span> = self
            .shortcuts
            .iter()
            .enumerate()
            .flat_map(|(i, (key, desc))| {
                let mut items = vec![
                    Span::styled(
                        format!(" {key} "),
                        Style::default().fg(self.theme.secondary).bold(),
                    ),
                    Span::raw(*desc),
                ];
                if i < self.shortcuts.len() - 1 {
                    items.push(Span::raw(" | "));
                }
                items
            })
            .collect();

        let help_bar = Paragraph::new(Line::from(spans));
        frame.render_widget(help_bar, area);
    }
}
