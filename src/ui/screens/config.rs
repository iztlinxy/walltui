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

    pub fn is_editable(&self) -> bool {
        matches!(self, ConfigField::ApiKey | ConfigField::DownloadDir)
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

        let lines: Vec<Line> = self.build_lines();

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
                ("Enter", "Toggle"),
                ("↑/↓", "Navigate"),
                ("Esc", "Back"),
            ]
        };
        HelpBar::new(&help, self.theme).render(frame, chunks[1]);
    }

    fn build_lines(&self) -> Vec<Line<'static>> {
        vec![
            self.api_key_line(),
            self.download_dir_line(),
            self.purity_line("SFW", self.config.purity_sfw, false),
            self.purity_line("Sketchy", self.config.purity_sketchy, false),
            self.purity_line(
                "NSFW",
                self.config.purity_nsfw,
                self.config.purity_nsfw && self.config.wallhaven_api_key.as_ref().map_or(true, |k| k.is_empty()),
            ),
            self.category_line("General", self.config.category_general),
            self.category_line("Anime", self.config.category_anime),
            self.category_line("People", self.config.category_people),
        ]
    }

    fn api_key_line(&self) -> Line<'static> {
        let value = match &self.config.wallhaven_api_key {
            Some(key) if !key.is_empty() => {
                let masked = format!("{}...{}", &key[..4], &key[key.len().saturating_sub(4)..]);
                Span::styled(masked, Style::default().fg(self.theme.success))
            }
            _ => Span::styled("(not set)", Style::default().fg(self.theme.secondary)),
        };
        Line::from(vec![
            Span::styled("API Key:  ", Style::default().fg(Color::Yellow).bold()),
            value,
        ])
    }

    fn download_dir_line(&self) -> Line<'static> {
        if self.editing && self.selected == 1 {
            let before: String = self.input.chars().take(self.cursor).collect();
            let mut iter = self.input.chars().skip(self.cursor);
            let cursor_char = iter.next().unwrap_or(' ').to_string();
            let after: String = iter.collect();
            Line::from(vec![
                Span::styled("Download:  ", Style::default().fg(Color::Yellow).bold()),
                Span::raw("> "),
                Span::raw(before),
                Span::styled(cursor_char, Style::default().bg(self.theme.primary).fg(self.theme.background)),
                Span::raw(after),
            ])
        } else {
            Line::from(vec![
                Span::styled("Download:  ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(self.config.download_dir.to_string_lossy().to_string()),
            ])
        }
    }

    fn purity_line(&self, label: &str, on: bool, warn: bool) -> Line<'static> {
        let marker = if on {
            Span::styled("[ON]", Style::default().fg(self.theme.success))
        } else {
            Span::styled("[OFF]", Style::default().fg(self.theme.secondary))
        };
        let mut spans = vec![
            Span::styled(format!("{label}:  "), Style::default().fg(Color::Yellow).bold()),
            marker,
        ];
        if warn {
            spans.push(Span::styled(" (needs API key)", Style::default().fg(self.theme.warning)));
        }
        Line::from(spans)
    }

    fn category_line(&self, label: &str, on: bool) -> Line<'static> {
        let marker = if on {
            Span::styled("[ON]", Style::default().fg(self.theme.success))
        } else {
            Span::styled("[OFF]", Style::default().fg(self.theme.secondary))
        };
        Line::from(vec![
            Span::styled(format!("{label}:  "), Style::default().fg(Color::Yellow).bold()),
            marker,
        ])
    }
}
