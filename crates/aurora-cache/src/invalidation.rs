//! Cache invalidation — tag-based, pattern-based, and cascade invalidation.

use std::collections::{HashMap, HashSet};

/// Invalidation strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationStrategy {
    /// Invalidate by exact key.
    ExactKey(String),
    /// Invalidate all keys with a given tag.
    ByTag(String),
    /// Invalidate all keys matching a prefix.
    ByPrefix(String),
    /// Invalidate everything.
    All,
}

/// Invalidation event record.
#[derive(Debug, Clone)]
pub struct InvalidationEvent {
    /// Strategy used.
    pub strategy: InvalidationStrategy,
    /// Number of keys invalidated.
    pub keys_invalidated: usize,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
}

/// Tag-based invalidation manager.
pub struct InvalidationManager {
    /// Key → set of tags.
    key_tags: HashMap<String, HashSet<String>>,
    /// Tag → set of keys.
    tag_keys: HashMap<String, HashSet<String>>,
    /// History of invalidation events.
    events: Vec<InvalidationEvent>,
    /// Total keys invalidated.
    total_invalidated: u64,
}

impl InvalidationManager {
    /// Create a new invalidation manager.
    pub fn new() -> Self {
        Self {
            key_tags: HashMap::new(),
            tag_keys: HashMap::new(),
            events: Vec::new(),
            total_invalidated: 0,
        }
    }

    /// Register a key with a set of tags.
    pub fn register(&mut self, key: &str, tags: &[&str]) {
        // Clean up old tags if key already exists to avoid stale reverse-index entries
        self.unregister(key);
        let tag_set: HashSet<String> = tags.iter().map(|t| t.to_string()).collect();
        for tag in &tag_set {
            self.tag_keys
                .entry(tag.clone())
                .or_default()
                .insert(key.to_string());
        }
        self.key_tags.insert(key.to_string(), tag_set);
    }

    /// Unregister a key (remove from all tags).
    pub fn unregister(&mut self, key: &str) {
        if let Some(tags) = self.key_tags.remove(key) {
            for tag in &tags {
                if let Some(keys) = self.tag_keys.get_mut(tag) {
                    keys.remove(key);
                    if keys.is_empty() {
                        self.tag_keys.remove(tag);
                    }
                }
            }
        }
    }

    /// Invalidate by exact key. Returns true if key existed.
    pub fn invalidate_key(&mut self, key: &str, timestamp_ms: u64) -> bool {
        let existed = self.key_tags.contains_key(key);
        if existed {
            self.unregister(key);
            self.total_invalidated += 1;
            self.events.push(InvalidationEvent {
                strategy: InvalidationStrategy::ExactKey(key.to_string()),
                keys_invalidated: 1,
                timestamp_ms,
            });
        }
        existed
    }

    /// Invalidate all keys with a given tag. Returns invalidated keys.
    pub fn invalidate_by_tag(&mut self, tag: &str, timestamp_ms: u64) -> Vec<String> {
        let keys: Vec<String> = self
            .tag_keys
            .get(tag)
            .map(|ks| ks.iter().cloned().collect())
            .unwrap_or_default();

        for key in &keys {
            self.unregister(key);
        }

        let count = keys.len();
        self.total_invalidated += count as u64;
        self.events.push(InvalidationEvent {
            strategy: InvalidationStrategy::ByTag(tag.to_string()),
            keys_invalidated: count,
            timestamp_ms,
        });

        keys
    }

    /// Invalidate all keys matching a prefix. Returns invalidated keys.
    pub fn invalidate_by_prefix(&mut self, prefix: &str, timestamp_ms: u64) -> Vec<String> {
        let keys: Vec<String> = self
            .key_tags
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();

        for key in &keys {
            self.unregister(key);
        }

        let count = keys.len();
        self.total_invalidated += count as u64;
        self.events.push(InvalidationEvent {
            strategy: InvalidationStrategy::ByPrefix(prefix.to_string()),
            keys_invalidated: count,
            timestamp_ms,
        });

        keys
    }

    /// Invalidate all keys. Returns count.
    pub fn invalidate_all(&mut self, timestamp_ms: u64) -> usize {
        let count = self.key_tags.len();
        self.key_tags.clear();
        self.tag_keys.clear();
        self.total_invalidated += count as u64;
        self.events.push(InvalidationEvent {
            strategy: InvalidationStrategy::All,
            keys_invalidated: count,
            timestamp_ms,
        });
        count
    }

    /// Get tags for a key.
    pub fn tags_for_key(&self, key: &str) -> Vec<String> {
        self.key_tags
            .get(key)
            .map(|tags| {
                let mut v: Vec<String> = tags.iter().cloned().collect();
                v.sort();
                v
            })
            .unwrap_or_default()
    }

