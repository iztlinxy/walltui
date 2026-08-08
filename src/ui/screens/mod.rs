pub mod config;
pub mod detail;
pub mod download;
pub mod gallery;
pub mod resolution_select;
pub mod search;
pub mod splash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Splash,
    Search,
    Detail,
    Download,
    Config,
    Gallery,
    ResolutionSelect,
}
