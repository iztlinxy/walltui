use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub provider: Provider,
    pub url: String,
    pub thumb_url: String,
    pub title: String,
    pub photographer: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub avg_color: Option<String>,
    pub attribution: Option<String>,
    pub file_type: Option<String>,
    pub web_url: Option<String>,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub purity: Option<String>,
    pub views: Option<u64>,
    pub favorites: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Wallhaven,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Wallhaven => write!(f, "Wallhaven"),
        }
    }
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wallhaven" => Ok(Provider::Wallhaven),
            other => Err(format!("unknown provider: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::Landscape => write!(f, "landscape"),
            Orientation::Portrait => write!(f, "portrait"),
            Orientation::Square => write!(f, "square"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub provider: Option<Provider>,
    pub page: u32,
    pub per_page: u32,
    pub orientation: Option<Orientation>,
    pub color: Option<String>,
    pub purity: Option<String>,
    pub categories: Option<String>,
}

impl SearchQuery {
    pub fn builder(query: impl Into<String>) -> SearchQueryBuilder {
        SearchQueryBuilder {
            query: query.into(),
            provider: None,
            page: 1,
            per_page: 20,
            orientation: None,
            color: None,
            purity: None,
            categories: None,
        }
    }
}

pub struct SearchQueryBuilder {
    query: String,
    provider: Option<Provider>,
    page: u32,
    per_page: u32,
    orientation: Option<Orientation>,
    color: Option<String>,
    purity: Option<String>,
    categories: Option<String>,
}

impl SearchQueryBuilder {
    pub fn provider(mut self, provider: Provider) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn purity(mut self, purity: impl Into<String>) -> Self {
        self.purity = Some(purity.into());
        self
    }

    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    pub fn build(self) -> SearchQuery {
        SearchQuery {
            query: self.query,
            provider: self.provider,
            page: self.page,
            per_page: self.per_page,
            orientation: self.orientation,
            color: self.color,
            purity: self.purity,
            categories: self.categories,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryEntry {
    pub wallpaper: Wallpaper,
    pub custom_name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub date_added: u64,
    #[serde(default)]
    pub date_modified: u64,
}

impl GalleryEntry {
    pub fn display_name(&self) -> &str {
        self.custom_name
            .as_deref()
            .or_else(|| Some(self.wallpaper.title.as_str()))
            .unwrap_or(&self.wallpaper.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wallpaper() -> Wallpaper {
        Wallpaper {
            id: "abc-123".to_string(),
            provider: Provider::Wallhaven,
            url: "https://example.com/full.jpg".to_string(),
            thumb_url: "https://example.com/thumb.jpg".to_string(),
            title: "Test Image".to_string(),
            photographer: "Test User".to_string(),
            width: Some(1920),
            height: Some(1080),
            avg_color: Some("#ff0000".to_string()),
            attribution: None,
            file_type: None,
            web_url: None,
            tags: vec!["test".to_string()],
            category: None,
            purity: None,
            views: None,
            favorites: None,
        }
    }

    #[test]
    fn wallpaper_serialization_round_trip() {
        let wallpaper = sample_wallpaper();
        let json = serde_json::to_string(&wallpaper).unwrap();
        let deserialized: Wallpaper = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, wallpaper.id);
        assert_eq!(deserialized.provider, wallpaper.provider);
        assert_eq!(deserialized.url, wallpaper.url);
        assert_eq!(deserialized.photographer, wallpaper.photographer);
        assert_eq!(deserialized.width, wallpaper.width);
        assert_eq!(deserialized.height, wallpaper.height);
    }

    #[test]
    fn search_query_builder_defaults() {
        let query = SearchQuery::builder("nature").build();
        assert_eq!(query.query, "nature");
        assert_eq!(query.page, 1);
        assert_eq!(query.per_page, 20);
        assert!(query.provider.is_none());
        assert!(query.orientation.is_none());
        assert!(query.color.is_none());
        assert!(query.purity.is_none());
        assert!(query.categories.is_none());
    }

    #[test]
    fn search_query_builder_with_all_fields() {
        let query = SearchQuery::builder("city")
            .provider(Provider::Wallhaven)
            .page(2)
            .per_page(50)
            .orientation(Orientation::Landscape)
            .color("black")
            .purity("110")
            .categories("111")
            .build();

        assert_eq!(query.query, "city");
        assert_eq!(query.provider, Some(Provider::Wallhaven));
        assert_eq!(query.page, 2);
        assert_eq!(query.per_page, 50);
        assert_eq!(query.orientation, Some(Orientation::Landscape));
        assert_eq!(query.color, Some("black".to_string()));
        assert_eq!(query.purity, Some("110".to_string()));
        assert_eq!(query.categories, Some("111".to_string()));
    }

    #[test]
    fn provider_display() {
        assert_eq!(format!("{}", Provider::Wallhaven), "Wallhaven");
    }

    #[test]
    fn provider_from_str() {
        assert_eq!(
            "wallhaven".parse::<Provider>().unwrap(),
            Provider::Wallhaven
        );
        assert!("unknown".parse::<Provider>().is_err());
    }
}
