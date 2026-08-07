# WallTUI Architecture

> A terminal user interface (TUI) tool for downloading wallpapers from multiple providers via API.

## Architecture Overview

WallTUI follows the **Elm Architecture** (Model-View-Update) adapted for terminal applications:

```
Event (Input) → Update (Reducer) → Model (State) → View (Render)
     ↑                                                    │
     └────────────────────────────────────────────────────┘
                    (ratatui draw cycle)
```

## Layered Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                        │
│  Screens · Widgets · Keymaps · Theme                         │
├──────────────────────────────────────────────────────────────┤
│                    APPLICATION LAYER                         │
│  App State · Router · Message Management                     │
├──────────────────────────────────────────────────────────────┤
│                      DOMAIN LAYER                            │
│  Provider Adapters · Search Engine · Download Manager        │
├──────────────────────────────────────────────────────────────┤
│                   INFRASTRUCTURE LAYER                       │
│  HTTP Client · Disk Cache · Config Files                     │
└──────────────────────────────────────────────────────────────┘
```

## Project Structure

```
walltui/
├── Cargo.toml
├── README.md
├── config.toml
├── docs/
│   ├── ARCHITECTURE.md
│   └── wallhaven-api-docs.md
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # App state and routing
│   ├── lib.rs               # Re-exports
│   ├── core/                # Domain logic
│   │   ├── models.rs
│   │   ├── search.rs
│   │   ├── download.rs
│   │   └── errors.rs
│   ├── providers/           # Provider adapters
│   │   ├── mod.rs
│   │   ├── wallhaven.rs
│   │   └── pixiv.rs
│   ├── ui/                  # Presentation layer
│   │   ├── app_layout.rs
│   │   ├── theme.rs
│   │   ├── screens/
│   │   │   ├── splash.rs
│   │   │   ├── search.rs
│   │   │   ├── detail.rs
│   │   │   ├── download.rs
│   │   │   └── config.rs
│   │   └── widgets/
│   │       ├── image_list.rs
│   │       ├── image_card.rs
│   │       ├── search_bar.rs
│   │       ├── progress_bar.rs
│   │       └── help_bar.rs
│   ├── infrastructure/      # I/O layer
│   │   ├── http.rs
│   │   ├── cache.rs
│   │   ├── config_loader.rs
│   │   └── fs.rs
│   ├── platform/            # Platform-specific wallpaper setter
│   │   └── mod.rs
│   └── utils.rs
└── tests/
```

## Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust |
| TUI Framework | ratatui + crossterm |
| HTTP Client | reqwest + tokio |
| Configuration | serde + toml |
| CLI Args | clap |
| Logging | tracing |
| Error Handling | color-eyre |
| Image Processing | image |
| Image Preview | viuer |

## Implementation Phases

### Phase 0: Setup & Foundations
- Cargo project setup
- Base dependencies
- Terminal init/restore lifecycle
- Event loop
- Module structure
- Tracing logging
- AppError enum
- color-eyre integration

### Phase 1: Data Models & Core Domain
- Wallpaper struct
- Provider enum (Wallhaven, Pixiv)
- SearchQuery with builder
- ProviderAdapter trait
- DownloadTask and DownloadStatus
- DownloadManager with FIFO queue
- Filename generator

### Phase 2: Provider Adapters
- WallhavenAdapter with search, download URL, rate limit handling
- PixivAdapter stub
- Provider factory

### Phase 3: Visualization, Search & UI
- Screen enum and routing
- AppLayout, Theme, HelpBar
- Splash, Search, Detail, Download, Config screens
- SearchBar, ImageList widgets
- Live search integration
- Thumbnail ASCII preview

### Phase 4: Downloads & Progress
- Real HTTP downloads with streaming progress
- tokio::sync::mpsc progress reporting
- Retry logic with exponential backoff
- Download cancellation

### Phase 5: Configuration & Polish
- TOML config load/save
- Theme persistence
- Settings screen

### Phase 6: Testing & Quality
- Unit tests for models
- clippy clean
- cargo fmt

### Phase 7: Advanced Features
- Cross-platform wallpaper setter

## Key Design Decisions

- **Rust + ratatui**: Mature TUI ecosystem with immediate-mode rendering.
- **Layered Architecture**: Separates pure logic from I/O and rendering.
- **Provider Adapter Pattern**: Uniform interface for different provider APIs.
- **Async Downloads with Sync State**: tokio tasks for downloads, channels for UI updates.
- **ASCII Image Preview**: Lightweight preview without terminal image protocol requirements.
