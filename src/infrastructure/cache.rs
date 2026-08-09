use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

pub struct TtlCache<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    ttl: Duration,
}

impl<K: Eq + Hash, V> TtlCache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + self.ttl,
        });
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
            .filter(|entry| entry.expires_at > Instant::now())
            .map(|entry| &entry.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|e| e.value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.iter()
            .filter(|(_, e)| e.expires_at > Instant::now())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn purge_expired(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| e.expires_at > Instant::now());
        before - self.entries.len()
    }
}
