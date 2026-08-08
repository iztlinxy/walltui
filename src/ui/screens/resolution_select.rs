use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::ui::theme::Theme;

#[derive(Debug, Clone)]
pub enum CropMode {
    Scale,
    CropCenter,
    Fit,
}

#[derive(Debug, Clone)]
pub enum ResolutionOption {
    Original,
    Custom(u32, u32, CropMode),
    HD720(CropMode, bool),
    FHD1080(CropMode, bool),
    QHD1440(CropMode, bool),
    UHD2160(CropMode, bool),
    Ultrawide2560(CropMode, bool),
    Ultrawide3440(CropMode, bool),
    MacBook16(CropMode, bool),
    Phone1080(CropMode, bool),
}

impl ResolutionOption {
    pub fn presets() -> Vec<ResolutionOption> {
        vec![
            ResolutionOption::Original,
            ResolutionOption::HD720(CropMode::Scale, false),
            ResolutionOption::FHD1080(CropMode::Scale, false),
            ResolutionOption::QHD1440(CropMode::Scale, false),
            ResolutionOption::UHD2160(CropMode::Scale, false),
            ResolutionOption::Ultrawide2560(CropMode::CropCenter, false),
            ResolutionOption::Ultrawide3440(CropMode::CropCenter, false),
            ResolutionOption::MacBook16(CropMode::Fit, false),
            ResolutionOption::Phone1080(CropMode::CropCenter, false),
        ]
    }

    pub fn label(&self) -> String {
        match self {
            ResolutionOption::Original => "Original (no resize)".to_string(),
            ResolutionOption::HD720(mode, _) => format!("1280x720 (HD) - {}", mode_label(mode)),
            ResolutionOption::FHD1080(mode, _) => format!("1920x1080 (FHD) - {}", mode_label(mode)),
            ResolutionOption::QHD1440(mode, _) => format!("2560x1440 (QHD) - {}", mode_label(mode)),
            ResolutionOption::UHD2160(mode, _) => format!("3840x2160 (4K) - {}", mode_label(mode)),
            ResolutionOption::Ultrawide2560(mode, _) => format!("2560x1080 (UW) - {}", mode_label(mode)),
            ResolutionOption::Ultrawide3440(mode, _) => format!("3440x1440 (UW+) - {}", mode_label(mode)),
            ResolutionOption::MacBook16(mode, _) => format!("3072x1920 (MB 16\") - {}", mode_label(mode)),
            ResolutionOption::Phone1080(mode, _) => format!("1080x1920 (Phone) - {}", mode_label(mode)),
            ResolutionOption::Custom(w, h, mode) => format!("{w}x{h} - {}", mode_label(mode)),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ResolutionOption::Original => (0, 0),
            ResolutionOption::HD720(_, _) => (1280, 720),
            ResolutionOption::FHD1080(_, _) => (1920, 1080),
            ResolutionOption::QHD1440(_, _) => (2560, 1440),
            ResolutionOption::UHD2160(_, _) => (3840, 2160),
            ResolutionOption::Ultrawide2560(_, _) => (2560, 1080),
            ResolutionOption::Ultrawide3440(_, _) => (3440, 1440),
            ResolutionOption::MacBook16(_, _) => (3072, 1920),
            ResolutionOption::Phone1080(_, _) => (1080, 1920),
            ResolutionOption::Custom(w, h, _) => (*w, *h),
        }
    }

    pub fn crop_mode(&self) -> CropMode {
        match self {
            ResolutionOption::Original => CropMode::Scale,
            ResolutionOption::HD720(m, _) => m.clone(),
            ResolutionOption::FHD1080(m, _) => m.clone(),
            ResolutionOption::QHD1440(m, _) => m.clone(),
            ResolutionOption::UHD2160(m, _) => m.clone(),
            ResolutionOption::Ultrawide2560(m, _) => m.clone(),
            ResolutionOption::Ultrawide3440(m, _) => m.clone(),
            ResolutionOption::MacBook16(m, _) => m.clone(),
            ResolutionOption::Phone1080(m, _) => m.clone(),
            ResolutionOption::Custom(_, _, m) => m.clone(),
        }
    }

    pub fn cycle_crop_mode(&mut self) {
        match self {
            ResolutionOption::HD720(m, _) |
            ResolutionOption::FHD1080(m, _) |
            ResolutionOption::QHD1440(m, _) |
            ResolutionOption::UHD2160(m, _) |
            ResolutionOption::Ultrawide2560(m, _) |
            ResolutionOption::Ultrawide3440(m, _) |
            ResolutionOption::MacBook16(m, _) |
            ResolutionOption::Phone1080(m, _) => {
                *m = match m {
                    CropMode::Scale => CropMode::CropCenter,
                    CropMode::CropCenter => CropMode::Fit,
                    CropMode::Fit => CropMode::Scale,
                };
            }
            ResolutionOption::Custom(_, _, m) => {
                *m = match m {
                    CropMode::Scale => CropMode::CropCenter,
                    CropMode::CropCenter => CropMode::Fit,
                    CropMode::Fit => CropMode::Scale,
                };
            }
            _ => {}
        }
    }
}

fn mode_label(mode: &CropMode) -> &'static str {
    match mode {
        CropMode::Scale => "Stretch",
        CropMode::CropCenter => "Crop",
        CropMode::Fit => "Fit",
    }
}

pub struct ResolutionSelectScreen<'a> {
    pub options: &'a [ResolutionOption],
    pub selected: usize,
    pub wallpaper_title: &'a str,
    pub wallpaper_dims: Option<(u32, u32)>,
    pub theme: &'a Theme,
}

impl<'a> ResolutionSelectScreen<'a> {
    pub fn new(
        options: &'a [ResolutionOption],
        selected: usize,
        wallpaper_title: &'a str,
        wallpaper_dims: Option<(u32, u32)>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            options,
            selected,
            wallpaper_title,
            wallpaper_dims,
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let dims_str = match self.wallpaper_dims {
            Some((w, h)) => format!("{w}x{h}"),
            None => "unknown".to_string(),
        };
        let header = Paragraph::new(Line::from(vec![
            Span::styled(" Select Resolution ", Style::default().fg(self.theme.primary).bold()),
            Span::raw(format!(" | {} [{}]", self.wallpaper_title, dims_str)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.primary)),
        );
        frame.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = self
            .options
            .iter()
            .map(|opt| {
                let line = Line::from(Span::raw(opt.label()));
                ListItem::new(line)
            })
            .collect();

        let mut state = ListState::default().with_selected(Some(self.selected));
        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Resolutions ")
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

        frame.render_stateful_widget(list, chunks[1], &mut state);

        let help_shortcuts: Vec<(&str, &str)> = vec![
            ("↑/↓", "Navigate"),
            ("Enter", "Download"),
            ("c", "Cycle crop mode"),
            ("Esc", "Back"),
        ];

        use crate::ui::widgets::help_bar::HelpBar;
        HelpBar::new(&help_shortcuts, self.theme).render(frame, chunks[2]);
    }
}
