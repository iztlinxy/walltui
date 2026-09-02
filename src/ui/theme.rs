use std::str::FromStr;

use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BorderType {
    #[default]
    Rounded,
    Plain,
    Double,
    Thick,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TitleAlignment {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub bg_base: Color,
    pub bg_surface: Color,
    pub bg_overlay: Color,
    pub bg_selected: Color,
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_disabled: Color,
    pub fg_contrast: Color,
    pub border_default: Color,
    pub border_focused: Color,
    pub border_error: Color,
    pub cursor: Color,
    pub scrollbar: Color,
    pub scrollbar_thumb: Color,
    pub background: Color,
    pub foreground: Color,
    pub cursor_blink_rate_ms: u64,
    pub scrollbar_width: u8,
    pub border_type: BorderType,
    pub title_alignment: TitleAlignment,
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

macro_rules! theme_builder {
    ($name:ident, $($field:ident = $color:expr),* $(,)?) => {
        pub fn $name() -> Self {
            let mut t = Self::base();
            $(t.$field = hex($color);)*
            t
        }
    };
}

impl Theme {
    fn base() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::White,
            success: Color::White,
            warning: Color::White,
            error: Color::White,
            info: Color::White,
            bg_base: Color::Black,
            bg_surface: Color::Black,
            bg_overlay: Color::Black,
            bg_selected: Color::Black,
            fg_primary: Color::White,
            fg_secondary: Color::White,
            fg_disabled: Color::White,
            fg_contrast: Color::White,
            border_default: Color::White,
            border_focused: Color::White,
            border_error: Color::White,
            cursor: Color::White,
            scrollbar: Color::White,
            scrollbar_thumb: Color::White,
            background: Color::Black,
            foreground: Color::White,
            cursor_blink_rate_ms: 530,
            scrollbar_width: 1,
            border_type: BorderType::Rounded,
            title_alignment: TitleAlignment::Center,
        }
    }

    theme_builder! {
        dark,
        primary = "#00d2ff",
        secondary = "#ebcb8b",
        success = "#a3be8c",
        warning = "#ebcb8b",
        error = "#bf616a",
        info = "#81a1c1",
        bg_base = "#1f2430",
        bg_surface = "#3b4252",
        bg_overlay = "#4c566a",
        bg_selected = "#73bf32",
        fg_primary = "#e5e9f0",
        fg_secondary = "#81a1c1",
        fg_disabled = "#4c566a",
        fg_contrast = "#1f2430",
        border_default = "#3b4252",
        border_focused = "#00d2ff",
        border_error = "#bf616a",
        cursor = "#00d2ff",
        scrollbar = "#3b4252",
        scrollbar_thumb = "#00d2ff",
        background = "#1f2430",
        foreground = "#e5e9f0",
    }

    theme_builder! {
        light,
        primary = "#4078F2",
        secondary = "#A626A4",
        success = "#50A14F",
        warning = "#C18401",
        error = "#E45649",
        info = "#0184BC",
        bg_base = "#FAFAFA",
        bg_surface = "#EAEAEB",
        bg_overlay = "#D7D7D8",
        bg_selected = "#E5E5E6",
        fg_primary = "#383A42",
        fg_secondary = "#696C77",
        fg_disabled = "#A0A1A7",
        fg_contrast = "#FFFFFF",
        border_default = "#EAEAEB",
        border_focused = "#4078F2",
        border_error = "#E45649",
        cursor = "#4078F2",
        scrollbar = "#D7D7D8",
        scrollbar_thumb = "#4078F2",
        background = "#FAFAFA",
        foreground = "#383A42",
    }

    theme_builder! {
        midnight,
        primary = "#82AAFF",
        secondary = "#C792EA",
        success = "#C3E88D",
        warning = "#FFCB6B",
        error = "#F07178",
        info = "#89DDFF",
        bg_base = "#0F111A",
        bg_surface = "#1A1C2A",
        bg_overlay = "#24263A",
        bg_selected = "#1C1F2E",
        fg_primary = "#B0BEC5",
        fg_secondary = "#676E95",
        fg_disabled = "#464B5D",
        fg_contrast = "#FFFFFF",
        border_default = "#1A1C2A",
        border_focused = "#82AAFF",
        border_error = "#F07178",
        cursor = "#82AAFF",
        scrollbar = "#24263A",
        scrollbar_thumb = "#82AAFF",
        background = "#0F111A",
        foreground = "#B0BEC5",
    }

    theme_builder! {
        solar,
        primary = "#268BD2",
        secondary = "#D33682",
        success = "#859900",
        warning = "#B58900",
        error = "#DC322F",
        info = "#2AA198",
        bg_base = "#002B36",
        bg_surface = "#073642",
        bg_overlay = "#0A3A47",
        bg_selected = "#083C4A",
        fg_primary = "#839496",
        fg_secondary = "#586E75",
        fg_disabled = "#465A63",
        fg_contrast = "#FDF6E3",
        border_default = "#073642",
        border_focused = "#268BD2",
        border_error = "#DC322F",
        cursor = "#268BD2",
        scrollbar = "#0A3A47",
        scrollbar_thumb = "#268BD2",
        background = "#002B36",
        foreground = "#839496",
    }

    theme_builder! {
        forest,
        primary = "#7FBBB3",
        secondary = "#D699B6",
        success = "#A7C080",
        warning = "#DBBC7F",
        error = "#E67E80",
        info = "#83C092",
        bg_base = "#232A2E",
        bg_surface = "#2D353B",
        bg_overlay = "#3A454A",
        bg_selected = "#2B3338",
        fg_primary = "#D3C6AA",
        fg_secondary = "#7A8478",
        fg_disabled = "#596259",
        fg_contrast = "#232A2E",
        border_default = "#2D353B",
        border_focused = "#7FBBB3",
        border_error = "#E67E80",
        cursor = "#7FBBB3",
        scrollbar = "#3A454A",
        scrollbar_thumb = "#7FBBB3",
        background = "#232A2E",
        foreground = "#D3C6AA",
    }

    theme_builder! {
        ocean,
        primary = "#7AA2F7",
        secondary = "#BB9AF7",
        success = "#9ECE6A",
        warning = "#E0AF68",
        error = "#F7768E",
        info = "#73DACA",
        bg_base = "#1A1B26",
        bg_surface = "#24283B",
        bg_overlay = "#2E3248",
        bg_selected = "#232534",
        fg_primary = "#A9B1D6",
        fg_secondary = "#565F89",
        fg_disabled = "#414868",
        fg_contrast = "#FFFFFF",
        border_default = "#24283B",
        border_focused = "#7AA2F7",
        border_error = "#F7768E",
        cursor = "#7AA2F7",
        scrollbar = "#2E3248",
        scrollbar_thumb = "#7AA2F7",
        background = "#1A1B26",
        foreground = "#A9B1D6",
    }

    pub fn resolve(&self, key: StyleKey) -> Style {
        match key {
            StyleKey::Primary => Style::default().fg(self.primary),
            StyleKey::Secondary => Style::default().fg(self.secondary),
            StyleKey::Success => Style::default().fg(self.success),
            StyleKey::Warning => Style::default().fg(self.warning),
            StyleKey::Error => Style::default().fg(self.error),
            StyleKey::Info => Style::default().fg(self.info),
            StyleKey::Muted => Style::default().fg(self.fg_secondary),
            StyleKey::Highlight => Style::default()
                .fg(self.primary)
                .add_modifier(Modifier::BOLD),
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
                .fg(self.fg_contrast)
                .add_modifier(Modifier::BOLD),
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
            foreground: hex(&colors
                .foreground
                .unwrap_or_else(|| colors.fg_primary.clone())),
            cursor_blink_rate_ms: data.styles.cursor_blink_rate_ms,
            scrollbar_width: data.styles.scrollbar_width,
            border_type: data.styles.border_type,
            title_alignment: data.styles.title_alignment,
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

#[derive(Debug, Default, Deserialize)]
struct ThemeData {
    #[serde(default)]
    colors: ThemeColors,
    #[serde(default)]
    styles: ThemeStyles,
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
            primary: "#00d2ff".to_string(),
            secondary: "#ebcb8b".to_string(),
            success: "#a3be8c".to_string(),
            warning: "#ebcb8b".to_string(),
            error: "#bf616a".to_string(),
            info: "#81a1c1".to_string(),
            bg_base: "#1f2430".to_string(),
            bg_surface: "#3b4252".to_string(),
            bg_overlay: "#4c566a".to_string(),
            bg_selected: "#73bf32".to_string(),
            fg_primary: "#e5e9f0".to_string(),
            fg_secondary: "#81a1c1".to_string(),
            fg_disabled: "#4c566a".to_string(),
            fg_contrast: "#1f2430".to_string(),
            border_default: "#3b4252".to_string(),
            border_focused: "#00d2ff".to_string(),
            border_error: "#bf616a".to_string(),
            cursor: "#00d2ff".to_string(),
            scrollbar: "#3b4252".to_string(),
            scrollbar_thumb: "#00d2ff".to_string(),
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
