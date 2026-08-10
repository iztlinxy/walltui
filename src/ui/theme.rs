use std::collections::HashMap;
use std::str::FromStr;

use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BorderType {
    Rounded,
    Plain,
    Double,
    Thick,
}

impl Default for BorderType {
    fn default() -> Self {
        Self::Rounded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TitleAlignment {
    Left,
    Center,
    Right,
}

impl Default for TitleAlignment {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    // Semantic colors
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // Background layers
    pub bg_base: Color,
    pub bg_surface: Color,
    pub bg_overlay: Color,
    pub bg_selected: Color,

    // Text
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_disabled: Color,
    pub fg_contrast: Color,

    // Borders
    pub border_default: Color,
    pub border_focused: Color,
    pub border_error: Color,

    // Special
    pub cursor: Color,
    pub scrollbar: Color,
    pub scrollbar_thumb: Color,

    // Legacy aliases for existing code
    pub background: Color,
    pub foreground: Color,

    // Styles
    pub cursor_blink_rate_ms: u64,
    pub scrollbar_width: u8,
    pub border_type: BorderType,
    pub title_alignment: TitleAlignment,

    // Animation
    pub spinner_frames: Vec<String>,
    pub progress_bar_filled: String,
    pub progress_bar_empty: String,
    pub progress_bar_half: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleKey {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Muted,
    Highlight,
    Border,
    BorderFocused,
    BorderError,
    Background,
    Surface,
    Overlay,
    Selected,
    Foreground,
    ForegroundMuted,
    ForegroundDisabled,
    Cursor,
    Title,
    HelpKey,
    SelectedItem,
}

fn hex(c: &str) -> Color {
    Color::from_str(c).unwrap_or(Color::White)
}

impl Theme {
    pub fn resolve(&self, key: StyleKey) -> Style {
        match key {
            StyleKey::Primary => Style::default().fg(self.primary),
            StyleKey::Secondary => Style::default().fg(self.secondary),
            StyleKey::Success => Style::default().fg(self.success),
            StyleKey::Warning => Style::default().fg(self.warning),
            StyleKey::Error => Style::default().fg(self.error),
            StyleKey::Info => Style::default().fg(self.info),
            StyleKey::Muted => Style::default().fg(self.fg_secondary),
            StyleKey::Highlight => Style::default().fg(self.primary).add_modifier(Modifier::BOLD),
            StyleKey::Border => Style::default().fg(self.border_default),
            StyleKey::BorderFocused => Style::default().fg(self.border_focused),
            StyleKey::BorderError => Style::default().fg(self.border_error),
            StyleKey::Background => Style::default().bg(self.bg_base),
            StyleKey::Surface => Style::default().bg(self.bg_surface),
            StyleKey::Overlay => Style::default().bg(self.bg_overlay),
            StyleKey::Selected => Style::default().bg(self.bg_selected),
            StyleKey::Foreground => Style::default().fg(self.fg_primary),
            StyleKey::ForegroundMuted => Style::default().fg(self.fg_secondary),
            StyleKey::ForegroundDisabled => Style::default().fg(self.fg_disabled),
            StyleKey::Cursor => Style::default().fg(self.cursor),
            StyleKey::Title => Style::default()
                .fg(self.primary)
                .add_modifier(Modifier::BOLD),
            StyleKey::HelpKey => Style::default()
                .fg(self.secondary)
                .add_modifier(Modifier::BOLD),
            StyleKey::SelectedItem => Style::default()
                .bg(self.bg_selected)
                .fg(self.fg_primary)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: hex("#61AFEF"),
            secondary: hex("#C678DD"),
            success: hex("#98C379"),
            warning: hex("#E5C07B"),
            error: hex("#E06C75"),
            info: hex("#56B6C2"),
            bg_base: hex("#282C34"),
            bg_surface: hex("#3E4451"),
            bg_overlay: hex("#4B5263"),
            bg_selected: hex("#2C313C"),
            fg_primary: hex("#ABB2BF"),
            fg_secondary: hex("#828997"),
            fg_disabled: hex("#5C6370"),
            fg_contrast: hex("#FFFFFF"),
            border_default: hex("#3E4451"),
            border_focused: hex("#61AFEF"),
            border_error: hex("#E06C75"),
            cursor: hex("#528BFF"),
            scrollbar: hex("#4B5263"),
            scrollbar_thumb: hex("#61AFEF"),
            background: hex("#282C34"),
            foreground: hex("#ABB2BF"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn light() -> Self {
        Self {
            primary: hex("#4078F2"),
            secondary: hex("#A626A4"),
            success: hex("#50A14F"),
            warning: hex("#C18401"),
            error: hex("#E45649"),
            info: hex("#0184BC"),
            bg_base: hex("#FAFAFA"),
            bg_surface: hex("#EAEAEB"),
            bg_overlay: hex("#D7D7D8"),
            bg_selected: hex("#E5E5E6"),
            fg_primary: hex("#383A42"),
            fg_secondary: hex("#696C77"),
            fg_disabled: hex("#A0A1A7"),
            fg_contrast: hex("#FFFFFF"),
            border_default: hex("#EAEAEB"),
            border_focused: hex("#4078F2"),
            border_error: hex("#E45649"),
            cursor: hex("#4078F2"),
            scrollbar: hex("#D7D7D8"),
            scrollbar_thumb: hex("#4078F2"),
            background: hex("#FAFAFA"),
            foreground: hex("#383A42"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn midnight() -> Self {
        Self {
            primary: hex("#82AAFF"),
            secondary: hex("#C792EA"),
            success: hex("#C3E88D"),
            warning: hex("#FFCB6B"),
            error: hex("#F07178"),
            info: hex("#89DDFF"),
            bg_base: hex("#0F111A"),
            bg_surface: hex("#1A1C2A"),
            bg_overlay: hex("#24263A"),
            bg_selected: hex("#1C1F2E"),
            fg_primary: hex("#B0BEC5"),
            fg_secondary: hex("#676E95"),
            fg_disabled: hex("#464B5D"),
            fg_contrast: hex("#FFFFFF"),
            border_default: hex("#1A1C2A"),
            border_focused: hex("#82AAFF"),
            border_error: hex("#F07178"),
            cursor: hex("#82AAFF"),
            scrollbar: hex("#24263A"),
            scrollbar_thumb: hex("#82AAFF"),
            background: hex("#0F111A"),
            foreground: hex("#B0BEC5"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn solar() -> Self {
        Self {
            primary: hex("#268BD2"),
            secondary: hex("#D33682"),
            success: hex("#859900"),
            warning: hex("#B58900"),
            error: hex("#DC322F"),
            info: hex("#2AA198"),
            bg_base: hex("#002B36"),
            bg_surface: hex("#073642"),
            bg_overlay: hex("#0A3A47"),
            bg_selected: hex("#083C4A"),
            fg_primary: hex("#839496"),
            fg_secondary: hex("#586E75"),
            fg_disabled: hex("#465A63"),
            fg_contrast: hex("#FDF6E3"),
            border_default: hex("#073642"),
            border_focused: hex("#268BD2"),
            border_error: hex("#DC322F"),
            cursor: hex("#268BD2"),
            scrollbar: hex("#0A3A47"),
            scrollbar_thumb: hex("#268BD2"),
            background: hex("#002B36"),
            foreground: hex("#839496"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn forest() -> Self {
        Self {
            primary: hex("#7FBBB3"),
            secondary: hex("#D699B6"),
            success: hex("#A7C080"),
            warning: hex("#DBBC7F"),
            error: hex("#E67E80"),
            info: hex("#83C092"),
            bg_base: hex("#232A2E"),
            bg_surface: hex("#2D353B"),
            bg_overlay: hex("#3A454A"),
            bg_selected: hex("#2B3338"),
            fg_primary: hex("#D3C6AA"),
            fg_secondary: hex("#7A8478"),
            fg_disabled: hex("#596259"),
            fg_contrast: hex("#232A2E"),
            border_default: hex("#2D353B"),
            border_focused: hex("#7FBBB3"),
            border_error: hex("#E67E80"),
            cursor: hex("#7FBBB3"),
            scrollbar: hex("#3A454A"),
            scrollbar_thumb: hex("#7FBBB3"),
            background: hex("#232A2E"),
            foreground: hex("#D3C6AA"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn ocean() -> Self {
        Self {
            primary: hex("#7AA2F7"),
            secondary: hex("#BB9AF7"),
            success: hex("#9ECE6A"),
            warning: hex("#E0AF68"),
            error: hex("#F7768E"),
            info: hex("#73DACA"),
            bg_base: hex("#1A1B26"),
            bg_surface: hex("#24283B"),
            bg_overlay: hex("#2E3248"),
            bg_selected: hex("#232534"),
            fg_primary: hex("#A9B1D6"),
            fg_secondary: hex("#565F89"),
            fg_disabled: hex("#414868"),
            fg_contrast: hex("#FFFFFF"),
            border_default: hex("#24283B"),
            border_focused: hex("#7AA2F7"),
            border_error: hex("#F7768E"),
            cursor: hex("#7AA2F7"),
            scrollbar: hex("#2E3248"),
            scrollbar_thumb: hex("#7AA2F7"),
            background: hex("#1A1B26"),
            foreground: hex("#A9B1D6"),
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
            spinner_frames: vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            progress_bar_filled: "=".to_string(),
            progress_bar_empty: " ".to_string(),
            progress_bar_half: ">".to_string(),
        }
    }

    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        let data: ThemeData = toml::from_str(s)?;
        Ok(Self::from_data(data))
    }

    fn from_data(data: ThemeData) -> Self {
        let colors = data.colors;
        Self {
            primary: hex(&colors.primary),
            secondary: hex(&colors.secondary),
            success: hex(&colors.success),
            warning: hex(&colors.warning),
            error: hex(&colors.error),
            info: hex(&colors.info),
            bg_base: hex(&colors.bg_base),
            bg_surface: hex(&colors.bg_surface),
            bg_overlay: hex(&colors.bg_overlay),
            bg_selected: hex(&colors.bg_selected),
            fg_primary: hex(&colors.fg_primary),
            fg_secondary: hex(&colors.fg_secondary),
            fg_disabled: hex(&colors.fg_disabled),
            fg_contrast: hex(&colors.fg_contrast),
            border_default: hex(&colors.border_default),
            border_focused: hex(&colors.border_focused),
            border_error: hex(&colors.border_error),
            cursor: hex(&colors.cursor),
            scrollbar: hex(&colors.scrollbar),
            scrollbar_thumb: hex(&colors.scrollbar_thumb),
            background: hex(&colors.background.unwrap_or_else(|| colors.bg_base.clone())),
            foreground: hex(&colors.foreground.unwrap_or_else(|| colors.fg_primary.clone())),
            cursor_blink_rate_ms: data.styles.cursor_blink_rate_ms,
            scrollbar_width: data.styles.scrollbar_width,
            border_type: data.styles.border_type,
            title_alignment: data.styles.title_alignment,
            spinner_frames: data.animation.spinner_frames,
            progress_bar_filled: data.animation.progress_bar_filled,
            progress_bar_empty: data.animation.progress_bar_empty,
            progress_bar_half: data.animation.progress_bar_half,
        }
    }

    pub fn cursor_blink_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.cursor_blink_rate_ms.max(100))
    }

    pub fn border_type(&self) -> ratatui::widgets::BorderType {
        match self.border_type {
            BorderType::Rounded => ratatui::widgets::BorderType::Rounded,
            BorderType::Plain => ratatui::widgets::BorderType::Plain,
            BorderType::Double => ratatui::widgets::BorderType::Double,
            BorderType::Thick => ratatui::widgets::BorderType::Thick,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

pub fn builtin_theme_names() -> Vec<&'static str> {
    vec!["dark", "light", "midnight", "solar", "forest", "ocean"]
}

pub fn builtin_theme(name: &str) -> Option<Theme> {
    match name {
        "dark" => Some(Theme::dark()),
        "light" => Some(Theme::light()),
        "midnight" => Some(Theme::midnight()),
        "solar" => Some(Theme::solar()),
        "forest" => Some(Theme::forest()),
        "ocean" => Some(Theme::ocean()),
        _ => None,
    }
}

pub fn cycle_theme_name(current: &str) -> &'static str {
    let names = builtin_theme_names();
    let pos = names.iter().position(|n| *n == current).unwrap_or(0);
    names[(pos + 1) % names.len()]
}

pub fn theme_registry() -> HashMap<&'static str, Theme> {
    let mut map = HashMap::new();
    for name in builtin_theme_names() {
        if let Some(theme) = builtin_theme(name) {
            map.insert(name, theme);
        }
    }
    map
}

#[derive(Debug, Default, Deserialize)]
struct ThemeData {
    #[serde(default)]
    colors: ThemeColors,
    #[serde(default)]
    styles: ThemeStyles,
    #[serde(default)]
    animation: ThemeAnimation,
}

#[derive(Debug, Deserialize)]
struct ThemeColors {
    primary: String,
    secondary: String,
    success: String,
    warning: String,
    error: String,
    info: String,
    bg_base: String,
    bg_surface: String,
    bg_overlay: String,
    bg_selected: String,
    fg_primary: String,
    fg_secondary: String,
    fg_disabled: String,
    fg_contrast: String,
    border_default: String,
    border_focused: String,
    border_error: String,
    cursor: String,
    scrollbar: String,
    scrollbar_thumb: String,
    background: Option<String>,
    foreground: Option<String>,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "#61AFEF".to_string(),
            secondary: "#C678DD".to_string(),
            success: "#98C379".to_string(),
            warning: "#E5C07B".to_string(),
            error: "#E06C75".to_string(),
            info: "#56B6C2".to_string(),
            bg_base: "#282C34".to_string(),
            bg_surface: "#3E4451".to_string(),
            bg_overlay: "#4B5263".to_string(),
            bg_selected: "#2C313C".to_string(),
            fg_primary: "#ABB2BF".to_string(),
            fg_secondary: "#828997".to_string(),
            fg_disabled: "#5C6370".to_string(),
            fg_contrast: "#FFFFFF".to_string(),
            border_default: "#3E4451".to_string(),
            border_focused: "#61AFEF".to_string(),
            border_error: "#E06C75".to_string(),
            cursor: "#528BFF".to_string(),
            scrollbar: "#4B5263".to_string(),
            scrollbar_thumb: "#61AFEF".to_string(),
            background: None,
            foreground: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ThemeStyles {
    #[serde(default = "default_blink_rate")]
    cursor_blink_rate_ms: u64,
    #[serde(default = "default_scrollbar_width")]
    scrollbar_width: u8,
    #[serde(default)]
    border_type: BorderType,
    #[serde(default)]
    title_alignment: TitleAlignment,
}

impl Default for ThemeStyles {
    fn default() -> Self {
        Self {
            cursor_blink_rate_ms: default_blink_rate(),
            scrollbar_width: default_scrollbar_width(),
            border_type: BorderType::default(),
            title_alignment: TitleAlignment::default(),
        }
    }
}

fn default_blink_rate() -> u64 {
    530
}

fn default_scrollbar_width() -> u8 {
    1
}

#[derive(Debug, Default, Deserialize)]
struct ThemeAnimation {
    #[serde(default = "default_spinner_frames")]
    spinner_frames: Vec<String>,
    #[serde(default = "default_progress_filled")]
    progress_bar_filled: String,
    #[serde(default = "default_progress_empty")]
    progress_bar_empty: String,
    #[serde(default = "default_progress_half")]
    progress_bar_half: String,
}

fn default_spinner_frames() -> Vec<String> {
    vec!["-".into(), "\\".into(), "|".into(), "/".into()]
}

fn default_progress_filled() -> String {
    "=".into()
}

fn default_progress_empty() -> String {
    " ".into()
}

fn default_progress_half() -> String {
    ">".into()
}
