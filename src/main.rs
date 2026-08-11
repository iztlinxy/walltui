use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;
use tracing::{info, warn};

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

    if std::env::var("WALLTUI_HEADLESS").is_ok() {
        let start = Instant::now();
        let mut app = App::new();
        let _ = app.tick().await;
        app.quit();
        info!("Headless startup time: {:?}", start.elapsed());
        return Ok(());
    }

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
    let mut dirty = true;

    while !app.should_quit {
        if dirty {
            let frame_start = Instant::now();
            terminal.draw(|frame| app.draw(frame))?;
            let frame_time = frame_start.elapsed();
            if frame_time > Duration::from_millis(33) {
                warn!("Slow frame: {:?}", frame_time);
            }
            dirty = false;
        }

        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            if handle_event(&mut app, event).await? {
                dirty = true;
            }
        }

        if app.tick().await {
            dirty = true;
        }
    }

    Ok(())
}

async fn handle_event(app: &mut App, event: Event) -> color_eyre::Result<bool> {
    if let Event::Key(key) = event
        && key.kind == KeyEventKind::Press
    {
        // Global shortcuts.
        if key.code == KeyCode::Char('t')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.modifiers.contains(KeyModifiers::SHIFT)
        {
            app.cycle_theme();
            return Ok(true);
        }

        match key.code {
            KeyCode::Char('q') => { app.quit(); return Ok(true); }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => { app.quit(); return Ok(true); }
            _ => return handle_screen_event(app, key.code, key.modifiers).await,
        }
    }
    Ok(false)
}

async fn handle_theme_select_event(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.theme_selected_index > 0 {
                app.theme_selected_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let themes = walltui::ui::theme::builtin_theme_names();
            if app.theme_selected_index + 1 < themes.len() {
                app.theme_selected_index += 1;
            }
        }
        KeyCode::Enter => {
            let themes = walltui::ui::theme::builtin_theme_names();
            if let Some(name) = themes.get(app.theme_selected_index) {
                app.apply_theme(name);
            }
        }
        _ => {}
    }
    true
}

async fn handle_screen_event(
    app: &mut App,
    key: KeyCode,
    modifiers: KeyModifiers,
) -> color_eyre::Result<bool> {
    let dirty = match app.current_screen {
        Screen::Splash => handle_splash_event(app, key),
        Screen::Search => handle_search_event(app, key, modifiers).await,
        Screen::Detail => handle_detail_event(app, key).await,
        Screen::Download => handle_download_event(app, key).await,
        Screen::Config => handle_config_event(app, key, modifiers),
        Screen::Gallery => handle_gallery_event(app, key, modifiers).await,
        Screen::ResolutionSelect => handle_resolution_select_event(app, key).await,
        Screen::ThemeSelect => handle_theme_select_event(app, key).await,
        Screen::YtDlpInput => handle_ytdlp_input_event(app, key, modifiers).await,
        Screen::YtDlpPreview => handle_ytdlp_preview_event(app, key, modifiers).await,
    };
    Ok(dirty)
}

fn handle_splash_event(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('s') => app.navigate_to(Screen::Search),
        KeyCode::Char('g') => {
            app.load_gallery();
            app.navigate_to(Screen::Gallery);
        }
        KeyCode::Char('c') => app.navigate_to(Screen::Config),
        _ => {}
    }
    true
}

async fn paste_from_clipboard() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let output = tokio::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "Get-Clipboard"])
            .output()
            .await
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() { None } else { Some(text) }
    }
    #[cfg(target_os = "linux")]
    {
        let output = tokio::process::Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()
            .await
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() { None } else { Some(text) }
    }
    #[cfg(target_os = "macos")]
    {
        let output = tokio::process::Command::new("pbpaste")
            .output()
            .await
            .ok()?;
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() { None } else { Some(text) }
    }
}