    /// Get keys for a tag.
    pub fn keys_for_tag(&self, tag: &str) -> Vec<String> {
        self.tag_keys
            .get(tag)
            .map(|keys| {
                let mut v: Vec<String> = keys.iter().cloned().collect();
                v.sort();
                v
            })
            .unwrap_or_default()
    }

    /// Get total registered keys.
    pub fn key_count(&self) -> usize {
        self.key_tags.len()
    }

    /// Get total registered tags.
    pub fn tag_count(&self) -> usize {
        self.tag_keys.len()
    }

    /// Get total invalidation events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get total keys invalidated across all events.
    pub fn total_invalidated(&self) -> u64 {
        self.total_invalidated
    }

    /// Get recent invalidation events.
    pub fn recent_events(&self, limit: usize) -> &[InvalidationEvent] {
        let start = self.events.len().saturating_sub(limit);
        &self.events[start..]
    }
}

impl Default for InvalidationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_tags() {
        let mut mgr = InvalidationManager::new();
        mgr.register("route:123", &["route", "region:us"]);
        assert_eq!(mgr.tags_for_key("route:123"), vec!["region:us", "route"]);
        assert_eq!(mgr.keys_for_tag("route"), vec!["route:123"]);
    }

    #[test]
    fn test_invalidate_by_key() {
        let mut mgr = InvalidationManager::new();
        mgr.register("k1", &["tag1"]);
        assert!(mgr.invalidate_key("k1", 1000));
        assert_eq!(mgr.key_count(), 0);
        assert!(!mgr.invalidate_key("k1", 2000)); // already removed
    }

    #[test]
    fn test_invalidate_by_tag() {
        let mut mgr = InvalidationManager::new();
        mgr.register("k1", &["route"]);
        mgr.register("k2", &["route", "tile"]);
        mgr.register("k3", &["tile"]);

        let invalidated = mgr.invalidate_by_tag("route", 1000);
        assert_eq!(invalidated.len(), 2); // k1 and k2
        assert_eq!(mgr.key_count(), 1); // only k3 remains
        assert_eq!(mgr.keys_for_tag("tile"), vec!["k3"]);
    }

    #[test]
    fn test_invalidate_by_prefix() {
        let mut mgr = InvalidationManager::new();
        mgr.register("route:100", &["route"]);
        mgr.register("route:200", &["route"]);
        mgr.register("tile:300", &["tile"]);

        let invalidated = mgr.invalidate_by_prefix("route:", 1000);
        assert_eq!(invalidated.len(), 2);
        assert_eq!(mgr.key_count(), 1);
    }

    #[test]
    fn test_invalidate_all() {
        let mut mgr = InvalidationManager::new();
        mgr.register("k1", &["a"]);
        mgr.register("k2", &["b"]);
        mgr.register("k3", &["c"]);
        let count = mgr.invalidate_all(1000);
        assert_eq!(count, 3);
        assert_eq!(mgr.key_count(), 0);
        assert_eq!(mgr.tag_count(), 0);
    }

    #[test]
    fn test_unregister_cleans_tags() {
        let mut mgr = InvalidationManager::new();
        mgr.register("k1", &["shared"]);
        mgr.register("k2", &["shared"]);
        mgr.unregister("k1");
        assert_eq!(mgr.keys_for_tag("shared"), vec!["k2"]);
        mgr.unregister("k2");
        assert_eq!(mgr.tag_count(), 0); // "shared" tag removed since no keys
    }

    #[test]
    fn test_event_history() {
        let mut mgr = InvalidationManager::new();
        mgr.register("k1", &["a"]);
        mgr.register("k2", &["a"]);
        mgr.invalidate_key("k1", 1000);
        mgr.invalidate_by_tag("a", 2000);
        assert_eq!(mgr.event_count(), 2);
        assert_eq!(mgr.total_invalidated(), 2); // 1 by key + 1 by tag
    }

    #[test]
    fn test_recent_events() {
        let mut mgr = InvalidationManager::new();
        for i in 0..5 {
            mgr.register(&format!("k{i}"), &["t"]);
            mgr.invalidate_key(&format!("k{i}"), i as u64 * 1000);
        }
        let recent = mgr.recent_events(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_cascade_invalidation() {
        // Register keys with overlapping tags
        let mut mgr = InvalidationManager::new();
        mgr.register("route:us:101", &["route", "region:us"]);
        mgr.register("route:us:102", &["route", "region:us"]);
        mgr.register("route:eu:201", &["route", "region:eu"]);
        mgr.register("tile:us:1", &["tile", "region:us"]);

        // Invalidate by region:us — should remove 3 keys
        let invalidated = mgr.invalidate_by_tag("region:us", 1000);
        assert_eq!(invalidated.len(), 3);
        assert_eq!(mgr.key_count(), 1); // only route:eu:201 remains
    }
}
