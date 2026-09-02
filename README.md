<p align="center">
  <img src="images/large.png" alt="WallTUI search screen" width="720">
</p>

<h1 align="center">WallTUI</h1>

<p align="center">
  <b>Terminal Wallpaper Manager</b><br>
  Browse, preview, and download wallpapers from Wallhaven without leaving your terminal.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.4.1-%2300d2ff?style=flat-square" alt="Version">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=flat-square&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/TUI-ratatui-%23ebcb8b?style=flat-square" alt="ratatui">
  <img src="https://img.shields.io/badge/license-MIT-%2300d2ff?style=flat-square" alt="License">
</p>

---

## Features

- **In-terminal previews** — color-accurate thumbnails rendered with Unicode half-blocks
- **Instant search** — query Wallhaven with live results, ratios, favorites, and tags
- **Built-in gallery** — browse downloaded wallpapers with thumbnails and metadata
- **Resolution picker** — download at 720p, 1080p, 1440p, 4K, ultrawide, and more
- **Persistent config** — API key, download folder, purity, and category filters saved automatically

## Install

### WinGet

```powershell
winget install WallTUI.WallTUI
```

### MSI / Portable

Download the latest release from [GitHub Releases](https://github.com/iztlinxy/walltui/releases/latest).

### From source

```bash
cargo build --release
./target/release/walltui
```

## Quick Start

```bash
walltui
```

Use the splash menu or quick keys:

| Key | Screen |
|:---:|:-------|
| `s` | Search |
| `g` | Gallery |
| `c` | Settings |
| `q` | Quit |

## Controls

### Search

| Key | Action |
|:---:|:-------|
| `/` | Edit query |
| `Enter` | Apply / open resolution picker |
| `d` | Save original |
| `f` | Fullscreen preview |
| `↑`/`↓` or `k`/`j` | Navigate results |
| `Esc` | Back |

### Gallery

| Key | Action |
|:---:|:-------|
| `↑`/`↓` or `k`/`j` | Navigate |
| `Enter` | Open in default viewer |
| `w` | Set as wallpaper |
| `r` | Rename |
| `d` | Delete |
| `Esc` | Back |

### Settings

| Key | Action |
|:---:|:-------|
| `↑`/`↓` or `k`/`j` | Navigate options |
| `Enter` | Edit text field or toggle switch |
| `←`/`→` | Cycle cursor style selector |
| `Esc` | Back / cancel edit |
| `Ctrl+S` | Save |

## Configuration

WallTUI stores its config and data under `%USERPROFILE%\.config\walltui`.

```toml
wallhaven_api_key = "your-key-here"
download_dir = "~/Downloads"
default_provider = "Wallhaven"
purity_sfw = true
purity_sketchy = false
purity_nsfw = false
category_general = true
category_anime = true
category_people = false
cursor_style = "block"
```

> **Note:** a Wallhaven API key is only required for NSFW content.

## License

MIT
