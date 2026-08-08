use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::core::errors::AppError;
use crate::core::models::{Provider, SearchQuery, Wallpaper};
use crate::providers::ProviderAdapter;

const BASE_URL: &str = "https://wallhaven.cc/api/v1";

pub struct WallhavenAdapter {
    api_key: Option<String>,
    client: reqwest::Client,
}

impl WallhavenAdapter {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    fn build_search_url(&self, query: &SearchQuery, nsfw_enabled: bool) -> String {
        let mut url = format!("{BASE_URL}/search?q={}", query.query);

        if nsfw_enabled {
            if let Some(key) = &self.api_key {
                url.push_str(&format!("&apikey={key}"));
            }
        }

        url.push_str("&sorting=random");

        if query.page > 1 {
            url.push_str(&format!("&page={}", query.page));
        }

        if let Some(purity) = &query.purity {
            url.push_str(&format!("&purity={purity}"));
        }

        if let Some(categories) = &query.categories {
            url.push_str(&format!("&categories={categories}"));
        }

        if let Some(orientation) = &query.orientation {
            let ratio = match orientation {
                crate::core::models::Orientation::Landscape => "16x9,16x10",
                crate::core::models::Orientation::Portrait => "9x16,10x16",
                crate::core::models::Orientation::Square => "1x1",
            };
            url.push_str(&format!("&ratios={ratio}"));
        }

        if let Some(color) = &query.color {
            let color_hex = color.trim_start_matches('#');
            url.push_str(&format!("&colors={color_hex}"));
        }

        url
    }
}

#[derive(Debug, Deserialize)]
struct WallhavenResponse {
    data: Vec<WallhavenWallpaper>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WallhavenTag {
    id: u64,
    name: String,
    #[serde(default)]
    alias: String,
    #[serde(default)]
    category_id: u64,
    #[serde(default)]
    category: String,
    #[serde(default)]
    purity: String,
    #[serde(default)]
    created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WallhavenWallpaper {
    id: String,
    url: String,
    short_url: String,
    #[serde(default)]
    views: u64,
    #[serde(default)]
    favorites: u64,
    #[serde(default)]
    source: String,
    #[serde(default)]
    purity: String,
    #[serde(default)]
    category: String,
    dimension_x: u32,
    dimension_y: u32,
    #[serde(default)]
    resolution: String,
    #[serde(default)]
    ratio: String,
    #[serde(default)]
    file_size: u64,
    #[serde(default)]
    file_type: String,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    colors: Vec<String>,
    #[serde(default)]
    path: String,
    #[serde(default)]
    thumbs: WallhavenThumbs,
    #[serde(default)]
    tags: Vec<WallhavenTag>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct WallhavenThumbs {
    #[serde(default)]
    large: String,
    #[serde(default)]
    original: String,
    #[serde(default)]
    small: String,
}

impl WallhavenWallpaper {
    fn to_wallpaper(&self) -> Wallpaper {
        let tags: Vec<String> = self.tags.iter().map(|t| t.name.clone()).collect();
        Wallpaper {
            id: self.id.clone(),
            provider: Provider::Wallhaven,
            url: self.path.clone(),
            thumb_url: self.thumbs.large.clone(),
            title: format!("Wallhaven {}", self.id),
            photographer: "Unknown".to_string(),
            width: Some(self.dimension_x),
            height: Some(self.dimension_y),
            avg_color: self.colors.first().cloned(),
            attribution: None,
            file_type: if self.file_type.is_empty() {
                None
            } else {
                Some(self.file_type.clone())
            },
            web_url: Some(self.url.clone()),
            tags,
            category: if self.category.is_empty() {
                None
            } else {
                Some(self.category.clone())
            },
            purity: if self.purity.is_empty() {
                None
            } else {
                Some(self.purity.clone())
            },
            views: Some(self.views),
            favorites: Some(self.favorites),
        }
    }
}

impl ProviderAdapter for WallhavenAdapter {
    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Wallpaper>, AppError>> + Send + 'a>> {
        Box::pin(async move {
            let nsfw_enabled = query.purity.as_ref().map(|p| p.contains('1') && p.len() == 3 && p.chars().nth(2) == Some('1')).unwrap_or(false);
            let url = self.build_search_url(query, nsfw_enabled);
            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| AppError::Network(format!("Wallhaven request failed: {e}")))?;

            let status = response.status();
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(AppError::Network("Rate limit exceeded".to_string()));
            }
            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(AppError::Network(
                    "Unauthorized: invalid API key".to_string(),
                ));
            }
            if !status.is_success() {
                return Err(AppError::Network(format!("HTTP {status}")));
            }

            let wh_response: WallhavenResponse = response
                .json()
                .await
                .map_err(|e| AppError::Network(format!("Failed to parse response: {e}")))?;

            Ok(wh_response.data.iter().map(|w| w.to_wallpaper()).collect())
        })
    }

    fn download_url(&self, wallpaper: &Wallpaper) -> Result<String, AppError> {
        Ok(wallpaper.url.clone())
    }

    fn name(&self) -> &'static str {
        "Wallhaven"
    }
}
