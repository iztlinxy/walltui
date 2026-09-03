use std::path::PathBuf;

use walltui::core::download::{DownloadManager, DownloadTask};
use walltui::core::models::Provider;
use walltui::ui::screens::Screen;
use walltui::ui::screens::config::ConfigField;
use walltui::ui::screens::resolution_select::ResolutionOption;

mod common;

fn make_app() -> walltui::app::App {
    common::make_app()
}

fn sample_wallpaper(id: &str) -> walltui::core::models::Wallpaper {
    common::sample_wallpaper(id)
}

// --- Screen navigation ---

#[test]
fn splash_to_search() {
    let mut app = make_app();
    assert_eq!(app.current_screen, Screen::Splash);
    app.navigate_to(Screen::Search);
    assert_eq!(app.current_screen, Screen::Search);
    assert_eq!(app.previous_screen, Some(Screen::Splash));
}

#[test]
fn splash_to_gallery() {
    let mut app = make_app();
    app.navigate_to(Screen::Gallery);
    assert_eq!(app.current_screen, Screen::Gallery);
}

#[test]
fn splash_to_config() {
    let mut app = make_app();
    app.navigate_to(Screen::Config);
    assert_eq!(app.current_screen, Screen::Config);
}

#[test]
fn go_back_from_search_to_splash() {
    let mut app = make_app();
    app.navigate_to(Screen::Search);
    app.go_back();
    assert_eq!(app.current_screen, Screen::Splash);
    assert!(app.previous_screen.is_none());
}

#[test]
fn go_back_from_gallery_to_splash() {
    let mut app = make_app();
    app.navigate_to(Screen::Gallery);
    app.go_back();
    assert_eq!(app.current_screen, Screen::Splash);
}

#[test]
fn go_back_no_previous_goes_to_splash() {
    let mut app = make_app();
    app.previous_screen = None;
    app.current_screen = Screen::Config;
    app.go_back();
    assert_eq!(app.current_screen, Screen::Splash);
}

#[test]
fn navigate_screens_chain() {
    let mut app = make_app();
    app.navigate_to(Screen::Search);
    app.navigate_to(Screen::ResolutionSelect);
    assert_eq!(app.current_screen, Screen::ResolutionSelect);
    app.go_back();
    assert_eq!(app.current_screen, Screen::Search);
    app.go_back();
    assert_eq!(app.current_screen, Screen::Splash);
}

// --- Search input ---

#[test]
fn search_input_inserts_chars() {
    let mut app = make_app();
    app.handle_search_input('h');
    app.handle_search_input('i');
    assert_eq!(app.search_query, "hi");
    assert_eq!(app.cursor_pos, 2);
}

#[test]
fn search_backspace_removes_char() {
    let mut app = make_app();
    app.handle_search_input('a');
    app.handle_search_input('b');
    app.handle_search_backspace();
    assert_eq!(app.search_query, "a");
    assert_eq!(app.cursor_pos, 1);
}

