//! LRU eviction cache with capacity limits, hit/miss tracking.

use crate::entry::CacheEntry;

/// An LRU cache that evicts the least recently used entries when at capacity.
#[derive(Debug)]
pub struct LruCache<V: Clone> {
    entries: Vec<CacheEntry<V>>,
    capacity: usize,
    default_ttl_ms: Option<u64>,
    hits: u64,
    misses: u64,
    evictions: u64,
    total_inserts: u64,
}

impl<V: Clone> LruCache<V> {
    /// Create a new LRU cache with the given capacity.
    pub fn new(capacity: usize, default_ttl_ms: Option<u64>) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
            default_ttl_ms,
            hits: 0,
            misses: 0,
            evictions: 0,
            total_inserts: 0,
        }
    }

    /// Get a value by key, marking it as most recently used.
    pub fn get(&mut self, key: &str, now_ms: u64) -> Option<V> {
        // First evict expired entries
        self.evict_expired(now_ms);

        if let Some(pos) = self.entries.iter().position(|e| e.key() == key) {
            let mut entry = self.entries.remove(pos);
            entry.touch(now_ms);
            let value = entry.value().clone();
            self.entries.push(entry);
            self.hits = self.hits.saturating_add(1);
            Some(value)
        } else {
            self.misses = self.misses.saturating_add(1);
            None
        }
    }

    /// Insert or update a key-value pair.
    /// Returns the evicted entry's key if an eviction occurred.
    pub fn put(&mut self, key: &str, value: V, now_ms: u64) -> Option<String> {
        self.evict_expired(now_ms);

        // If key exists, update it
        if let Some(pos) = self.entries.iter().position(|e| e.key() == key) {
            let mut entry = self.entries.remove(pos);
            entry.set_value(value);
            entry.touch(now_ms);
            self.entries.push(entry);
            self.total_inserts = self.total_inserts.saturating_add(1);
            return None;
        }

        // If at capacity, evict LRU (front of the Vec)
        let evicted_key = if self.entries.len() >= self.capacity && self.capacity > 0 {
            let evicted = self.entries.remove(0);
            self.evictions = self.evictions.saturating_add(1);
            Some(evicted.key().to_string())
        } else {
            None
        };

        let entry = CacheEntry::new(key.to_string(), value, now_ms, self.default_ttl_ms);
        self.entries.push(entry);
        self.total_inserts = self.total_inserts.saturating_add(1);
        evicted_key
    }

    /// Remove a key from the cache.
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.key() == key) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if a key exists (without updating access order).
    pub fn contains(&self, key: &str, now_ms: u64) -> bool {
        self.entries
            .iter()
            .any(|e| e.key() == key && !e.is_expired(now_ms))
    }

    /// Evict all expired entries.
    pub fn evict_expired(&mut self, now_ms: u64) {
        let before = self.entries.len();
        self.entries.retain(|e| !e.is_expired(now_ms));
        let removed = before - self.entries.len();
        self.evictions = self.evictions.saturating_add(removed as u64);
    }

    /// Current number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Cache capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Total cache hits.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Total cache misses.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Total evictions (LRU + TTL).
    pub fn evictions(&self) -> u64 {
        self.evictions
    }

    /// Total inserts.
    pub fn total_inserts(&self) -> u64 {
        self.total_inserts
    }

    /// Hit rate: hits / (hits + misses).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// Fill ratio: current entries / capacity.
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.entries.len() as f64 / self.capacity as f64
    }

    /// Clear the cache and reset stats.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
        self.total_inserts = 0;
    }

    /// Get keys in order from LRU (oldest) to MRU (newest).
    pub fn keys(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.key().to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cache() {
        let c: LruCache<u32> = LruCache::new(5, None);
        assert_eq!(c.capacity(), 5);
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut c = LruCache::new(5, None);
        c.put("a", 1, 1000);
        assert_eq!(c.get("a", 1000), Some(1));
        assert_eq!(c.hits(), 1);
    }

    #[test]
    fn test_miss() {
        let mut c: LruCache<u32> = LruCache::new(5, None);
        assert_eq!(c.get("missing", 1000), None);
        assert_eq!(c.misses(), 1);
    }

    #[test]
    fn test_lru_eviction() {
        let mut c = LruCache::new(3, None);
        c.put("a", 1, 100);
        c.put("b", 2, 200);
        c.put("c", 3, 300);
        // Cache full: [a, b, c]
        let evicted = c.put("d", 4, 400);
        assert_eq!(evicted, Some("a".to_string())); // LRU evicted
        assert_eq!(c.get("a", 500), None); // gone
        assert_eq!(c.get("d", 500), Some(4)); // new entry present
        assert_eq!(c.evictions(), 1);
    }

    #[test]
    fn test_access_promotes_to_mru() {
        let mut c = LruCache::new(3, None);
        c.put("a", 1, 100);
        c.put("b", 2, 200);
        c.put("c", 3, 300);
        // Access 'a' to promote it
        c.get("a", 350);
        // Now order is [b, c, a]. Insert d evicts b (LRU)
        let evicted = c.put("d", 4, 400);
        assert_eq!(evicted, Some("b".to_string()));
        assert!(c.contains("a", 400)); // a still present
    }

    #[test]
    fn test_ttl_eviction() {
        let mut c = LruCache::new(5, Some(2000));
        c.put("a", 1, 1000);
        c.put("b", 2, 2000);
        assert!(c.contains("a", 2000)); // not expired yet
        assert!(!c.contains("a", 3000)); // expired (1000 + 2000 = 3000)
        c.evict_expired(3000);
        assert_eq!(c.len(), 1); // only b remains
    }

    #[test]
    fn test_update_existing() {
        let mut c = LruCache::new(5, None);
        c.put("a", 1, 100);
        c.put("a", 2, 200);
        assert_eq!(c.get("a", 300), Some(2));
        assert_eq!(c.len(), 1); // no duplicate
        assert_eq!(c.total_inserts(), 2);
    }

    #[test]
    fn test_remove() {
        let mut c = LruCache::new(5, None);
        c.put("a", 1, 100);
        assert!(c.remove("a"));
        assert!(!c.remove("a")); // already removed
        assert!(c.is_empty());
    }

    #[test]
    fn test_hit_rate() {
        let mut c = LruCache::new(5, None);
        c.put("a", 1, 100);
        c.get("a", 200); // hit
        c.get("b", 300); // miss
        assert!((c.hit_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fill_ratio() {
        let mut c = LruCache::new(4, None);
        c.put("a", 1, 100);
        c.put("b", 2, 200);
        assert!((c.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clear() {
        let mut c = LruCache::new(5, None);
        c.put("a", 1, 100);
        c.put("b", 2, 200);
        c.get("a", 300);
        c.clear();
        assert!(c.is_empty());
        assert_eq!(c.hits(), 0);
        assert_eq!(c.misses(), 0);
    }

    #[test]
    fn test_keys_order() {
        let mut c = LruCache::new(5, None);
        c.put("x", 1, 100);
        c.put("y", 2, 200);
        c.put("z", 3, 300);
        // Access x to move it to MRU
        c.get("x", 350);
        let keys = c.keys();
        assert_eq!(keys, vec!["y", "z", "x"]); // LRU to MRU
    }
}
