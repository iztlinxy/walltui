# WallTUI

A terminal user interface (TUI) tool for searching and downloading wallpapers from multiple providers.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Features

- Search wallpapers from **Wallhaven** and **Pixiv**
- Terminal-based UI with multiple screens
- Download wallpapers with progress tracking
- Randomized search results
- Pagination support (`n` / `p`)
- ASCII thumbnail preview in search and detail screens
- Cross-platform wallpaper setter (Windows, Linux, macOS)
- Theme switching (dark / light)
- Configuration file persistence

## Installation

Requires [Rust](https://rustup.rs/) 1.70+.

```bash
git clone https://github.com/yourusername/walltui.git
cd walltui
cargo build --release
```

The binary will be available at `target/release/walltui`.

## Usage

```bash
cargo run
```

### Global Keys

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `?` | Toggle contextual help |

### Splash Screen

| Key | Action |
|-----|--------|
| `s` | Go to search screen |
| `c` | Go to settings |
| `q` | Quit |

### Search Screen

| Key | Action |
|-----|--------|
| `/` | Focus search bar |
| `Enter` | Execute search |
| `1` / `2` | Switch provider (Wallhaven / Pixiv) |
| `↑` / `↓` or `k` / `j` | Navigate results |
| `n` / `p` | Next / previous page |
| `Enter` | View detail |
| `d` | Go to downloads screen |
| `Esc` | Back |

### Detail Screen

| Key | Action |
|-----|--------|
| `d` | Download current image |
| `o` | Open wallpaper page in browser |
| `Esc` | Back |

### Downloads Screen

| Key | Action |
|-----|--------|
| `x` | Cancel selected download |
| `r` | Retry failed download |
| `c` | Clear completed downloads |
| `Esc` | Back |

### Settings Screen

| Key | Action |
|-----|--------|
| `t` | Toggle theme |
| `Ctrl+S` | Save configuration |
| `Esc` | Back |

## Configuration

Configuration is stored at the OS config directory:

- Windows: `%APPDATA%\walltui\config.toml`
- Linux: `~/.config/walltui/config.toml`
- macOS: `~/Library/Application Support/walltui/config.toml`

Example `config.toml`:

```toml
wallhaven_api_key = "your-wallhaven-api-key"
pixiv_api_key = "your-pixiv-api-key"
download_dir = "~/Downloads/walltui"
default_provider = "Wallhaven"
theme = "dark"
```

### Providers

| Provider | API Key Required | Notes |
|----------|-----------------|-------|
| Wallhaven | Optional | Required for NSFW content |
| Pixiv | Optional | Stub implementation |

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architecture and project structure.

## License

MIT
