use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub background: Color,
    pub foreground: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            background: Color::Black,
            foreground: Color::White,
        }
    }

    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            background: Color::White,
            foreground: Color::Black,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
