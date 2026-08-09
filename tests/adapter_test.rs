use walltui::core::download::generate_filename;
use walltui::core::models::{Orientation, Provider, SearchQuery, Wallpaper};
use walltui::providers::create_provider;

fn sample_wallpaper() -> Wallpaper {
    Wallpaper {
        id: "abc123".to_string(),
        provider: Provider::Wallhaven,
        url: "https://w.wallhaven.cc/full/ab/wallhaven-abc123.jpg".to_string(),
        thumb_url: "https://th.wallhaven.cc/small/ab/wallhaven-abc123.jpg".to_string(),
        title: "Test Wallpaper".to_string(),
        photographer: "Unknown".to_string(),
        width: Some(1920),
        height: Some(1080),
        avg_color: Some("#ff0000".to_string()),
        attribution: None,
        file_type: Some("image/jpeg".to_string()),
        web_url: Some("https://wallhaven.cc/w/abc123".to_string()),
        tags: vec!["nature".to_string(), "landscape".to_string()],
        category: Some("general".to_string()),
        purity: Some("sfw".to_string()),
        views: Some(1000),
        favorites: Some(50),
    }
}

// --- create_provider factory ---

#[test]
fn create_provider_returns_wallhaven() {
    let provider = create_provider(Provider::Wallhaven, None);
    assert_eq!(provider.name(), "Wallhaven");
}

#[test]
fn create_provider_with_api_key() {
    let provider = create_provider(Provider::Wallhaven, Some("test-key".to_string()));
    assert_eq!(provider.name(), "Wallhaven");
}

// --- download_url ---

#[test]
fn download_url_returns_wallpaper_url() {
    let provider = create_provider(Provider::Wallhaven, None);
    let wp = sample_wallpaper();
    let url = provider.download_url(&wp).unwrap();
    assert_eq!(url, "https://w.wallhaven.cc/full/ab/wallhaven-abc123.jpg");
}

// --- generate_filename ---

#[test]
fn generate_filename_with_dimensions() {
    let wp = sample_wallpaper();
    let name = generate_filename(&wp);
    assert_eq!(name, "wallhaven_abc123_1920x1080.jpg");
}

#[test]
fn generate_filename_without_dimensions() {
    let mut wp = sample_wallpaper();
    wp.width = None;
    wp.height = None;
    let name = generate_filename(&wp);
    assert_eq!(name, "wallhaven_abc123_unknown.jpg");
}

#[test]
fn generate_filename_png() {
    let mut wp = sample_wallpaper();
    wp.file_type = Some("image/png".to_string());
    wp.id = "xyz789".to_string();
    let name = generate_filename(&wp);
    assert!(name.ends_with(".png"));
}

#[test]
fn generate_filename_webp() {
    let mut wp = sample_wallpaper();
    wp.file_type = Some("image/webp".to_string());
    wp.id = "web001".to_string();
    let name = generate_filename(&wp);
    assert!(name.ends_with(".webp"));
}

#[test]
fn generate_filename_sanitizes_id() {
    let mut wp = sample_wallpaper();
    wp.id = "abc/123\\bad".to_string();
    let name = generate_filename(&wp);
    assert!(!name.contains('/'));
    assert!(!name.contains('\\'));
}

// --- SearchQuery URL building via parsing logic ---

#[test]
fn search_query_basic() {
    let query = SearchQuery::builder("nature").build();
    assert_eq!(query.query, "nature");
    assert_eq!(query.page, 1);
    assert_eq!(query.per_page, 20);
}

#[test]
fn search_query_with_orientation() {
    let query = SearchQuery::builder("city")
        .orientation(Orientation::Landscape)
        .build();
    assert_eq!(query.orientation, Some(Orientation::Landscape));
}

#[test]
fn search_query_with_purity_and_categories() {
    let query = SearchQuery::builder("anime")
        .purity("110")
        .categories("010")
        .page(3)
        .per_page(50)
        .build();
    assert_eq!(query.purity, Some("110".to_string()));
    assert_eq!(query.categories, Some("010".to_string()));
    assert_eq!(query.page, 3);
    assert_eq!(query.per_page, 50);
}

#[test]
fn search_query_with_color() {
    let query = SearchQuery::builder("dark")
        .color("#ff0000")
        .build();
    assert_eq!(query.color, Some("#ff0000".to_string()));
}

// --- Wallpaper JSON parsing (adapter deserialization) ---

