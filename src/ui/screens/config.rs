use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;

pub struct ConfigScreen<'a> {
    theme: &'a Theme,
    download_dir: &'a str,
    input: &'a str,
    editing: bool,
    cursor: usize,
}

impl<'a> ConfigScreen<'a> {
    pub fn new(
        theme: &'a Theme,
        download_dir: &'a str,
        input: &'a str,
        editing: bool,
        cursor: usize,
    ) -> Self {
        Self {
            theme,
            download_dir,
            input,
            editing,
            cursor,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let mut config_lines = vec![
            Line::from(Span::styled(
                " Configuration",
                Style::default().fg(self.theme.primary).bold(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                " API Keys:",
                Style::default().fg(self.theme.secondary).bold(),
            )),
            Line::from("   Wallhaven: (not set)"),
            Line::from("   Pixiv: (not set)"),
            Line::from(""),
            Line::from(Span::styled(
                " Download Directory:",
                Style::default().fg(self.theme.secondary).bold(),
            )),
        ];

        if self.editing {
            config_lines.push(input_line(self.input, self.cursor, self.theme));
            config_lines.push(Line::from(Span::styled(
                "   [editing]",
                Style::default().fg(self.theme.warning),
            )));
        } else {
            config_lines.push(Line::from(format!("   {}", self.download_dir)));
        }

        let config = Paragraph::new(config_lines).block(
            Block::default()
                .title(" Settings ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );

        frame.render_widget(config, chunks[0]);

        let help_shortcuts: Vec<(&str, &str)> = if self.editing {
            vec![("Enter", "Confirm"), ("Esc", "Cancel"), ("Ctrl+S", "Save")]
        } else {
            vec![("Enter", "Edit folder"), ("Esc", "Back")]
        };
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[1]);
    }
}

fn input_line(input: &str, cursor: usize, theme: &Theme) -> Line<'static> {
    let before: String = input.chars().take(cursor).collect();
    let mut iter = input.chars().skip(cursor);
    let cursor_char = iter.next().unwrap_or(' ').to_string();
    let after: String = iter.collect();

    Line::from(vec![
        Span::raw("> "),
        Span::raw(before),
        Span::styled(
            cursor_char,
            Style::default()
                .bg(theme.primary)
                .fg(theme.background),
        ),
        Span::raw(after),
    ])
}
