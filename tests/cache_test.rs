use std::time::Duration;

use walltui::infrastructure::cache::TtlCache;

#[test]
fn cache_insert_and_get() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("key1", "value1");
    assert_eq!(cache.get(&"key1"), Some(&"value1"));
}

#[test]
fn cache_get_missing_key() {
    let cache: TtlCache<&str, &str> = TtlCache::new(Duration::from_secs(60));
    assert_eq!(cache.get(&"nope"), None);
}

#[test]
fn cache_overwrite_value() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("k", "v1");
    cache.insert("k", "v2");
    assert_eq!(cache.get(&"k"), Some(&"v2"));
}

#[test]
fn cache_remove() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("k", 42);
    let removed = cache.remove(&"k");
    assert_eq!(removed, Some(42));
    assert_eq!(cache.get(&"k"), None);
}

#[test]
fn cache_remove_missing() {
    let mut cache: TtlCache<&str, i32> = TtlCache::new(Duration::from_secs(60));
    assert_eq!(cache.remove(&"nope"), None);
}

#[test]
fn cache_contains_key() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("yes", true);
    assert!(cache.contains_key(&"yes"));
    assert!(!cache.contains_key(&"no"));
}

#[test]
fn cache_len_and_empty() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
    cache.insert("a", 1);
    cache.insert("b", 2);
    assert_eq!(cache.len(), 2);
    assert!(!cache.is_empty());
}

#[test]
fn cache_clear() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("a", 1);
    cache.insert("b", 2);
    cache.clear();
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn cache_expires_after_ttl() {
    let mut cache = TtlCache::new(Duration::from_millis(50));
    cache.insert("temp", "gone");
    assert_eq!(cache.get(&"temp"), Some(&"gone"));
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cache.get(&"temp"), None);
}

#[test]
fn cache_len_excludes_expired() {
    let mut cache = TtlCache::new(Duration::from_millis(50));
    cache.insert("a", 1);
    cache.insert("b", 2);
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn cache_contains_key_false_after_expiry() {
    let mut cache = TtlCache::new(Duration::from_millis(50));
    cache.insert("x", true);
    std::thread::sleep(Duration::from_millis(100));
    assert!(!cache.contains_key(&"x"));
}

#[test]
fn cache_purge_expired() {
    let mut cache = TtlCache::new(Duration::from_millis(50));
    cache.insert("fast", 1);
    cache.insert("slow", 2);
    std::thread::sleep(Duration::from_millis(100));
    cache.insert("fresh", 3);
    let purged = cache.purge_expired();
    assert_eq!(purged, 2);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&"fresh"), Some(&3));
}

#[test]
fn cache_purge_expired_none_when_fresh() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("a", 1);
    cache.insert("b", 2);
    let purged = cache.purge_expired();
    assert_eq!(purged, 0);
    assert_eq!(cache.len(), 2);
}

#[test]
fn cache_mixed_expiry_survivors() {
    let mut cache = TtlCache::new(Duration::from_millis(50));
    cache.insert("e1", 1);
    cache.insert("e2", 2);
    std::thread::sleep(Duration::from_millis(100));
    cache.insert("ok1", 10);
    cache.insert("ok2", 20);
    std::thread::sleep(Duration::from_millis(30));
    assert_eq!(cache.get(&"e1"), None);
    assert_eq!(cache.get(&"e2"), None);
    assert_eq!(cache.get(&"ok1"), Some(&10));
    assert_eq!(cache.get(&"ok2"), Some(&20));
}

#[test]
fn cache_different_ttls() {
    let mut short = TtlCache::new(Duration::from_millis(20));
    let mut long = TtlCache::new(Duration::from_secs(60));
    short.insert("k", "short");
    long.insert("k", "long");
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(short.get(&"k"), None);
    assert_eq!(long.get(&"k"), Some(&"long"));
}

#[test]
fn cache_string_keys() {
    let mut cache = TtlCache::new(Duration::from_secs(60));
    cache.insert("wallpaper_123".to_string(), vec![1, 2, 3]);
    assert_eq!(
        cache.get(&"wallpaper_123".to_string()),
        Some(&vec![1, 2, 3])
    );
}
