//! TTL (Time-To-Live) cache — entries expire after a configurable duration.

use std::collections::HashMap;

/// Entry in the TTL cache.
#[derive(Debug, Clone)]
struct TtlEntry<V> {
    value: V,
    /// When the entry was created (epoch millis).
    created_at_ms: u64,
    /// Time-to-live in milliseconds.
    ttl_ms: u64,
    /// Access count.
    access_count: u64,
}

impl<V> TtlEntry<V> {
    /// Whether this entry has expired at the given time.
    fn is_expired(&self, now_ms: u64) -> bool {
        now_ms >= self.created_at_ms + self.ttl_ms
    }
}

/// TTL cache with automatic expiration.
pub struct TtlCache<V: Clone> {
    entries: HashMap<String, TtlEntry<V>>,
    default_ttl_ms: u64,
    max_entries: usize,
    hits: u64,
    misses: u64,
    expirations: u64,
}

impl<V: Clone> TtlCache<V> {
    /// Create a new TTL cache.
    pub fn new(max_entries: usize, default_ttl_ms: u64) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl_ms,
            max_entries,
            hits: 0,
            misses: 0,
            expirations: 0,
        }
    }

    /// Insert a value with the default TTL.
    pub fn insert(&mut self, key: String, value: V, now_ms: u64) -> Result<(), String> {
        self.insert_with_ttl(key, value, now_ms, self.default_ttl_ms)
    }

    /// Insert a value with a custom TTL.
    pub fn insert_with_ttl(
        &mut self,
        key: String,
        value: V,
        now_ms: u64,
        ttl_ms: u64,
    ) -> Result<(), String> {
        // Remove expired entries first to free space
        self.evict_expired(now_ms);

        if !self.entries.contains_key(&key) && self.entries.len() >= self.max_entries {
            return Err(format!("Cache full: max {} entries", self.max_entries));
        }

        self.entries.insert(
            key,
            TtlEntry {
                value,
                created_at_ms: now_ms,
                ttl_ms,
                access_count: 0,
            },
        );
        Ok(())
    }

    /// Get a value, returning None if expired or missing.
    pub fn get(&mut self, key: &str, now_ms: u64) -> Option<V> {
        // Check if entry exists and is not expired
        let expired = self
            .entries
            .get(key)
            .map(|e| e.is_expired(now_ms))
            .unwrap_or(false);

        if expired {
            self.entries.remove(key);
            self.expirations += 1;
            self.misses += 1;
            return None;
        }

        if let Some(entry) = self.entries.get_mut(key) {
            entry.access_count += 1;
            self.hits += 1;
            Some(entry.value.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    /// Remove expired entries.
    pub fn evict_expired(&mut self, now_ms: u64) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| !e.is_expired(now_ms));
        let evicted = before - self.entries.len();
        self.expirations += evicted as u64;
        evicted
    }

    /// Remove a specific key.
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.entries.remove(key).map(|e| e.value)
    }

    /// Get the number of entries (may include expired).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Check if a key exists and is not expired.
    pub fn contains(&self, key: &str, now_ms: u64) -> bool {
        self.entries.get(key).is_some_and(|e| !e.is_expired(now_ms))
    }

    /// Get remaining TTL for a key in milliseconds.
    pub fn remaining_ttl(&self, key: &str, now_ms: u64) -> Option<u64> {
        self.entries.get(key).and_then(|e| {
            let expires_at = e.created_at_ms + e.ttl_ms;
            if now_ms >= expires_at {
                None
            } else {
                Some(expires_at - now_ms)
            }
        })
    }

    /// Get hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// Get total expirations.
    pub fn expirations(&self) -> u64 {
        self.expirations
    }

    /// Get hits count.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Get misses count.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut cache = TtlCache::new(10, 5000);
        cache
            .insert("k1".to_string(), "v1".to_string(), 1000)
            .unwrap();
        assert_eq!(cache.get("k1", 2000), Some("v1".to_string()));
        assert_eq!(cache.hits(), 1);
    }

    #[test]
    fn test_expiration() {
        let mut cache = TtlCache::new(10, 1000);
        cache
            .insert("k1".to_string(), "v1".to_string(), 1000)
            .unwrap();
        // Not expired at 1999
        assert!(cache.contains("k1", 1999));
        // Expired at 2000
        assert!(!cache.contains("k1", 2000));
        assert_eq!(cache.get("k1", 2000), None);
        assert_eq!(cache.expirations(), 1);
    }

    #[test]
    fn test_custom_ttl() {
        let mut cache = TtlCache::new(10, 5000);
        cache
            .insert_with_ttl("k1".to_string(), "v1".to_string(), 1000, 100)
            .unwrap();
        assert_eq!(cache.get("k1", 1099), Some("v1".to_string()));
        assert_eq!(cache.get("k1", 1100), None);
    }

    #[test]
    fn test_remaining_ttl() {
        let mut cache = TtlCache::new(10, 5000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        assert_eq!(cache.remaining_ttl("k1", 3000), Some(3000)); // 1000+5000-3000
        assert_eq!(cache.remaining_ttl("k1", 6000), None); // expired
        assert_eq!(cache.remaining_ttl("missing", 1000), None);
    }

    #[test]
    fn test_evict_expired() {
        let mut cache = TtlCache::new(10, 1000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        cache.insert("k2".to_string(), 2, 1000).unwrap();
        cache.insert("k3".to_string(), 3, 1600).unwrap();
        // At 2500: k1 expired (1000+1000=2000 <= 2500), k2 expired (1000+1000=2000 <= 2500),
        //          k3 alive  (1600+1000=2600 > 2500)
        let evicted = cache.evict_expired(2500);
        assert_eq!(evicted, 2);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_full() {
        let mut cache = TtlCache::new(2, 5000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        cache.insert("k2".to_string(), 2, 1000).unwrap();
        let err = cache.insert("k3".to_string(), 3, 1000).unwrap_err();
        assert!(err.contains("Cache full"));
    }

    #[test]
    fn test_cache_full_with_expired_eviction() {
        let mut cache = TtlCache::new(2, 1000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        cache.insert("k2".to_string(), 2, 1000).unwrap();
        // Insert at 3000 — k1 and k2 are expired (1000+1000=2000 < 3000)
        // evict_expired runs first, so insert succeeds
        cache.insert("k3".to_string(), 3, 3000).unwrap();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get("k3", 3000), Some(3));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = TtlCache::new(2, 5000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        cache.insert("k1".to_string(), 100, 2000).unwrap();
        assert_eq!(cache.get("k1", 2000), Some(100));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut cache = TtlCache::new(10, 5000);
        cache
            .insert("k1".to_string(), "v1".to_string(), 1000)
            .unwrap();
        let removed = cache.remove("k1");
        assert_eq!(removed, Some("v1".to_string()));
        assert!(cache.is_empty());
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = TtlCache::new(10, 5000);
        cache.insert("k1".to_string(), 1, 1000).unwrap();
        cache.get("k1", 1000); // hit
        cache.get("k1", 1000); // hit
        cache.get("missing", 1000); // miss
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 0.01);
    }
}
