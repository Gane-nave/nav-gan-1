//! Caching strategies — LRU cache, TTL-based cache, multi-tier cache.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// LRU (Least Recently Used) cache with configurable capacity.
pub struct LruCache<V> {
    capacity: usize,
    entries: RwLock<HashMap<String, V>>,
    order: RwLock<VecDeque<String>>,
    hits: RwLock<u64>,
    misses: RwLock<u64>,
}

impl<V: Clone> LruCache<V> {
    /// Create a new LRU cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Cache capacity must be > 0");
        Self {
            capacity,
            entries: RwLock::new(HashMap::new()),
            order: RwLock::new(VecDeque::new()),
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        }
    }

    /// Get a value from the cache.
    pub fn get(&self, key: &str) -> Option<V> {
        let entries = self.entries.read();
        if let Some(value) = entries.get(key) {
            *self.hits.write() += 1;
            let mut order = self.order.write();
            order.retain(|k| k != key);
            order.push_back(key.to_string());
            Some(value.clone())
        } else {
            *self.misses.write() += 1;
            None
        }
    }

    /// Insert a value into the cache.
    pub fn insert(&self, key: &str, value: V) {
        let mut entries = self.entries.write();
        let mut order = self.order.write();

        // If key already exists, update it
        if entries.contains_key(key) {
            entries.insert(key.to_string(), value);
            order.retain(|k| k != key);
            order.push_back(key.to_string());
            return;
        }

        // Evict if at capacity
        while entries.len() >= self.capacity {
            if let Some(evicted) = order.pop_front() {
                entries.remove(&evicted);
            } else {
                break;
            }
        }

        entries.insert(key.to_string(), value);
        order.push_back(key.to_string());
    }

    /// Remove a value from the cache.
    pub fn remove(&self, key: &str) -> Option<V> {
        let mut entries = self.entries.write();
        let mut order = self.order.write();
        order.retain(|k| k != key);
        entries.remove(key)
    }

    /// Clear the cache.
    pub fn clear(&self) {
        self.entries.write().clear();
        self.order.write().clear();
    }

    /// Current number of entries.
    pub fn len(&self) -> usize {
        self.entries.read().len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.read().is_empty()
    }

    /// Cache hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let hits = *self.hits.read();
        let misses = *self.misses.read();
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    /// Cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.len(),
            capacity: self.capacity,
            hits: *self.hits.read(),
            misses: *self.misses.read(),
            hit_rate: self.hit_rate(),
        }
    }
}

/// TTL (Time-To-Live) cache entry.
struct TtlEntry<V> {
    value: V,
    expires_at: Instant,
}

/// TTL-based cache — entries expire after a configurable duration.
pub struct TtlCache<V> {
    entries: RwLock<HashMap<String, TtlEntry<V>>>,
    default_ttl: Duration,
    max_size: usize,
    hits: RwLock<u64>,
    misses: RwLock<u64>,
}

impl<V: Clone> TtlCache<V> {
    /// Create a new TTL cache.
    pub fn new(default_ttl: Duration, max_size: usize) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            default_ttl,
            max_size,
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        }
    }

    /// Get a value if it exists and hasn't expired.
    pub fn get(&self, key: &str) -> Option<V> {
        let entries = self.entries.read();
        if let Some(entry) = entries.get(key) {
            if Instant::now() < entry.expires_at {
                *self.hits.write() += 1;
                return Some(entry.value.clone());
            }
        }
        *self.misses.write() += 1;
        None
    }

    /// Insert a value with the default TTL.
    pub fn insert(&self, key: &str, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
    }

    /// Insert a value with a custom TTL.
    pub fn insert_with_ttl(&self, key: &str, value: V, ttl: Duration) {
        let mut entries = self.entries.write();

        // Evict expired entries first
        let now = Instant::now();
        entries.retain(|_, e| e.expires_at > now);

        // Evict oldest if still at capacity
        if entries.len() >= self.max_size && !entries.contains_key(key) {
            if let Some(oldest_key) = entries
                .iter()
                .min_by_key(|(_, e)| e.expires_at)
                .map(|(k, _)| k.clone())
            {
                entries.remove(&oldest_key);
            }
        }

        entries.insert(
            key.to_string(),
            TtlEntry {
                value,
                expires_at: now + ttl,
            },
        );
    }

    /// Remove an entry.
    pub fn remove(&self, key: &str) {
        self.entries.write().remove(key);
    }

    /// Purge all expired entries.
    pub fn purge_expired(&self) -> usize {
        let mut entries = self.entries.write();
        let before = entries.len();
        let now = Instant::now();
        entries.retain(|_, e| e.expires_at > now);
        before - entries.len()
    }

    /// Current size (including possibly expired entries).
    pub fn len(&self) -> usize {
        self.entries.read().len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.read().is_empty()
    }

    /// Hit rate.
    pub fn hit_rate(&self) -> f64 {
        let hits = *self.hits.read();
        let misses = *self.misses.read();
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub size: usize,
    pub capacity: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

/// Cache tier configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheTierConfig {
    pub name: String,
    pub capacity: usize,
    pub ttl_secs: u64,
    pub priority: u32,
}

