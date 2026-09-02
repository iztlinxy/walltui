<p align="center">
  <pre>
 _    _   ___   _     _     _____  _   _  _____
| |  | | / _ \ | |   | |   |_   _|| | | ||_   _|
| |/\| || |_| || |   | |     | |  | | | |  | |
|  /\  ||  _  || |__ | |__   | |  | |_| | _| |_
\_/ \_/ |_| |_|\____/|____|  |_|  |_____| |_____|
  </pre>
  <p align="center"><b>Terminal Wallpaper Manager</b></p>
  <p align="center"><b>v1.0</b></p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-%23007EC6?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/TUI-ratatui-%23B14B2E?style=for-the-badge&logo=rust&logoColor=white" alt="ratatui">
  <img src="https://img.shields.io/badge/license-MIT-%23007EC6?style=for-the-badge" alt="License">
</p>

---

Browse, preview, and download wallpapers from **Wallhaven** — all from your terminal.

## Features

- **ASCII thumbnail previews** — color-accurate previews using Unicode half-blocks
- **Resolution picker** — download at 720p, 1080p, 1440p, 4K, ultrawide, or phone with stretch/crop/fit modes
- **Gallery** — browse your downloaded wallpapers with cached thumbnails
- **Download manager** — queue, cancel, retry with progress tracking
- **Search filters** — toggle SFW / Sketchy / NSFW purity and General / Anime / People categories
- **Persistent config** — API key, download dir, purity, and categories saved automatically

## Quick Start

```bash
cargo run --release
```

Or build and run the binary:

```bash
cargo build --release
./target/release/walltui
```

## Installation

### WinGet (recommended)

```powershell
winget install WallTUI.WallTUI
```

### MSI installer

Download `walltui-x86_64-pc-windows-msvc.msi` from the [latest release](https://github.com/iztlinxy/walltui/releases/latest) and run it. The installer adds `walltui` to your PATH.

### Portable ZIP

Download `walltui-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/iztlinxy/walltui/releases/latest), extract it, and run `walltui.exe`.

### PowerShell one-liner

```powershell
iwr https://github.com/iztlinxy/walltui/releases/latest/download/walltui-installer.ps1 -OutFile walltui-installer.ps1; .\walltui-installer.ps1
```

### Build from source

See [Quick Start](#quick-start).

## Screens

```
Splash ──┬── s ──→ Search ──→ Detail ──┬── d ──→ Resolution Select ──→ Downloads
         │                              │
         ├── g ──→ Gallery              └── o ──→ Open in browser
         │
         └── c ──→ Settings
```

## Keybindings

### Global

| Key | Action |
|:---:|--------|
| `q` / `Ctrl+C` | Quit |

### Splash

| Key | Action |
|:---:|--------|
| `s` | Search |
| `g` | Gallery |
| `c` | Settings |

### Search

| Key | Action |
|:---:|--------|
| `/` | Focus search bar |
| `Enter` | Execute search |
| `↑`/`↓` or `k`/`j` | Navigate results |
| `Enter` | View detail |
| `n` / `p` | Next / prev page |
| `Esc` | Back |

### Detail

| Key | Action |
|:---:|--------|
| `d` | Download (opens resolution picker) |
| `o` | Open in browser |
| `Esc` | Back |

### Resolution Select

| Key | Action |
|:---:|--------|
| `↑`/`↓` or `k`/`j` | Navigate resolutions |
| `c` | Cycle crop mode (Stretch → Crop → Fit) |
| `Enter` | Download selected |
| `Esc` | Back |

### Downloads

| Key | Action |
|:---:|--------|
| `x` | Cancel |
| `r` | Retry |
| `c` | Clear completed |
| `Esc` | Back |

### Gallery

| Key | Action |
|:---:|--------|
| `↑`/`↓` or `k`/`j` | Navigate |
| `Enter` | Open in default viewer |
| `r` | Reload |
| `Esc` | Back |

### Settings

| Key | Action |
|:---:|--------|
| `↑`/`↓` or `k`/`j` | Navigate fields |
| `Enter` | Edit (API key, download dir) or Toggle (purity, categories) |
| `Esc` | Back / Cancel edit |
| `Ctrl+S` | Save |

## Download Resolutions

| Preset | Dimensions | Default Mode |
|--------|-----------|--------------|
| Original | — | No resize |
| HD | 1280×720 | Stretch |
| FHD | 1920×1080 | Stretch |
| QHD | 2560×1440 | Stretch |
| 4K | 3840×2160 | Stretch |
| Ultrawide | 2560×1080 | Crop |
| Ultrawide+ | 3440×1440 | Crop |
| MacBook 16" | 3072×1920 | Fit |
| Phone | 1080×1920 | Crop |

**Crop modes:**
- **Stretch** — exact resize, may distort aspect ratio
- **Crop** — resize to target, center-crop to fill
- **Fit** — resize to fit within bounds, preserves aspect ratio

## Settings

Toggle purity and category filters directly from the Settings screen. Filters are sent to Wallhaven as binary strings (e.g. `110` = SFW+Sketchy, `101` = SFW+NSFW).

| Field | Description |
|-------|-------------|
| **API Key** | Wallhaven API key. Only sent when NSFW is enabled |
| **Download** | Directory where wallpapers are saved |
| **SFW** | Safe for work content |
| **Sketchy** | Borderline content |
| **NSFW** | Not safe for work (requires API key) |
| **General** | General category wallpapers |
| **Anime** | Anime category wallpapers |
| **People** | People category wallpapers |

> **API Key:** Optional for Wallhaven. Required only for NSFW content. Get yours at [wallhaven.cc/settings](https://wallhaven.cc/settings).

## Configuration

WallTUI is Windows-only. All configuration and data live under `%USERPROFILE%\.config\walltui`:

| File | Path |
|------|------|
| Config | `%USERPROFILE%\.config\walltui\config.toml` |
| Gallery index | `%USERPROFILE%\.config\walltui\gallery.json` |
| Custom themes | `%USERPROFILE%\.config\walltui\themes\*.toml` |

```toml
wallhaven_api_key = "your-key-here"
download_dir = "~/Downloads/wallpapers"
default_provider = "Wallhaven"
purity_sfw = true
purity_sketchy = false
purity_nsfw = false
category_general = true
category_anime = true
category_people = false
```

## Tech Stack

| Layer | Crate |
|-------|-------|
| TUI | [ratatui](https://ratatui.rs) + [crossterm](https://github.com/crossterm-rs/crossterm) |
| Async | [tokio](https://tokio.rs) |
| HTTP | [reqwest](https://github.com/seanmonstar/reqwest) |
| Images | [image](https://github.com/image-rs/image) (Lanczos3 resize) |
| Config | serde + toml |
| Errors | color-eyre |
| Logging | tracing |

## License

MIT
