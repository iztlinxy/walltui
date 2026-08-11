use std::future::Future;
use std::pin::Pin;

pub mod wallhaven;
pub mod ytdlp;

use crate::core::errors::AppError;
use crate::core::models::{Provider, SearchQuery, Wallpaper};

pub trait ProviderAdapter: Send + Sync {
    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Wallpaper>, AppError>> + Send + 'a>>;

    fn download_url(&self, wallpaper: &Wallpaper) -> Result<String, AppError>;

    fn name(&self) -> &'static str;
}

pub fn create_provider(provider: Provider, api_key: Option<String>) -> Box<dyn ProviderAdapter> {
    match provider {
        Provider::Wallhaven => Box::new(wallhaven::WallhavenAdapter::new(api_key)),
        Provider::YtDlp => Box::new(ytdlp::YtDlpProviderAdapter::new()),
    }
}
