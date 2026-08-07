use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tracing::{info, warn};

use walltui::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _log_guard = init_logging()?;
    init_panic_hook();

    info!("WallTUI starting");

    let mut terminal = ratatui::init();
    let result = run(&mut terminal).await;
    ratatui::restore();
    info!("WallTUI exiting");
    result
}

fn init_logging() -> color_eyre::Result<tracing_appender::non_blocking::WorkerGuard> {
    let log_dir = std::env::temp_dir();
    let file_appender = tracing_appender::rolling::never(&log_dir, "walltui.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    Ok(guard)
}

fn init_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        default_hook(info);
    }));
}

async fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = App::new();
    let mut tick_rate = tokio::time::interval(Duration::from_millis(250));

    while !app.should_quit {
        terminal.draw(|frame| app.draw(frame))?;

        tokio::select! {
            _ = tick_rate.tick() => {
                app.tick();
            }
            result = tokio::task::spawn_blocking(|| event::read()) => {
                let event = result??;
                handle_event(&mut app, event)?;
            }
        }
    }

    Ok(())
}

fn handle_event(app: &mut App, event: Event) -> color_eyre::Result<()> {
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
                _ => warn!("unhandled key: {:?}", key.code),
            }
        }
    }
    Ok(())
}
