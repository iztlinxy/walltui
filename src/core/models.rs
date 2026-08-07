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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Unsplash,
    Pexels,
    Pixabay,
    Commons,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Unsplash => write!(f, "Unsplash"),
            Provider::Pexels => write!(f, "Pexels"),
            Provider::Pixabay => write!(f, "Pixabay"),
            Provider::Commons => write!(f, "Commons"),
        }
    }
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unsplash" => Ok(Provider::Unsplash),
            "pexels" => Ok(Provider::Pexels),
            "pixabay" => Ok(Provider::Pixabay),
            "commons" => Ok(Provider::Commons),
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

    pub fn build(self) -> SearchQuery {
        SearchQuery {
            query: self.query,
            provider: self.provider,
            page: self.page,
            per_page: self.per_page,
            orientation: self.orientation,
            color: self.color,
        }
    }
}
