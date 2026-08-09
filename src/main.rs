use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;
use tracing::info;

use walltui::app::App;
use walltui::core::models::Provider;
use walltui::ui::screens::Screen;
use walltui::ui::screens::resolution_select::ResolutionOption;

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

    while !app.should_quit {
        terminal.draw(|frame| app.draw(frame))?;

        if event::poll(Duration::from_millis(50))? {
            let event = event::read()?;
            handle_event(&mut app, event).await?;
        }

        app.tick().await;
    }

    Ok(())
}

async fn handle_event(app: &mut App, event: Event) -> color_eyre::Result<()> {
    if let Event::Key(key) = event
        && key.kind == KeyEventKind::Press
    {
        match key.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
            _ => handle_screen_event(app, key.code, key.modifiers).await?,
        }
    }
    Ok(())
}

async fn handle_screen_event(
    app: &mut App,
    key: KeyCode,
    modifiers: KeyModifiers,
) -> color_eyre::Result<()> {
    match app.current_screen {
        Screen::Splash => handle_splash_event(app, key),
        Screen::Search => handle_search_event(app, key, modifiers).await,
        Screen::Detail => handle_detail_event(app, key).await,
        Screen::Download => handle_download_event(app, key).await,
        Screen::Config => handle_config_event(app, key, modifiers),
        Screen::Gallery => handle_gallery_event(app, key).await,
        Screen::ResolutionSelect => handle_resolution_select_event(app, key).await,
    }
    Ok(())
}

fn handle_splash_event(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('s') => app.navigate_to(Screen::Search),
        KeyCode::Char('g') => {
            app.load_gallery();
            app.navigate_to(Screen::Gallery);
        }
        KeyCode::Char('c') => app.navigate_to(Screen::Config),
        _ => {}
    }
}

async fn handle_search_event(app: &mut App, key: KeyCode, _modifiers: KeyModifiers) {
    if app.search_focused {
        match key {
            KeyCode::Esc => {
                app.search_focused = false;
            }
            KeyCode::Enter => {
                app.search_focused = false;
                app.reset_search_page();
                app.execute_search().await;
            }
            KeyCode::Backspace => {
                app.handle_search_backspace();
            }
            KeyCode::Left => {
                app.handle_search_left();
            }
            KeyCode::Right => {
                app.handle_search_right();
            }
            KeyCode::Char(c) => {
                app.handle_search_input(c);
            }
            _ => {}
        }
    } else {
        match key {
            KeyCode::Esc => app.go_back(),
            KeyCode::Char('/') => {
                app.search_focused = true;
            }
            KeyCode::Char('1') => app.switch_provider(Provider::Wallhaven),
            KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
            KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
            KeyCode::Enter => {
                if !app.wallpapers.is_empty() {
                    app.navigate_to(Screen::Detail);
                }
            }
            KeyCode::Char('d') => {
                app.navigate_to(Screen::Download);
            }
            KeyCode::Char('n') if !app.search_query.is_empty() => {
                app.search_next_page().await;
            }
            KeyCode::Char('p') if !app.search_query.is_empty() => {
                app.search_prev_page().await;
            }
            _ => {}
        }
    }
}

async fn handle_detail_event(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Char('d') => {
            if let Some(wallpaper) = app.wallpapers.get(app.selected_index).cloned() {
                app.pending_download_wallpaper = Some(wallpaper);
                app.resolution_selected_index = 0;
                app.navigate_to(Screen::ResolutionSelect);
            }
        }
        KeyCode::Char('o') => {
            if let Some(wallpaper) = app.wallpapers.get(app.selected_index) {
                let url = wallpaper.web_url.as_deref().unwrap_or(&wallpaper.url);
                let _ = open::that(url);
            }
        }
        _ => {}
    }
}

async fn handle_download_event(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_index > 0 {
                app.selected_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_index + 1 < app.download_tasks.len() {
                app.selected_index += 1;
            }
        }
        KeyCode::Char('x') => {
            app.cancel_download(app.selected_index).await;
        }
        KeyCode::Char('r') => {
            app.retry_download(app.selected_index).await;
        }
        KeyCode::Char('c') => {
            app.clear_completed_downloads().await;
        }
        _ => {}
    }
}

fn handle_config_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    if app.config_editing {
        match key {
            KeyCode::Esc => app.cancel_config_edit(),
            KeyCode::Enter => app.confirm_config_edit(),
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                app.confirm_config_edit();
            }
            KeyCode::Backspace => app.handle_config_backspace(),
            KeyCode::Left => app.handle_config_left(),
            KeyCode::Right => app.handle_config_right(),
            KeyCode::Char(c) => app.handle_config_input(c),
            _ => {}
        }
    } else {
        match key {
            KeyCode::Esc => app.go_back(),
            KeyCode::Enter => app.start_config_edit(),
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                app.start_config_edit();
            }
            KeyCode::Up | KeyCode::Char('k') => app.move_config_selection_up(),
            KeyCode::Down | KeyCode::Char('j') => app.move_config_selection_down(),
            _ => {}
        }
    }
}

async fn handle_gallery_event(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => app.move_gallery_selection_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_gallery_selection_down(),
        KeyCode::Char('r') => app.load_gallery(),
        KeyCode::Enter => {
            if let Some(wallpaper) = app.gallery_wallpapers.get(app.gallery_selected_index) {
                let _ = open::that(&wallpaper.url);
            }
        }
        KeyCode::Char('w') => {
            if let Some(wallpaper) = app.gallery_wallpapers.get(app.gallery_selected_index) {
                let path = std::path::Path::new(&wallpaper.url);
                match walltui::platform::set_wallpaper(path) {
                    Ok(()) => app.notification = Some("Wallpaper set successfully".to_string()),
                    Err(e) => app.notification = Some(format!("Failed to set wallpaper: {e}")),
                }
            }
        }
        _ => {}
    }
}

async fn handle_resolution_select_event(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.resolution_selected_index > 0 {
                app.resolution_selected_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.resolution_selected_index + 1 < app.resolution_options.len() {
                app.resolution_selected_index += 1;
            }
        }
        KeyCode::Char('c') => {
            if let Some(opt) = app.resolution_options.get_mut(app.resolution_selected_index) {
                opt.cycle_crop_mode();
            }
        }
        KeyCode::Enter => {
            if let Some(wallpaper) = app.pending_download_wallpaper.clone() {
                let resolution = app.resolution_options.get(app.resolution_selected_index).cloned();
                let resolution = match resolution {
                    Some(ResolutionOption::Original) => None,
                    Some(opt) => Some(opt),
                    None => None,
                };
                app.enqueue_download(&wallpaper, resolution);
                app.pending_download_wallpaper = None;
                app.navigate_to(Screen::Download);
            }
        }
        _ => {}
    }
}
