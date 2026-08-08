use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::infrastructure::config_loader::AppConfig;
use crate::ui::theme::Theme;
use crate::ui::widgets::help_bar::HelpBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigField {
    ApiKey,
    DownloadDir,
    PuritySfw,
    PuritySketchy,
    PurityNsfw,
    CategoryGeneral,
    CategoryAnime,
    CategoryPeople,
}

impl ConfigField {
    pub fn all() -> Vec<ConfigField> {
        vec![
            ConfigField::ApiKey,
            ConfigField::DownloadDir,
            ConfigField::PuritySfw,
            ConfigField::PuritySketchy,
            ConfigField::PurityNsfw,
            ConfigField::CategoryGeneral,
            ConfigField::CategoryAnime,
            ConfigField::CategoryPeople,
        ]
    }

    pub fn is_toggle(&self) -> bool {
        matches!(
            self,
            ConfigField::PuritySfw
                | ConfigField::PuritySketchy
                | ConfigField::PurityNsfw
                | ConfigField::CategoryGeneral
                | ConfigField::CategoryAnime
                | ConfigField::CategoryPeople
        )
    }
}

pub struct ConfigScreen<'a> {
    theme: &'a Theme,
    config: &'a AppConfig,
    selected: usize,
    editing: bool,
    input: &'a str,
    cursor: usize,
}

impl<'a> ConfigScreen<'a> {
    pub fn new(
        theme: &'a Theme,
        config: &'a AppConfig,
        selected: usize,
        editing: bool,
        input: &'a str,
        cursor: usize,
    ) -> Self {
        Self {
            theme,
            config,
            selected,
            editing,
            input,
            cursor,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(" Configuration", Style::default().fg(self.theme.primary).bold())),
            Line::from(""),
        ];

        lines.push(section_label("API Key"));
        let key_display = match &self.config.wallhaven_api_key {
            Some(key) if !key.is_empty() => {
                let masked = format!("{}...{}", &key[..4], &key[key.len().saturating_sub(4)..]);
                Span::styled(format!("  {masked}"), Style::default().fg(self.theme.success))
            }
            _ => Span::styled("  (not set)", Style::default().fg(self.theme.secondary)),
        };
        lines.push(Line::from(key_display));
        lines.push(Line::from(""));

        lines.push(section_label("Download Directory"));
        if self.editing && self.selected == 1 {
            lines.push(input_line(self.input, self.cursor, self.theme));
        } else {
            lines.push(Line::from(format!("  {}", self.config.download_dir.display())));
        }
        lines.push(Line::from(""));

        lines.push(section_label("Purity"));
        lines.push(toggle_line("SFW", self.config.purity_sfw, self.theme));
        lines.push(toggle_line("Sketchy", self.config.purity_sketchy, self.theme));
        let nsfw_warn = self.config.purity_nsfw && self.config.wallhaven_api_key.as_ref().map_or(true, |k| k.is_empty());
        lines.push(toggle_line_warn("NSFW", self.config.purity_nsfw, nsfw_warn, self.theme));
        lines.push(Line::from(""));

        lines.push(section_label("Categories"));
        lines.push(toggle_line("General", self.config.category_general, self.theme));
        lines.push(toggle_line("Anime", self.config.category_anime, self.theme));
        lines.push(toggle_line("People", self.config.category_people, self.theme));

        let list_items: Vec<ListItem> = lines.into_iter().map(ListItem::new).collect();
        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(" Settings ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.primary)),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.primary)
                    .fg(self.theme.background)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, chunks[0], &mut state);

        let help = if self.editing {
            vec![("Enter", "Confirm"), ("Esc", "Cancel"), ("Ctrl+S", "Save")]
        } else {
            vec![
                ("Enter", "Edit / Toggle"),
                ("↑/↓", "Navigate"),
                ("Esc", "Back"),
            ]
        };
        HelpBar::new(&help, self.theme).render(frame, chunks[1]);
    }
}

fn section_label(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!(" {text}"),
        Style::default().fg(Color::Yellow).bold(),
    ))
}

fn toggle_text(on: bool) -> &'static str {
    if on { " [ON]" } else { "[OFF]" }
}

fn toggle_line(label: &str, on: bool, theme: &Theme) -> Line<'static> {
    let style = if on {
        Style::default().fg(theme.success)
    } else {
        Style::default().fg(theme.secondary)
    };
    Line::from(vec![
        Span::raw(format!("  {label}")),
        Span::styled(toggle_text(on), style),
    ])
}

fn toggle_line_warn(label: &str, on: bool, warn: bool, theme: &Theme) -> Line<'static> {
    let marker_style = if on {
        Style::default().fg(theme.success)
    } else {
        Style::default().fg(theme.secondary)
    };
    let mut spans = vec![
        Span::raw(format!("  {label}")),
        Span::styled(toggle_text(on), marker_style),
    ];
    if warn {
        spans.push(Span::styled(" (needs API key)", Style::default().fg(theme.warning)));
    }
    Line::from(spans)
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
            Style::default().bg(theme.primary).fg(theme.background),
        ),
        Span::raw(after),
    ])
}
