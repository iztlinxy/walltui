use std::future::Future;
use std::pin::Pin;

pub mod pixiv;
pub mod wallhaven;

use crate::core::errors::AppError;
use crate::core::models::{SearchQuery, Wallpaper};

pub trait ProviderAdapter: Send + Sync {
    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Wallpaper>, AppError>> + Send + 'a>>;

    fn download_url(&self, wallpaper: &Wallpaper) -> Result<String, AppError>;

    fn name(&self) -> &'static str;
}
