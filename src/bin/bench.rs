use std::time::{Duration, Instant};

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui_image::picker::Picker;
use walltui::app::App;
use walltui::ui::screens::Screen;

#[global_allocator]
static PEAK: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

fn timeit<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    rt.block_on(run());
}

async fn run() {
    let startup = timeit(|| {
        let _ = App::new(Picker::halfblocks());
    });
    println!("startup_ms: {}", startup.as_secs_f64() * 1000.0);

    let mut app = App::new(Picker::halfblocks());
    app.tick().await;

    let nav = timeit(|| app.navigate_to(Screen::Search));
    println!("navigate_to_search_ms: {}", nav.as_secs_f64() * 1000.0);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("terminal");
    let draw = timeit(|| {
        terminal.draw(|frame| app.draw(frame)).unwrap();
    });
    println!("draw_search_ms: {}", draw.as_secs_f64() * 1000.0);

    app.navigate_to(Screen::Gallery);
    let draw_gallery = timeit(|| {
        terminal.draw(|frame| app.draw(frame)).unwrap();
    });
    println!("draw_gallery_ms: {}", draw_gallery.as_secs_f64() * 1000.0);

    println!("peak_mem_kb: {:.1}", PEAK.peak_usage_as_kb());
}
