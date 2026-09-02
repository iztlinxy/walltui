use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::infrastructure::config_loader::AppConfig;
use crate::ui::theme::{StyleKey, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigField {
    ApiKey,
    DownloadDir,
    CursorStyle,
    PuritySfw,
    PuritySketchy,
    PurityNsfw,
    CategoryGeneral,
    CategoryAnime,
    CategoryPeople,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSection {
    General,
    Purity,
    Categories,
}

impl ConfigSection {
    pub fn title(&self) -> &'static str {
        match self {
            ConfigSection::General => "General",
            ConfigSection::Purity => "Content Purity",
            ConfigSection::Categories => "Categories",
        }
    }
}

impl ConfigField {
    pub fn all() -> Vec<ConfigField> {
        vec![
            ConfigField::ApiKey,
            ConfigField::DownloadDir,
            ConfigField::CursorStyle,
            ConfigField::PuritySfw,
            ConfigField::PuritySketchy,
            ConfigField::PurityNsfw,
            ConfigField::CategoryGeneral,
            ConfigField::CategoryAnime,
            ConfigField::CategoryPeople,
        ]
    }

    pub fn section(&self) -> ConfigSection {
        match self {
            ConfigField::ApiKey | ConfigField::DownloadDir | ConfigField::CursorStyle => {
                ConfigSection::General
            }
            ConfigField::PuritySfw | ConfigField::PuritySketchy | ConfigField::PurityNsfw => {
                ConfigSection::Purity
            }
            ConfigField::CategoryGeneral
            | ConfigField::CategoryAnime
            | ConfigField::CategoryPeople => ConfigSection::Categories,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ConfigField::ApiKey => "API Key",
            ConfigField::DownloadDir => "Download Folder",
            ConfigField::CursorStyle => "Cursor Style",
            ConfigField::PuritySfw => "SFW",
            ConfigField::PuritySketchy => "Sketchy",
            ConfigField::PurityNsfw => "NSFW",
            ConfigField::CategoryGeneral => "General",
            ConfigField::CategoryAnime => "Anime",
            ConfigField::CategoryPeople => "People",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ConfigField::ApiKey => {
                "Your personal Wallhaven API key. Required to search or download NSFW content. Leave empty to use the public API with limited access."
            }
            ConfigField::DownloadDir => {
                "Folder where downloaded wallpapers are saved. The path supports ~ as a shortcut for the user home directory."
            }
            ConfigField::CursorStyle => {
                "Visual style of the text cursor in input fields. Choose between block, line or underline."
            }
            ConfigField::PuritySfw => {
                "Include Safe For Work wallpapers in search results. This is the default and recommended filter."
            }
            ConfigField::PuritySketchy => {
                "Include Sketchy or borderline content in search results."
            }
            ConfigField::PurityNsfw => {
                "Include Not Safe For Work content. Requires a valid Wallhaven API key and enabling this option in your Wallhaven account."
            }
            ConfigField::CategoryGeneral => {
                "Include general photography and digital art in search results."
            }
            ConfigField::CategoryAnime => {
                "Include anime and illustration content in search results."
            }
            ConfigField::CategoryPeople => {
                "Include people and portrait content in search results."
            }
        }
    }

    pub fn default_value(&self) -> &'static str {
        match self {
            ConfigField::ApiKey => "(not set)",
            ConfigField::DownloadDir => "Downloads folder",
            ConfigField::CursorStyle => "block",
            ConfigField::PuritySfw => "ON",
            ConfigField::PuritySketchy => "OFF",
            ConfigField::PurityNsfw => "OFF",
            ConfigField::CategoryGeneral => "ON",
            ConfigField::CategoryAnime => "ON",
            ConfigField::CategoryPeople => "OFF",
        }
    }

    pub fn is_editable(&self) -> bool {
        matches!(
            self,
            ConfigField::ApiKey | ConfigField::DownloadDir | ConfigField::CursorStyle
        )
    }

    pub fn is_toggle(&self) -> bool {
        !self.is_editable()
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
    #[allow(clippy::too_many_arguments)]
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
            .constraints([
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(area);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(chunks[0]);

        self.render_options(frame, body[0]);
        self.render_description(frame, body[1]);
        self.render_footer(frame, chunks[1]);
    }

    fn render_options(&self, frame: &mut Frame, area: Rect) {
        let fields = ConfigField::all();
        let mut items: Vec<ListItem> = Vec::new();
        let mut field_indices: Vec<Option<usize>> = Vec::new();
        let mut last_section: Option<ConfigSection> = None;

        for (i, field) in fields.iter().enumerate() {
            let section = field.section();
            if last_section != Some(section) {
                items.push(ListItem::new(Line::from(vec![Span::raw("")])));
                field_indices.push(None);
                items.push(ListItem::new(Line::from(vec![Span::styled(
                    format!(" {} ", section.title()),
                    Style::default()
                        .fg(self.theme.fg_primary)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )])));
                field_indices.push(None);
                last_section = Some(section);
            }

            let is_selected = i == self.selected;
            let line = self.build_field_line(*field, is_selected);
            items.push(ListItem::new(line));
            field_indices.push(Some(i));
        }

        let visual_selected = field_indices
            .iter()
            .position(|idx| *idx == Some(self.selected))
            .unwrap_or(0);

        let mut state = ListState::default().with_selected(Some(visual_selected));
        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Settings ")
                    .borders(Borders::ALL)
                    .border_style(self.theme.resolve(StyleKey::BorderFocused)),
            )
            .highlight_style(
                Style::default()
                    .bg(self.theme.bg_selected)
                    .fg(self.theme.fg_contrast)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut state);
    }

    fn build_field_line(&self, field: ConfigField, is_selected: bool) -> Line<'static> {
        let label = field.label();
        match field {
            ConfigField::ApiKey => self.api_key_line(label),
            ConfigField::DownloadDir => self.download_dir_line(label),
            ConfigField::CursorStyle => self.cursor_style_line(label),
            _ => self.toggle_line(label, self.field_value(field), is_selected),
        }
    }

    fn field_value(&self, field: ConfigField) -> bool {
        match field {
            ConfigField::PuritySfw => self.config.purity_sfw,
            ConfigField::PuritySketchy => self.config.purity_sketchy,
            ConfigField::PurityNsfw => self.config.purity_nsfw,
            ConfigField::CategoryGeneral => self.config.category_general,
            ConfigField::CategoryAnime => self.config.category_anime,
            ConfigField::CategoryPeople => self.config.category_people,
            _ => false,
        }
    }

    fn api_key_line(&self, label: &str) -> Line<'static> {
        let editing = self.editing && self.selected_field == ConfigField::ApiKey;
        if editing {
            self.editable_line(label, self.input, self.cursor)
        } else {
            let value = match &self.config.wallhaven_api_key {
                Some(key) if !key.is_empty() => Span::styled(
                    "••••••••",
                    Style::default().fg(self.theme.fg_secondary),
                ),
                _ => Span::styled("(not set)", self.theme.resolve(StyleKey::Secondary)),
            };
            Line::from(vec![
                Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
                value,
            ])
        }
    }

    fn download_dir_line(&self, label: &str) -> Line<'static> {
        let editing = self.editing && self.selected_field == ConfigField::DownloadDir;
        if editing {
            self.editable_line(label, self.input, self.cursor)
        } else {
            Line::from(vec![
                Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
                Span::raw(self.config.download_dir.to_string_lossy().to_string()),
            ])
        }
    }

    fn cursor_style_line(&self, label: &str) -> Line<'static> {
        let value = self.config.cursor_style.clone();
        Line::from(vec![
            Span::styled(format!("{label}:  "), self.theme.resolve(StyleKey::Title)),
            Span::raw("< "),
            Span::styled(value, Style::default().fg(self.theme.secondary).bold()),
            Span::raw(" >"),
        ])
    }

    fn toggle_line(&self, label: &str, on: bool, is_selected: bool) -> Line<'static> {
        let check = if on { "✓" } else { " " };
        let text = if on { "ON" } else { "OFF" };
        let (label_fg, box_fg, text_fg) = if is_selected {
            (
                self.theme.fg_contrast,
                self.theme.fg_contrast,
                self.theme.fg_contrast,
            )
        } else if on {
            (self.theme.fg_primary, self.theme.success, self.theme.success)
        } else {
            (
                self.theme.fg_primary,
                self.theme.fg_disabled,
                self.theme.fg_disabled,
            )
        };
        Line::from(vec![
            Span::styled(format!("{label}:  "), Style::default().fg(label_fg)),
            Span::styled(format!("[{check}]"), Style::default().fg(box_fg)),
            Span::styled(format!(" {text}"), Style::default().fg(text_fg)),
        ])
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

    fn render_description(&self, frame: &mut Frame, area: Rect) {
        let field = self.selected_field;
        let title = field.label();
        let description = field.description();
        let default_value = field.default_value();

        let lines = vec![
            Line::from(vec![
                Span::styled(title, self.theme.resolve(StyleKey::Title)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Description:  ", self.theme.resolve(StyleKey::Title)),
                Span::styled(description, Style::default().fg(self.theme.fg_secondary)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Default:  ", self.theme.resolve(StyleKey::Title)),
                Span::styled(default_value, Style::default().fg(self.theme.secondary)),
            ]),
        ];

        let panel = Paragraph::new(lines).block(
            Block::default()
                .title(" Description ")
                .borders(Borders::ALL)
                .border_style(self.theme.resolve(StyleKey::BorderFocused)),
        );
        frame.render_widget(panel, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let items = if self.editing {
            vec![
                ("Enter", "Confirm"),
                ("Esc", "Cancel"),
                ("Ctrl+S", "Save"),
            ]
        } else {
            vec![
                ("↑/↓", "Navigate"),
                ("Enter", "Edit / Toggle"),
                ("←/→", "Cycle Cursor"),
                ("Esc", "Back"),
            ]
        };
        let spans: Vec<Span> = items
            .iter()
            .enumerate()
            .flat_map(|(i, (key, desc))| {
                let mut parts = vec![
                    Span::styled(
                        format!("[ {key} ]"),
                        self.theme
                            .resolve(StyleKey::Secondary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" {desc}")),
                ];
                if i < items.len() - 1 {
                    parts.push(Span::raw("  "));
                }
                parts
            })
            .collect();

        let footer = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(self.theme.resolve(StyleKey::Border)),
            );
        frame.render_widget(footer, area);
    }
}
