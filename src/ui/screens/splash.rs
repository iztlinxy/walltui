use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::ui::theme::Theme;

pub struct SplashScreen<'a> {
    theme: &'a Theme,
    recent_downloads: &'a [String],
    selected_index: usize,
}

impl<'a> SplashScreen<'a> {
    pub fn new(theme: &'a Theme, recent_downloads: &'a [String], selected_index: usize) -> Self {
        Self {
            theme,
            recent_downloads,
            selected_index,
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

        let body = chunks[0];

        // Build centered content: logo + subtitle + menu + optional recent downloads.
        let ascii_logo: Vec<Line> = vec![
            Line::from("██╗    ██╗ █████╗ ██╗     ██╗   ████████╗██╗   ██╗██╗").centered(),
            Line::from("██║    ██║██╔══██╗██║     ██║   ╚══██╔══╝██║   ██║██║").centered(),
            Line::from("██║ █╗ ██║███████║██║     ██║      ██║   ██║   ██║██║").centered(),
            Line::from("██║███╗██║██╔══██║██║     ██║      ██║   ██║   ██║██║").centered(),
            Line::from("╚███╔███╔╝██║  ██║███████╗███████╗ ██║   ╚██████╔╝██║").centered(),
            Line::from(" ╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚══════╝ ╚═╝    ╚═════╝ ╚═╝").centered(),
        ];

        let mut content_lines = ascii_logo;
        content_lines.push(Line::from(""));
        content_lines.push(
            Line::from(Span::styled(
                "Terminal Wallpaper Manager",
                Style::default().fg(self.theme.fg_primary).bold(),
            ))
            .centered(),
        );
        content_lines.push(Line::from(""));

        // Vertical menu rendered as a compact centered box.
        let menu_items = vec![
            ("s", "Search"),
            ("g", "Gallery"),
            ("c", "Settings"),
            ("q", "Quit"),
        ];

        let menu_width = menu_items
            .iter()
            .map(|(k, l)| format!("[{}] {}", k, l).len())
            .max()
            .unwrap_or(12)
            .max(12)
            + 4;

        let menu_lines: Vec<Line> = menu_items
            .iter()
            .enumerate()
            .map(|(i, (key, label))| {
                let selected = i == self.selected_index;
                let base = if selected {
                    Style::default().bg(self.theme.primary).fg(self.theme.bg_base)
                } else {
                    Style::default().fg(self.theme.fg_primary)
                };
                let label_text = format!("[{}] {}", key, label);
                Line::from(vec![
                    Span::styled(" ", base),
                    Span::styled(
                        format!("[{key}]"),
                        if selected {
                            base.add_modifier(ratatui::style::Modifier::BOLD)
                        } else {
                            base.fg(self.theme.secondary)
                        },
                    ),
                    Span::styled(format!(" {label}"), base),
                    Span::styled(
                        " ".repeat(menu_width - 2 - label_text.len()),
                        base,
                    ),
                ])
            })
            .collect();

        let menu_paragraph = Paragraph::new(menu_lines.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.border_focused)),
            )
            .alignment(Alignment::Left);

        // Render logo/subtitle into a temporary paragraph to know height, then menu.
        let logo_height = content_lines.len() as u16;
        let menu_height = menu_lines.len() as u16 + 2; // borders
        let recent_height = if self.recent_downloads.is_empty() {
            0
        } else {
            2 + self.recent_downloads.len().min(5) as u16
        };
        let total_height = logo_height + menu_height + recent_height;

        let available_height = body.height.saturating_sub(2);
        let start_y = body.y + available_height.saturating_sub(total_height) / 2;

        let logo_area = Rect {
            x: body.x,
            y: start_y,
            width: body.width,
            height: logo_height,
        };
        let logo = Paragraph::new(content_lines).alignment(Alignment::Center);
        frame.render_widget(logo, logo_area);

        let menu_x = body.x + (body.width.saturating_sub(menu_width as u16 + 2)) / 2;
        let menu_area = Rect {
            x: menu_x,
            y: logo_area.y + logo_area.height,
            width: menu_width as u16 + 2,
            height: menu_height,
        };
        frame.render_widget(Clear, menu_area);
        frame.render_widget(menu_paragraph, menu_area);

        if !self.recent_downloads.is_empty() {
            let recent_y = menu_area.y + menu_area.height + 1;
            let mut recent_lines: Vec<Line> = vec![
                Line::from(Span::styled(
                    "Recent downloads",
                    Style::default().fg(self.theme.fg_primary).bold(),
                ))
                .centered(),
            ];
            for title in self.recent_downloads.iter().take(5) {
                recent_lines.push(
                    Line::from(Span::styled(
                        format!("• {title}"),
                        Style::default().fg(self.theme.fg_secondary),
                    ))
                    .centered(),
                );
            }
            let recent_area = Rect {
                x: body.x,
                y: recent_y,
                width: body.width,
                height: recent_lines.len() as u16,
            };
            frame.render_widget(
                Paragraph::new(recent_lines).alignment(Alignment::Center),
                recent_area,
            );
        }

        // Bottom navigation bar.
        let nav_spans = vec![
            Span::styled("Navigation: ", Style::default().fg(self.theme.fg_secondary).bold()),
            Span::styled("[↑/↓]", Style::default().fg(self.theme.secondary)),
            Span::styled(" (or ", Style::default().fg(self.theme.fg_secondary)),
            Span::styled("[j/k]", Style::default().fg(self.theme.secondary)),
            Span::styled(") Move ", Style::default().fg(self.theme.fg_secondary)),
            Span::styled("• ", Style::default().fg(self.theme.fg_secondary)),
            Span::styled("[Enter]", Style::default().fg(self.theme.secondary)),
            Span::styled(" Select ", Style::default().fg(self.theme.fg_secondary)),
            Span::styled("• ", Style::default().fg(self.theme.fg_secondary)),
            Span::styled("[s/g/c/q]", Style::default().fg(self.theme.secondary)),
            Span::styled(" Quick Action", Style::default().fg(self.theme.fg_secondary)),
        ];
        let help = Paragraph::new(Line::from(nav_spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(self.theme.border_default)),
            );
        frame.render_widget(help, chunks[1]);
    }
}
