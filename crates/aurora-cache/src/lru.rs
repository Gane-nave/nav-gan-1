//! LRU (Least Recently Used) cache — fixed-capacity cache with eviction.

use std::collections::HashMap;

/// Entry in the LRU cache.
#[derive(Debug, Clone)]
struct LruEntry<V> {
    value: V,
    /// Access counter (higher = more recently used).
    access_order: u64,
    /// Byte size of the value.
    byte_size: usize,
}

/// LRU cache with fixed capacity.
pub struct LruCache<V: Clone> {
    entries: HashMap<String, LruEntry<V>>,
    max_entries: usize,
    access_counter: u64,
    hits: u64,
    misses: u64,
    evictions: u64,
}

impl<V: Clone> LruCache<V> {
    /// Create a new LRU cache with a maximum number of entries.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
            access_counter: 0,
            hits: 0,
            misses: 0,
            evictions: 0,
        }
    }

    /// Get a value from the cache, updating access order.
    pub fn get(&mut self, key: &str) -> Option<V> {
        self.access_counter += 1;
        if let Some(entry) = self.entries.get_mut(key) {
            entry.access_order = self.access_counter;
            self.hits += 1;
            Some(entry.value.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    /// Peek at a value without updating access order.
    pub fn peek(&self, key: &str) -> Option<&V> {
        self.entries.get(key).map(|e| &e.value)
    }

    /// Insert a key-value pair, evicting the LRU entry if at capacity.
    /// Returns the evicted key if one was removed.
    pub fn insert(&mut self, key: String, value: V, byte_size: usize) -> Option<String> {
        self.access_counter += 1;

        // If key already exists, update in place
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.value = value;
            entry.access_order = self.access_counter;
            entry.byte_size = byte_size;
            return None;
        }

        // If at capacity, evict LRU entry
        let evicted = if self.entries.len() >= self.max_entries {
            let lru_key = self
                .entries
                .iter()
                .min_by_key(|(_, e)| e.access_order)
                .map(|(k, _)| k.clone());
            if let Some(ref k) = lru_key {
                self.entries.remove(k);
                self.evictions += 1;
            }
            lru_key
        } else {
            None
        };

        self.entries.insert(
            key,
            LruEntry {
                value,
                access_order: self.access_counter,
                byte_size,
            },
        );

        evicted
    }

    /// Remove a key from the cache.
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.entries.remove(key).map(|e| e.value)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get cache hit count.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Get cache miss count.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Get eviction count.
    pub fn evictions(&self) -> u64 {
        self.evictions
    }

    /// Get hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }

    /// Get total byte size of cached values.
    pub fn total_bytes(&self) -> usize {
        self.entries.values().map(|e| e.byte_size).sum()
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
        let mut cache = LruCache::new(3);
        cache.insert("k1".to_string(), "v1".to_string(), 2);
        assert_eq!(cache.get("k1"), Some("v1".to_string()));
        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 0);
    }

    #[test]
    fn test_miss() {
        let mut cache: LruCache<String> = LruCache::new(3);
        assert_eq!(cache.get("missing"), None);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_eviction_on_capacity() {
        let mut cache = LruCache::new(2);
        cache.insert("k1".to_string(), 1, 10);
        cache.insert("k2".to_string(), 2, 10);
        let evicted = cache.insert("k3".to_string(), 3, 10);
        assert!(evicted.is_some());
        assert_eq!(evicted.unwrap(), "k1"); // k1 is LRU (oldest access)
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.evictions(), 1);
    }

    #[test]
    fn test_access_updates_lru_order() {
        let mut cache = LruCache::new(2);
        cache.insert("k1".to_string(), 1, 10);
        cache.insert("k2".to_string(), 2, 10);
        // Access k1 to make it more recent than k2
        cache.get("k1");
        let evicted = cache.insert("k3".to_string(), 3, 10);
        assert_eq!(evicted, Some("k2".to_string())); // k2 is now LRU
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = LruCache::new(2);
        cache.insert("k1".to_string(), 1, 10);
        let evicted = cache.insert("k1".to_string(), 100, 20);
        assert!(evicted.is_none()); // no eviction for update
        assert_eq!(cache.get("k1"), Some(100));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut cache = LruCache::new(3);
        cache.insert("k1".to_string(), "v1".to_string(), 2);
        let removed = cache.remove("k1");
        assert_eq!(removed, Some("v1".to_string()));
        assert!(!cache.contains("k1"));
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = LruCache::new(3);
        cache.insert("k1".to_string(), 1, 10);
        cache.get("k1"); // hit
        cache.get("k1"); // hit
        cache.get("missing"); // miss
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_total_bytes() {
        let mut cache = LruCache::new(5);
        cache.insert("k1".to_string(), 1, 100);
        cache.insert("k2".to_string(), 2, 200);
        assert_eq!(cache.total_bytes(), 300);
    }

    #[test]
    fn test_clear() {
        let mut cache = LruCache::new(5);
        cache.insert("k1".to_string(), 1, 10);
        cache.insert("k2".to_string(), 2, 10);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_peek_does_not_update_order() {
        let mut cache = LruCache::new(2);
        cache.insert("k1".to_string(), 1, 10);
        cache.insert("k2".to_string(), 2, 10);
        // Peek at k1 — should NOT update access order
        assert_eq!(cache.peek("k1"), Some(&1));
        // Insert k3 — should evict k1 (still LRU since peek didn't update)
        let evicted = cache.insert("k3".to_string(), 3, 10);
        assert_eq!(evicted, Some("k1".to_string()));
    }
}