/// Multi-tier cache — combines L1 (fast/small) and L2 (slow/large) caches.
pub struct MultiTierCache<V: Clone> {
    l1: LruCache<V>,
    l2: LruCache<V>,
}

impl<V: Clone> MultiTierCache<V> {
    /// Create a new multi-tier cache.
    pub fn new(l1_capacity: usize, l2_capacity: usize) -> Self {
        Self {
            l1: LruCache::new(l1_capacity),
            l2: LruCache::new(l2_capacity),
        }
    }

    /// Get from L1, falling back to L2 (and promoting to L1).
    pub fn get(&self, key: &str) -> Option<V> {
        if let Some(value) = self.l1.get(key) {
            return Some(value);
        }
        if let Some(value) = self.l2.get(key) {
            // Promote to L1
            self.l1.insert(key, value.clone());
            return Some(value);
        }
        None
    }

    /// Insert into both tiers.
    pub fn insert(&self, key: &str, value: V) {
        self.l1.insert(key, value.clone());
        self.l2.insert(key, value);
    }

    /// Remove from both tiers.
    pub fn remove(&self, key: &str) {
        self.l1.remove(key);
        self.l2.remove(key);
    }

    /// Get stats for both tiers.
    pub fn stats(&self) -> (CacheStats, CacheStats) {
        (self.l1.stats(), self.l2.stats())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        let cache = LruCache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        assert_eq!(cache.get("a"), Some(1));
        assert_eq!(cache.get("b"), Some(2));
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_lru_cache_eviction() {
        let cache = LruCache::new(2);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3); // Should evict "a"

        assert_eq!(cache.get("a"), None);
        assert_eq!(cache.get("b"), Some(2));
        assert_eq!(cache.get("c"), Some(3));
    }

    #[test]
    fn test_lru_cache_access_refreshes() {
        let cache = LruCache::new(2);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.get("a"); // Refresh "a"
        cache.insert("c", 3); // Should evict "b", not "a"

        assert_eq!(cache.get("a"), Some(1));
        assert_eq!(cache.get("b"), None);
        assert_eq!(cache.get("c"), Some(3));
    }

    #[test]
    fn test_lru_cache_update() {
        let cache = LruCache::new(2);
        cache.insert("a", 1);
        cache.insert("a", 10);
        assert_eq!(cache.get("a"), Some(10));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_lru_cache_remove() {
        let cache = LruCache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        assert_eq!(cache.remove("a"), Some(1));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_lru_cache_hit_rate() {
        let cache = LruCache::new(2);
        cache.insert("a", 1);
        cache.get("a"); // hit
        cache.get("b"); // miss
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_ttl_cache_basic() {
        let cache = TtlCache::new(Duration::from_secs(60), 100);
        cache.insert("a", 1);
        assert_eq!(cache.get("a"), Some(1));
    }

    #[test]
    fn test_ttl_cache_expired() {
        let cache = TtlCache::new(Duration::from_millis(1), 100);
        cache.insert("a", 1);
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(cache.get("a"), None);
    }

    #[test]
    fn test_ttl_cache_purge() {
        let cache = TtlCache::new(Duration::from_millis(1), 100);
        cache.insert("a", 1);
        cache.insert("b", 2);
        std::thread::sleep(Duration::from_millis(10));
        let purged = cache.purge_expired();
        assert_eq!(purged, 2);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_ttl_cache_max_size() {
        let cache = TtlCache::new(Duration::from_secs(60), 2);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3); // Should evict oldest
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_multi_tier_cache() {
        let cache = MultiTierCache::new(2, 4);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // All should be in L2
        assert_eq!(cache.get("a"), Some(1));
        assert_eq!(cache.get("c"), Some(3));
    }

    #[test]
    fn test_multi_tier_l2_promotion() {
        let cache = MultiTierCache::new(1, 4);
        cache.insert("a", 1);
        cache.insert("b", 2); // "a" evicted from L1 but still in L2

        // Get "a" — should promote from L2 to L1
        assert_eq!(cache.get("a"), Some(1));
        // "a" now in L1
        let (l1_stats, _) = cache.stats();
        assert_eq!(l1_stats.size, 1);
    }

    #[test]
    fn test_multi_tier_remove() {
        let cache = MultiTierCache::new(2, 4);
        cache.insert("a", 1);
        cache.remove("a");
        assert_eq!(cache.get("a"), None);
    }

    #[test]
    fn test_cache_stats() {
        let cache = LruCache::new(10);
        cache.insert("a", 1);
        cache.get("a");
        cache.get("missing");
        let stats = cache.stats();
        assert_eq!(stats.size, 1);
        assert_eq!(stats.capacity, 10);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
