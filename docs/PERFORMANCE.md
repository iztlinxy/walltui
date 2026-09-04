# Performance Benchmarks

Measured on Windows with the release profile enabled in `Cargo.toml`.

## Binary Size

| Build | Size | Target |
|-------|------|--------|
| `target/release/walltui.exe` | ~5.1 MB | < 6 MB |

Release profile:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

## Dependency Bloat Baseline

Run:

```bash
cargo bloat --release --crates -n 10
```

Top crates by size:

| Crate | File % | .text % | Size |
|-------|--------|---------|------|
| std | 15.3% | 19.2% | 729.0 KiB |
| rustls | 6.9% | 8.7% | 330.1 KiB |
| image_webp | 5.4% | 6.8% | 259.0 KiB |
| zune_jpeg | 4.8% | 6.0% | 227.2 KiB |
| walltui | 3.9% | 4.8% | 183.3 KiB |
| image | 3.3% | 4.1% | 154.8 KiB |
| tokio | 2.4% | 3.1% | 116.3 KiB |
| regex_syntax | 2.4% | 3.0% | 113.8 KiB |
| ring | 2.3% | 2.9% | 109.4 KiB |
| regex_automata | 2.3% | 2.9% | 109.4 KiB |

Total `.text` section: 3.7 MiB (79.9% of file).

## Top Functions

Run:

```bash
cargo bloat --release -n 10
```

| Function | File % | .text % | Size |
|----------|--------|---------|------|
| `walltui::run` | 0.8% | 1.0% | 36.8 KiB |
| `image::DynamicImage::resize_exact` | 0.7% | 0.9% | 35.4 KiB |
| `walltui::app::App::draw` | 0.6% | 0.8% | 30.5 KiB |
| `toml_edit::parser::value` | 0.6% | 0.7% | 28.2 KiB |
| `std::sys::process::windows::Command::spawn_with_attributes` | 0.5% | 0.7% | 24.8 KiB |

## Measured Runtime

| Metric | Target | Measured | Notes |
|--------|--------|----------|-------|
| Startup time | < 200 ms | ~360 ms | `WALLTUI_HEADLESS=1` median of 5 runs |
| Binary size | < 5 MB | 4.65 MB | Release build with LTO + strip |
| Frame time (idle) | < 8 ms (120 FPS) | — | Logged if frame > 16 ms |
| Frame time (search) | < 16 ms (60 FPS) | — | Logged if frame > 16 ms |
| Memory (idle) | < 30 MB Working Set | — | Use `Get-Process walltui` |
| Memory (100 results) | < 50 MB Working Set | — | Use `Get-Process walltui` |
| Download throughput | > 10 MB/s | — | Progress bar speed |

## Performance Tests

Run:

```bash
cargo run --bin bench --release --features bench
```

- Measures headless startup, navigation, and draw times.
- Reports peak memory usage via `peak_alloc`.

The release workflow asserts `target/release/walltui.exe` is under 6 MB.
