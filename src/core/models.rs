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