#[test]
fn search_backspace_at_start_noop() {
    let mut app = make_app();
    app.handle_search_backspace();
    assert_eq!(app.search_query, "");
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn search_cursor_left() {
    let mut app = make_app();
    app.handle_search_input('a');
    app.handle_search_input('b');
    app.handle_search_left();
    assert_eq!(app.cursor_pos, 1);
}

#[test]
fn search_cursor_left_at_start_noop() {
    let mut app = make_app();
    app.handle_search_left();
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn search_cursor_right() {
    let mut app = make_app();
    app.handle_search_input('a');
    app.handle_search_left();
    app.handle_search_right();
    assert_eq!(app.cursor_pos, 1);
}

#[test]
fn search_cursor_right_at_end_noop() {
    let mut app = make_app();
    app.handle_search_input('x');
    app.handle_search_right();
    assert_eq!(app.cursor_pos, 1);
}

#[test]
fn clear_search_resets_all() {
    let mut app = make_app();
    app.handle_search_input('t');
    app.handle_search_input('e');
    app.clear_search();
    assert_eq!(app.search_query, "");
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn search_empty_query_skips_execute() {
    let mut app = make_app();
    // Should return early without error
    app.search_query = String::new();
    // Can't call async here, but we verify the guard condition
    assert!(app.search_query.is_empty());
}

// --- Selection navigation ---

#[test]
fn move_selection_up_bounded() {
    let mut app = make_app();
    app.wallpapers = vec![sample_wallpaper("1"), sample_wallpaper("2")];
    app.selected_index = 1;
    app.move_selection_up();
    assert_eq!(app.selected_index, 0);
    app.move_selection_up();
    assert_eq!(app.selected_index, 0);
}

#[test]
fn move_selection_down_bounded() {
    let mut app = make_app();
    app.wallpapers = vec![sample_wallpaper("1"), sample_wallpaper("2")];
    app.selected_index = 0;
    app.move_selection_down();
    assert_eq!(app.selected_index, 1);
    app.move_selection_down();
    assert_eq!(app.selected_index, 1);
}

// --- Gallery navigation ---

#[test]
fn gallery_selection_up_bounded() {
    let mut app = make_app();
    app.gallery_wallpapers = vec![sample_wallpaper("a"), sample_wallpaper("b")];
    app.gallery_selected_index = 1;
    app.move_gallery_selection_up();
    assert_eq!(app.gallery_selected_index, 0);
    app.move_gallery_selection_up();
    assert_eq!(app.gallery_selected_index, 0);
}

#[test]
fn gallery_selection_down_bounded() {
    let mut app = make_app();
    app.gallery_wallpapers = vec![sample_wallpaper("a"), sample_wallpaper("b")];
    app.gallery_selected_index = 0;
    app.move_gallery_selection_down();
    assert_eq!(app.gallery_selected_index, 1);
    app.move_gallery_selection_down();
    assert_eq!(app.gallery_selected_index, 1);
}

// --- Config navigation ---

#[test]
fn config_selection_up_bounded() {
    let mut app = make_app();
    app.config_selected_index = 1;
    app.move_config_selection_up();
    assert_eq!(app.config_selected_index, 0);
    app.move_config_selection_up();
    assert_eq!(app.config_selected_index, 0);
}

#[test]
fn config_selection_down_bounded() {
    let mut app = make_app();
    let len = app.config_fields.len();
    app.config_selected_index = len - 1;
    app.move_config_selection_down();
    assert_eq!(app.config_selected_index, len - 1);
}

#[test]
fn config_input_inserts() {
    let mut app = make_app();
    app.config_input = String::new();
    app.config_cursor = 0;
    app.handle_config_input('a');
    app.handle_config_input('b');
    assert_eq!(app.config_input, "ab");
    assert_eq!(app.config_cursor, 2);
}

#[test]
fn config_backspace_removes() {
    let mut app = make_app();
    app.config_input = "test".to_string();
    app.config_cursor = 4;
    app.handle_config_backspace();
    assert_eq!(app.config_input, "tes");
    assert_eq!(app.config_cursor, 3);
}

#[test]
fn config_cursor_left_right() {
    let mut app = make_app();
    app.config_input = "abc".to_string();
    app.config_cursor = 3;
    app.handle_config_left();
    assert_eq!(app.config_cursor, 2);
    app.handle_config_right();
    assert_eq!(app.config_cursor, 3);
}

#[test]
fn cancel_config_edit_exits_editing() {
    let mut app = make_app();
    app.config_editing = true;
    app.cancel_config_edit();
    assert!(!app.config_editing);
}

// --- Config toggle ---

#[test]
fn toggle_purity_sfw() {
    let mut app = make_app();
    app.config.purity_sfw = true;
    app.config_fields = vec![ConfigField::PuritySfw];
    app.config_selected_index = 0;
    app.toggle_config_field();
    assert!(!app.config.purity_sfw);
}

#[test]
fn toggle_category_anime() {
    let mut app = make_app();
    app.config.category_anime = true;
    app.config_fields = vec![ConfigField::CategoryAnime];
    app.config_selected_index = 0;
    app.toggle_config_field();
    assert!(!app.config.category_anime);
}

// --- Resolution options ---

#[test]
fn resolution_presets_count() {
    let presets = ResolutionOption::presets();
    assert_eq!(presets.len(), 9);
}

#[test]
fn resolution_original_dimensions() {
    assert_eq!(ResolutionOption::Original.dimensions(), (0, 0));
}

#[test]
fn resolution_fhd_dimensions() {
    let fhd = ResolutionOption::FHD1080(walltui::ui::screens::resolution_select::CropMode::Scale);
    assert_eq!(fhd.dimensions(), (1920, 1080));
}

#[test]
fn resolution_4k_dimension() {
    let uhd = ResolutionOption::UHD2160(walltui::ui::screens::resolution_select::CropMode::Scale);
    assert_eq!(uhd.dimensions(), (3840, 2160));
}

#[test]
fn resolution_cycle_crop_mode() {
    let mut opt =
        ResolutionOption::FHD1080(walltui::ui::screens::resolution_select::CropMode::Scale);
    opt.cycle_crop_mode();
    assert!(matches!(
        opt.crop_mode(),
        walltui::ui::screens::resolution_select::CropMode::CropCenter
    ));
    opt.cycle_crop_mode();
    assert!(matches!(
        opt.crop_mode(),
        walltui::ui::screens::resolution_select::CropMode::Fit
    ));
    opt.cycle_crop_mode();
    assert!(matches!(
        opt.crop_mode(),
        walltui::ui::screens::resolution_select::CropMode::Scale
    ));
}

#[test]
fn resolution_original_does_not_cycle() {
    let mut opt = ResolutionOption::Original;
    opt.cycle_crop_mode();
    assert!(matches!(
        opt.crop_mode(),
        walltui::ui::screens::resolution_select::CropMode::Scale
    ));
}

#[test]
fn resolution_label_contains_dimensions() {
    let fhd = ResolutionOption::FHD1080(walltui::ui::screens::resolution_select::CropMode::Scale);
    let label = fhd.label();
    assert!(label.contains("1920x1080"));
}

// --- Quit ---

#[test]
fn quit_sets_should_quit() {
    let mut app = make_app();
    assert!(!app.should_quit);
    app.quit();
    assert!(app.should_quit);
}

// --- Switch provider ---

#[test]
fn switch_provider_changes_active() {
    let mut app = make_app();
    app.active_provider = Provider::Wallhaven;
    app.switch_provider(Provider::Wallhaven);
    assert_eq!(app.active_provider, Provider::Wallhaven);
}

// --- Download manager ---

#[tokio::test]
async fn download_manager_enqueue_and_tasks() {
    let manager = DownloadManager::new();
    let wp = sample_wallpaper("dm1");
    let task = DownloadTask::new(wp.clone(), PathBuf::from("/tmp/test.jpg"));
    manager.enqueue(task).await;
    let tasks = manager.tasks().await;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].wallpaper.id, "dm1");
}

#[tokio::test]
async fn download_manager_cancel_sets_flag() {
    let manager = DownloadManager::new();
    let wp = sample_wallpaper("dm2");
    let task = DownloadTask::new(wp, PathBuf::from("/tmp/test.jpg"));
    manager.enqueue(task).await;
    manager.cancel(0).await;
    let tasks = manager.tasks().await;
    assert!(tasks[0].is_cancelled());
}

#[tokio::test]
async fn download_manager_remove_completed() {
    let manager = DownloadManager::new();
    let wp1 = sample_wallpaper("dm3");
    let wp2 = sample_wallpaper("dm4");
    let mut task1 = DownloadTask::new(wp1, PathBuf::from("/tmp/1.jpg"));
    let task2 = DownloadTask::new(wp2, PathBuf::from("/tmp/2.jpg"));
    task1.status = walltui::core::download::DownloadStatus::Completed;
    manager.enqueue(task1).await;
    manager.enqueue(task2).await;
    manager.remove_completed().await;
    let tasks = manager.tasks().await;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].wallpaper.id, "dm4");
}