async fn handle_search_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> bool {
    if app.search_focused {
        if key == KeyCode::Char('v') && modifiers.contains(KeyModifiers::CONTROL) {
            if let Some(text) = paste_from_clipboard().await {
                app.search_query = text.clone();
                app.cursor_pos = text.len();
            }
            return true;
        }

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
            KeyCode::Char('5') => {
                app.reset_ytdlp();
                app.navigate_to(Screen::YtDlpInput);
            }
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
    true
}

async fn handle_detail_event(app: &mut App, key: KeyCode) -> bool {
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
    true
}

async fn handle_download_event(app: &mut App, key: KeyCode) -> bool {
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
    true
}

fn handle_config_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> bool {
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
    true
}

async fn handle_gallery_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> bool {
    if app.confirm_dialog.is_some() {
        match key {
            KeyCode::Char('y') | KeyCode::Char('Y') => app.confirm_gallery_delete(),
            _ => app.confirm_dialog = None,
        }
        return true;
    }

    if app.gallery_editing {
        match key {
            KeyCode::Esc => app.cancel_gallery_rename(),
            KeyCode::Enter => app.confirm_gallery_rename(),
            KeyCode::Backspace => app.handle_gallery_edit_backspace(),
            KeyCode::Left => app.handle_gallery_edit_left(),
            KeyCode::Right => app.handle_gallery_edit_right(),
            KeyCode::Char(c) => app.handle_gallery_edit_input(c),
            _ => {}
        }
        return true;
    }

    match key {
        KeyCode::Esc => app.go_back(),
        KeyCode::Up | KeyCode::Char('k') => app.move_gallery_selection_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_gallery_selection_down(),
        KeyCode::Char('R') => app.load_gallery(),
        KeyCode::Char('r') => app.start_gallery_rename(),
        KeyCode::Char('d') => app.prompt_delete_gallery_selected(),
        KeyCode::Enter => {
            if let Some(wallpaper) = app.gallery_wallpapers.get(app.gallery_selected_index) {
                let _ = open::that(&wallpaper.url);
            }
        }
        KeyCode::Char('w') => {
            if let Some(wallpaper) = app.gallery_wallpapers.get(app.gallery_selected_index) {
                if wallpaper.is_video() {
                    let path = std::path::Path::new(&wallpaper.url);
                    match walltui::platform::set_video_wallpaper(path) {
                        Ok(()) => app.toast_manager.show("Video wallpaper set!"),
                        Err(e) => app.toast_manager.show(format!("Failed to set video wallpaper: {e}")),
                    }
                } else {
                    let path = std::path::Path::new(&wallpaper.url);
                    match walltui::platform::set_wallpaper(path) {
                        Ok(()) => app.toast_manager.show("Wallpaper set successfully"),
                        Err(e) => app.toast_manager.show(format!("Failed to set wallpaper: {e}")),
                    }
                }
            }
        }
        KeyCode::Char('W') if modifiers.contains(KeyModifiers::SHIFT) => {
            match walltui::platform::stop_video_wallpaper() {
                Ok(()) => app.toast_manager.show("Video wallpaper stopped"),
                Err(e) => app.toast_manager.show(format!("Failed to stop video wallpaper: {e}")),
            }
        }
        _ => {}
    }
    true
}

async fn handle_resolution_select_event(app: &mut App, key: KeyCode) -> bool {
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
    true
}

async fn handle_ytdlp_input_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> bool {
    if key == KeyCode::Char('v') && modifiers.contains(KeyModifiers::CONTROL) {
        if let Some(text) = paste_from_clipboard().await {
            app.ytdlp_url = text.clone();
            app.ytdlp_cursor_pos = text.len();
        }
        return true;
    }

    match key {
        KeyCode::Esc => {
            app.reset_ytdlp();
            app.go_back();
        }
        KeyCode::Enter => {
            app.fetch_ytdlp_metadata().await;
        }
        KeyCode::Backspace => {
            app.handle_ytdlp_backspace();
        }
        KeyCode::Left => {
            app.handle_ytdlp_left();
        }
        KeyCode::Right => {
            app.handle_ytdlp_right();
        }
        KeyCode::Char(c) => {
            app.handle_ytdlp_input(c);
        }
        _ => {}
    }
    true
}

async fn handle_ytdlp_preview_event(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> bool {
    match key {
        KeyCode::Esc => {
            app.reset_ytdlp();
            app.go_back();
        }
        KeyCode::Char('d') => {
            app.download_ytdlp_clip().await;
        }
        KeyCode::Char('x') => {
            app.ytdlp_downloading = false;
            app.ytdlp_progress_rx = None;
        }
        KeyCode::Left => {
            app.adjust_clip_start(-1);
        }
        KeyCode::Right => {
            app.adjust_clip_start(1);
        }
        KeyCode::Char('L') if modifiers.contains(KeyModifiers::SHIFT) => {
            app.adjust_clip_end(-1);
        }
        KeyCode::Char('R') if modifiers.contains(KeyModifiers::SHIFT) => {
            app.adjust_clip_end(1);
        }
        _ => {}
    }
    true
}
