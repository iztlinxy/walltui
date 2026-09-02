use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::ui::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropMode {
    Scale,
    CropCenter,
    Fit,
}

#[derive(Debug, Clone)]
pub enum ResolutionOption {
    Original,
    Custom(u32, u32, CropMode),
    HD720(CropMode),
    FHD1080(CropMode),
    QHD1440(CropMode),
    UHD2160(CropMode),
    Ultrawide2560(CropMode),
    Ultrawide3440(CropMode),
    MacBook16(CropMode),
    Phone1080(CropMode),
}

impl ResolutionOption {
    pub fn presets() -> Vec<ResolutionOption> {
        vec![
            ResolutionOption::Original,
            ResolutionOption::HD720(CropMode::Scale),
            ResolutionOption::FHD1080(CropMode::Scale),
            ResolutionOption::QHD1440(CropMode::Scale),
            ResolutionOption::UHD2160(CropMode::Scale),
            ResolutionOption::Ultrawide2560(CropMode::CropCenter),
            ResolutionOption::Ultrawide3440(CropMode::CropCenter),
            ResolutionOption::MacBook16(CropMode::Fit),
            ResolutionOption::Phone1080(CropMode::CropCenter),
        ]
    }

    pub fn label(&self) -> String {
        let (name, dims, mode) = match self {
            ResolutionOption::Original => return "Original (no resize)".to_string(),
            ResolutionOption::HD720(m) => ("HD", (1280, 720), m),
            ResolutionOption::FHD1080(m) => ("FHD", (1920, 1080), m),
            ResolutionOption::QHD1440(m) => ("QHD", (2560, 1440), m),
            ResolutionOption::UHD2160(m) => ("4K", (3840, 2160), m),
            ResolutionOption::Ultrawide2560(m) => ("UW", (2560, 1080), m),
            ResolutionOption::Ultrawide3440(m) => ("UW+", (3440, 1440), m),
            ResolutionOption::MacBook16(m) => ("MB 16\"", (3072, 1920), m),
            ResolutionOption::Phone1080(m) => ("Phone", (1080, 1920), m),
            ResolutionOption::Custom(w, h, m) => {
                return format!("{w}x{h} - {}", mode_label(m));
            }
        };
        format!("{}x{} ({}) - {}", dims.0, dims.1, name, mode_label(mode))
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ResolutionOption::Original => (0, 0),
            ResolutionOption::HD720(_) => (1280, 720),
            ResolutionOption::FHD1080(_) => (1920, 1080),
            ResolutionOption::QHD1440(_) => (2560, 1440),
            ResolutionOption::UHD2160(_) => (3840, 2160),
            ResolutionOption::Ultrawide2560(_) => (2560, 1080),
            ResolutionOption::Ultrawide3440(_) => (3440, 1440),
            ResolutionOption::MacBook16(_) => (3072, 1920),
            ResolutionOption::Phone1080(_) => (1080, 1920),
            ResolutionOption::Custom(w, h, _) => (*w, *h),
        }
    }

    pub fn crop_mode(&self) -> CropMode {
        match self {
            ResolutionOption::Original => CropMode::Scale,
            ResolutionOption::HD720(m)
            | ResolutionOption::FHD1080(m)
            | ResolutionOption::QHD1440(m)
            | ResolutionOption::UHD2160(m)
            | ResolutionOption::Ultrawide2560(m)
            | ResolutionOption::Ultrawide3440(m)
            | ResolutionOption::MacBook16(m)
            | ResolutionOption::Phone1080(m)
            | ResolutionOption::Custom(_, _, m) => *m,
        }
    }

    pub fn cycle_crop_mode(&mut self) {
        let mode = match self {
            ResolutionOption::Original => return,
            ResolutionOption::HD720(m)
            | ResolutionOption::FHD1080(m)
            | ResolutionOption::QHD1440(m)
            | ResolutionOption::UHD2160(m)
            | ResolutionOption::Ultrawide2560(m)
            | ResolutionOption::Ultrawide3440(m)
            | ResolutionOption::MacBook16(m)
            | ResolutionOption::Phone1080(m)
            | ResolutionOption::Custom(_, _, m) => m,
        };
        *mode = match mode {
            CropMode::Scale => CropMode::CropCenter,
            CropMode::CropCenter => CropMode::Fit,
            CropMode::Fit => CropMode::Scale,
        };
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
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        let dims_str = match self.wallpaper_dims {
            Some((w, h)) => format!("{w}x{h}"),
            None => "unknown".to_string(),
        };
        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                " Select Resolution ",
                Style::default().fg(self.theme.primary).bold(),
            ),
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