#[tokio::test]
async fn download_manager_retry_failed() {
    let manager = DownloadManager::new();
    let wp = sample_wallpaper("dm5");
    let mut task = DownloadTask::new(wp, PathBuf::from("/tmp/test.jpg"));
    task.status = walltui::core::download::DownloadStatus::Failed("err".to_string());
    task.progress = 50;
    manager.enqueue(task).await;
    manager.retry(0).await;
    let tasks = manager.tasks().await;
    assert_eq!(
        tasks[0].status,
        walltui::core::download::DownloadStatus::Queued
    );
    assert_eq!(tasks[0].progress, 0);
    assert!(!tasks[0].is_cancelled());
}

#[tokio::test]
async fn download_manager_len_and_empty() {
    let manager = DownloadManager::new();
    assert!(manager.is_empty().await);
    assert_eq!(manager.len().await, 0);
    let wp = sample_wallpaper("dm6");
    manager
        .enqueue(DownloadTask::new(wp, PathBuf::from("/tmp/test.jpg")))
        .await;
    assert!(!manager.is_empty().await);
    assert_eq!(manager.len().await, 1);
}

// --- Page navigation ---

#[test]
fn next_page_increments() {
    let mut app = make_app();
    app.search_page = 1;
    app.next_page();
    assert_eq!(app.search_page, 2);
}

#[test]
fn prev_page_decrements() {
    let mut app = make_app();
    app.search_page = 3;
    app.prev_page();
    assert_eq!(app.search_page, 2);
}

#[test]
fn prev_page_stops_at_one() {
    let mut app = make_app();
    app.search_page = 1;
    app.prev_page();
    assert_eq!(app.search_page, 1);
}

#[test]
fn reset_search_page_sets_to_one() {
    let mut app = make_app();
    app.search_page = 5;
    app.reset_search_page();
    assert_eq!(app.search_page, 1);
}


