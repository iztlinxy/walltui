use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

pub struct TtlCache<K, V, C = fn() -> Instant> {
    entries: HashMap<K, CacheEntry<V>>,
    ttl: Duration,
    clock: C,
}

impl<K: Eq + Hash, V> TtlCache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self::with_clock(ttl, Instant::now)
    }
}

impl<K: Eq + Hash, V, C: Fn() -> Instant> TtlCache<K, V, C> {
    pub fn with_clock(ttl: Duration, clock: C) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
            clock,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let now = (self.clock)();
        self.entries.insert(
            key,
            CacheEntry {
                value,
                expires_at: now + self.ttl,
            },
        );
    }

    fn now(&self) -> Instant {
        (self.clock)()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let now = self.now();
        self.entries
            .get(key)
            .filter(|entry| entry.expires_at > now)
            .map(|entry| &entry.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|e| e.value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        let now = self.now();
        self.entries
            .iter()
            .filter(|(_, e)| e.expires_at > now)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn purge_expired(&mut self) -> usize {
        let now = self.now();
        let before = self.entries.len();
        self.entries.retain(|_, e| e.expires_at > now);
        before - self.entries.len()
    }
}
