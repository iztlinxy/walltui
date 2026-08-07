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
