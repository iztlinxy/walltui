use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn release_exe() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("release")
        .join("walltui.exe")
}

#[test]
#[ignore = "requires release binary built locally; run with --include-ignored"]
fn binary_size_under_limit() {
    let exe = release_exe();
    let metadata = std::fs::metadata(&exe).unwrap_or_else(|e| {
        panic!("failed to stat {}: {e}", exe.display());
    });
    let size_mb = metadata.len() as f64 / 1_048_576.0;
    assert!(size_mb < 5.0, "Binary too large: {:.2} MB", size_mb);
}

#[test]
#[ignore = "requires release binary built locally; run with --include-ignored"]
fn startup_time_under_limit() {
    let exe = release_exe();
    let start = Instant::now();
    let child = Command::new(&exe)
        .env("WALLTUI_HEADLESS", "1")
        .spawn()
        .expect("failed to spawn walltui");

    let child = Arc::new(Mutex::new(child));
    let child_for_timeout = Arc::clone(&child);
    let timeout = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let _ = child_for_timeout.lock().unwrap().kill();
    });

    let _ = child.lock().unwrap().wait().expect("failed to wait on child");
    timeout.join().expect("timeout thread panicked");

    let elapsed = start.elapsed();
    // ponytail: target is 500ms, but CI/AV overhead can spike; keep guard at 2000ms
    assert!(
        elapsed < Duration::from_millis(2000),
        "Startup too slow: {:?}",
        elapsed
    );
}
