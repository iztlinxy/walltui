pub mod config;
pub mod download;
pub mod gallery;
pub mod resolution_select;
pub mod search;
pub mod splash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Splash,
    Search,
    Download,
    Config,
    Gallery,
    ResolutionSelect,
}
