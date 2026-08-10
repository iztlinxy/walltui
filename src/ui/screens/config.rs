use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::infrastructure::config_loader::AppConfig;
use crate::ui::theme::{StyleKey, Theme};
use crate::ui::widgets::help_bar::HelpBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigField {
    ApiKey,
    DownloadDir,
    ThemeName,
    CursorStyle,
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
            ConfigField::ThemeName,
            ConfigField::CursorStyle,
            ConfigField::PuritySfw,
            ConfigField::PuritySketchy,
            ConfigField::PurityNsfw,
            ConfigField::CategoryGeneral,
            ConfigField::CategoryAnime,
            ConfigField::CategoryPeople,
        ]
    }

    pub fn is_editable(&self) -> bool {
        matches!(
            self,
            ConfigField::ApiKey | ConfigField::DownloadDir | ConfigField::ThemeName | ConfigField::CursorStyle
        )
    }
}

pub struct ConfigScreen<'a> {
    theme: &'a Theme,
    config: &'a AppConfig,
    selected: usize,
    selected_field: ConfigField,
    editing: bool,
    input: &'a str,
    cursor: usize,
    cursor_visible: bool,
    cursor_style: &'a str,
}

impl<'a> ConfigScreen<'a> {
    pub fn new(
        theme: &'a Theme,
        config: &'a AppConfig,
        selected: usize,
        selected_field: ConfigField,
        editing: bool,
        input: &'a str,
        cursor: usize,
        cursor_visible: bool,
        cursor_style: &'a str,
    ) -> Self {
        Self {
            theme,
            config,
            selected,
            selected_field,
            editing,
            input,
            cursor,
            cursor_visible,
            cursor_style,
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
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.bg_selected)
                    .fg(self.theme.fg_primary)
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
            self.theme_name_line(),
            self.cursor_style_line(),
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
                Span::styled(masked, self.theme.resolve(StyleKey::Success))
            }
            _ => Span::styled("(not set)", self.theme.resolve(StyleKey::Secondary)),
        };
        Line::from(vec![
            Span::styled("API Key:  ", self.theme.resolve(StyleKey::Title)),
            value,
        ])
    }

    fn download_dir_line(&self) -> Line<'static> {
        let editing = self.editing && self.selected_field == ConfigField::DownloadDir;
        if editing {
            let before: String = self.input.chars().take(self.cursor).collect();
            let iter = self.input.chars().skip(self.cursor);
            let rest: String = iter.collect();
            let cursor_str = if self.cursor_visible {
                match self.cursor_style {
                    "line" => "▏".to_string(),
                    "underline" => "_".to_string(),
                    _ => "█".to_string(),
                }
            } else {
                String::new()
            };
            let (cursor_char, after) = if cursor_str.is_empty() {
                (String::new(), rest)
            } else {
                let mut chars = rest.chars();
                let current = chars.next().unwrap_or(' ').to_string();
                let after: String = chars.collect();
                (current, after)
            };
            Line::from(vec![
                Span::styled("Download:  ", self.theme.resolve(StyleKey::Title)),
                Span::raw("> "),
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
            ])
        } else {
            Line::from(vec![
                Span::styled("Download:  ", self.theme.resolve(StyleKey::Title)),
                Span::raw(self.config.download_dir.to_string_lossy().to_string()),
            ])
        }
    }

    fn theme_name_line(&self) -> Line<'static> {
        let editing = self.editing && self.selected_field == ConfigField::ThemeName;
        if editing {
            self.editable_line("Theme", self.input, self.cursor)
        } else {
            Line::from(vec![
                Span::styled("Theme:  ", self.theme.resolve(StyleKey::Title)),
                Span::raw(self.config.theme_name.clone()),
            ])
        }
    }

    fn cursor_style_line(&self) -> Line<'static> {
        let editing = self.editing && self.selected_field == ConfigField::CursorStyle;
        if editing {
            self.editable_line("Cursor", self.input, self.cursor)
        } else {
            Line::from(vec![
                Span::styled("Cursor:  ", self.theme.resolve(StyleKey::Title)),
                Span::raw(self.config.cursor_style.clone()),
            ])
        }
    }

    fn editable_line(&self, label: &str, input: &str, cursor: usize) -> Line<'static> {
        let before: String = input.chars().take(cursor).collect();
        let rest: String = input.chars().skip(cursor).collect();
        let cursor_str = if self.cursor_visible {
            match self.cursor_style {
                "line" => "▏".to_string(),
                "underline" => "_".to_string(),
                _ => "█".to_string(),
            }
        } else {
            String::new()
        };
        let (cursor_char, after) = if cursor_str.is_empty() {
            (String::new(), rest)
        } else {
            let mut chars = rest.chars();
            let current = chars.next().unwrap_or(' ').to_string();
            let after: String = chars.collect();
            (current, after)
        };
        Line::from(vec![
            Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
            Span::raw("> "),
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
        ])
    }

    fn purity_line(&self, label: &str, on: bool, warn: bool) -> Line<'static> {
        let marker = if on {
            Span::styled("[ON]", self.theme.resolve(StyleKey::Success))
        } else {
            Span::styled("[OFF]", self.theme.resolve(StyleKey::Secondary))
        };
        let mut spans = vec![
            Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
            marker,
        ];
        if warn {
            spans.push(Span::styled(
                " (needs API key)",
                self.theme.resolve(StyleKey::Warning),
            ));
        }
        Line::from(spans)
    }

    fn category_line(&self, label: &str, on: bool) -> Line<'static> {
        let marker = if on {
            Span::styled("[ON]", self.theme.resolve(StyleKey::Success))
        } else {
            Span::styled("[OFF]", self.theme.resolve(StyleKey::Secondary))
        };
        Line::from(vec![
            Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
            marker,
        ])
    }
}
