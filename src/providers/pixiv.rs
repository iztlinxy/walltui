use std::future::Future;
use std::pin::Pin;

use crate::core::errors::AppError;
use crate::core::models::{SearchQuery, Wallpaper};
use crate::providers::ProviderAdapter;

pub struct PixivAdapter {
    api_key: Option<String>,
}

impl PixivAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl ProviderAdapter for PixivAdapter {
    fn search<'a>(
        &'a self,
        _query: &'a SearchQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Wallpaper>, AppError>> + Send + 'a>> {
        Box::pin(async move {
            Err(AppError::Provider("Pixiv adapter not yet implemented".to_string()))
        })
    }

    fn download_url(&self, _wallpaper: &Wallpaper) -> Result<String, AppError> {
        Err(AppError::Provider("Pixiv adapter not yet implemented".to_string()))
    }

    fn name(&self) -> &'static str {
        "Pixiv"
    }
}
