use walltui::core::models::{Orientation, Provider, SearchQuery, Wallpaper};

fn sample_wallpaper() -> Wallpaper {
    Wallpaper {
        id: "abc-123".to_string(),
        provider: Provider::Pixabay,
        url: "https://example.com/full.jpg".to_string(),
        thumb_url: "https://example.com/thumb.jpg".to_string(),
        title: "Test Image".to_string(),
        photographer: "Test User".to_string(),
        width: Some(1920),
        height: Some(1080),
        avg_color: Some("#ff0000".to_string()),
        attribution: None,
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
}

#[test]
fn search_query_builder_with_all_fields() {
    let query = SearchQuery::builder("city")
        .provider(Provider::Unsplash)
        .page(2)
        .per_page(50)
        .orientation(Orientation::Landscape)
        .color("black")
        .build();

    assert_eq!(query.query, "city");
    assert_eq!(query.provider, Some(Provider::Unsplash));
    assert_eq!(query.page, 2);
    assert_eq!(query.per_page, 50);
    assert_eq!(query.orientation, Some(Orientation::Landscape));
    assert_eq!(query.color, Some("black".to_string()));
}

#[test]
fn provider_display() {
    assert_eq!(format!("{}", Provider::Unsplash), "Unsplash");
    assert_eq!(format!("{}", Provider::Pexels), "Pexels");
    assert_eq!(format!("{}", Provider::Pixabay), "Pixabay");
    assert_eq!(format!("{}", Provider::Commons), "Commons");
}

#[test]
fn provider_from_str() {
    assert_eq!("unsplash".parse::<Provider>().unwrap(), Provider::Unsplash);
    assert_eq!("PEXELS".parse::<Provider>().unwrap(), Provider::Pexels);
    assert_eq!("Pixabay".parse::<Provider>().unwrap(), Provider::Pixabay);
    assert_eq!("commons".parse::<Provider>().unwrap(), Provider::Commons);
    assert!("unknown".parse::<Provider>().is_err());
}