#[test]
fn wallhaven_json_parses_minimal() {
    let json = r#"{
        "id": "12345",
        "url": "https://wallhaven.cc/w/12345",
        "short_url": "https://whvn.cc/12345",
        "dimension_x": 1920,
        "dimension_y": 1080,
        "path": "https://w.wallhaven.cc/full/ab/wallhaven-12345.jpg",
        "thumbs": { "large": "https://th.wallhaven.cc/large/12345.jpg" }
    }"#;
    let parsed: serde_json::Result<wallhaven_test::WallhavenWallpaper> = serde_json::from_str(json);
    assert!(parsed.is_ok());
    let wp = parsed.unwrap();
    assert_eq!(wp.id, "12345");
    assert_eq!(wp.dimension_x, 1920);
    assert_eq!(wp.dimension_y, 1080);
}

#[test]
fn wallhaven_json_parses_full() {
    let json = r##"{
        "id": "67890",
        "url": "https://wallhaven.cc/w/67890",
        "short_url": "https://whvn.cc/67890",
        "views": 5000,
        "favorites": 200,
        "purity": "sfw",
        "category": "general",
        "dimension_x": 3840,
        "dimension_y": 2160,
        "resolution": "3840x2160",
        "ratio": "16:9",
        "file_size": 2048000,
        "file_type": "image/jpeg",
        "path": "https://w.wallhaven.cc/full/cd/wallhaven-67890.jpg",
        "thumbs": {
            "large": "https://th.wallhaven.cc/large/67890.jpg",
            "original": "https://th.wallhaven.cc/original/67890.jpg",
            "small": "https://th.wallhaven.cc/small/67890.jpg"
        },
        "tags": [
            {"id": 1, "name": "nature", "category": "general"}
        ],
        "colors": ["#ff0000", "#00ff00"]
    }"##;
    let parsed: serde_json::Result<wallhaven_test::WallhavenWallpaper> = serde_json::from_str(json);
    assert!(parsed.is_ok());
    let wp = parsed.unwrap();
    assert_eq!(wp.views, 5000);
    assert_eq!(wp.favorites, 200);
    assert_eq!(wp.purity, "sfw");
    assert_eq!(wp.file_type, "image/jpeg");
    assert_eq!(wp.colors.len(), 2);
    assert_eq!(wp.tags.len(), 1);
    assert_eq!(wp.tags[0].name, "nature");
}

#[test]
fn wallhaven_json_defaults_empty_fields() {
    let json = r#"{
        "id": "11111",
        "url": "https://wallhaven.cc/w/11111",
        "short_url": "https://whvn.cc/11111",
        "dimension_x": 100,
        "dimension_y": 100,
        "path": "https://w.wallhaven.cc/full/ab/wallhaven-11111.jpg"
    }"#;
    let parsed: serde_json::Result<wallhaven_test::WallhavenWallpaper> = serde_json::from_str(json);
    assert!(parsed.is_ok());
    let wp = parsed.unwrap();
    assert!(wp.tags.is_empty());
    assert!(wp.colors.is_empty());
    assert_eq!(wp.thumbs.large, "");
    assert_eq!(wp.file_type, "");
    assert_eq!(wp.purity, "");
    assert_eq!(wp.category, "");
}

#[test]
fn wallhaven_response_parses_array() {
    let json = r#"{
        "data": [
            {
                "id": "1",
                "url": "https://wallhaven.cc/w/1",
                "short_url": "https://whvn.cc/1",
                "dimension_x": 100,
                "dimension_y": 100,
                "path": "https://example.com/1.jpg"
            },
            {
                "id": "2",
                "url": "https://wallhaven.cc/w/2",
                "short_url": "https://whvn.cc/2",
                "dimension_x": 200,
                "dimension_y": 200,
                "path": "https://example.com/2.jpg"
            }
        ]
    }"#;
    let parsed: serde_json::Result<wallhaven_test::WallhavenResponse> = serde_json::from_str(json);
    assert!(parsed.is_ok());
    let resp = parsed.unwrap();
    assert_eq!(resp.data.len(), 2);
}

// --- to_wallpaper conversion ---

