use ratatui::Frame;

#[derive(Default)]
pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget("WallTUI — press q to quit", frame.area());
    }
}