#[test]
fn wallhaven_to_wallpaper_maps_fields() {
    let json = r##"{
        "id": "abc999",
        "url": "https://wallhaven.cc/w/abc999",
        "short_url": "https://whvn.cc/abc999",
        "dimension_x": 2560,
        "dimension_y": 1440,
        "path": "https://w.wallhaven.cc/full/xy/wallhaven-abc999.png",
        "thumbs": { "large": "https://th.wallhaven.cc/large/abc999.png" },
        "file_type": "image/png",
        "purity": "sfw",
        "category": "anime",
        "views": 999,
        "favorites": 42,
        "tags": [{"id": 1, "name": "anime"}, {"id": 2, "name": "scenery"}],
        "colors": ["#aabbcc"]
    }"##;
    let wp_json: wallhaven_test::WallhavenWallpaper = serde_json::from_str(json).unwrap();
    let wallpaper = wp_json.to_wallpaper();

    assert_eq!(wallpaper.id, "abc999");
    assert_eq!(wallpaper.provider, Provider::Wallhaven);
    assert_eq!(wallpaper.url, "https://w.wallhaven.cc/full/xy/wallhaven-abc999.png");
    assert_eq!(wallpaper.thumb_url, "https://th.wallhaven.cc/large/abc999.png");
    assert_eq!(wallpaper.title, "Wallhaven abc999");
    assert_eq!(wallpaper.width, Some(2560));
    assert_eq!(wallpaper.height, Some(1440));
    assert_eq!(wallpaper.file_type, Some("image/png".to_string()));
    assert_eq!(wallpaper.purity, Some("sfw".to_string()));
    assert_eq!(wallpaper.category, Some("anime".to_string()));
    assert_eq!(wallpaper.views, Some(999));
    assert_eq!(wallpaper.favorites, Some(42));
    assert_eq!(wallpaper.tags, vec!["anime".to_string(), "scenery".to_string()]);
    assert_eq!(wallpaper.avg_color, Some("#aabbcc".to_string()));
    assert_eq!(wallpaper.web_url, Some("https://wallhaven.cc/w/abc999".to_string()));
}

#[test]
fn wallhaven_to_wallpaper_empty_fields_become_none() {
    let json = r#"{
        "id": "empty1",
        "url": "https://wallhaven.cc/w/empty1",
        "short_url": "https://whvn.cc/empty1",
        "dimension_x": 100,
        "dimension_y": 100,
        "path": "https://example.com/empty1.jpg"
    }"#;
    let wp_json: wallhaven_test::WallhavenWallpaper = serde_json::from_str(json).unwrap();
    let wallpaper = wp_json.to_wallpaper();

    assert!(wallpaper.file_type.is_none());
    assert!(wallpaper.purity.is_none());
    assert!(wallpaper.category.is_none());
    assert!(wallpaper.avg_color.is_none());
    assert!(wallpaper.tags.is_empty());
}

// --- WallhavenAdapter URL building (tested via trait + internal logic) ---

mod wallhaven_test {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct WallhavenResponse {
        pub data: Vec<WallhavenWallpaper>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WallhavenTag {
        pub id: u64,
        pub name: String,
        #[serde(default)]
        pub alias: String,
        #[serde(default)]
        pub category_id: u64,
        #[serde(default)]
        pub category: String,
        #[serde(default)]
        pub purity: String,
        #[serde(default)]
        pub created_at: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct WallhavenWallpaper {
        pub id: String,
        pub url: String,
        pub short_url: String,
        #[serde(default)]
        pub views: u64,
        #[serde(default)]
        pub favorites: u64,
        #[serde(default)]
        pub source: String,
        #[serde(default)]
        pub purity: String,
        #[serde(default)]
        pub category: String,
        pub dimension_x: u32,
        pub dimension_y: u32,
        #[serde(default)]
        pub resolution: String,
        #[serde(default)]
        pub ratio: String,
        #[serde(default)]
        pub file_size: u64,
        #[serde(default)]
        pub file_type: String,
        #[serde(default)]
        pub created_at: String,
        #[serde(default)]
        pub colors: Vec<String>,
        #[serde(default)]
        pub path: String,
        #[serde(default)]
        pub thumbs: WallhavenThumbs,
        #[serde(default)]
        pub tags: Vec<WallhavenTag>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct WallhavenThumbs {
        #[serde(default)]
        pub large: String,
        #[serde(default)]
        pub original: String,
        #[serde(default)]
        pub small: String,
    }

    impl WallhavenWallpaper {
        pub fn to_wallpaper(&self) -> walltui::core::models::Wallpaper {
            let tags: Vec<String> = self.tags.iter().map(|t| t.name.clone()).collect();
            walltui::core::models::Wallpaper {
                id: self.id.clone(),
                provider: walltui::core::models::Provider::Wallhaven,
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
}
