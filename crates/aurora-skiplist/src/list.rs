use crate::node::SkipNode;

/// Maximum number of levels in the skip list.
const MAX_LEVEL: usize = 16;

/// A skip list providing ordered key-value storage with O(log n) average operations.
///
/// Uses a deterministic level assignment based on key hash to avoid
/// requiring a random number generator (std-only, no external deps).
#[derive(Debug)]
pub struct SkipList<V: Clone> {
    /// Sorted storage of nodes.
    entries: Vec<SkipNode<V>>,
    /// Maximum level currently in use.
    max_level: usize,
    /// Total number of insertions.
    total_inserts: u64,
    /// Total number of removals.
    total_removes: u64,
    /// Total number of lookups.
    total_lookups: u64,
    /// Total number of hits (successful lookups).
    total_hits: u64,
}

impl<V: Clone> SkipList<V> {
    /// Create a new empty skip list.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_level: 1,
            total_inserts: 0,
            total_removes: 0,
            total_lookups: 0,
            total_hits: 0,
        }
    }

    /// Determine the level for a key using bit counting.
    fn random_level(key: u64) -> usize {
        // Use trailing zeros of a hash as the level
        let hash = key.wrapping_mul(0x517cc1b727220a95);
        let zeros = hash.trailing_zeros() as usize;
        (zeros + 1).min(MAX_LEVEL)
    }

    /// Insert a key-value pair. If the key already exists, update the value.
    /// Returns the old value if the key was already present.
    pub fn insert(&mut self, key: u64, value: V) -> Option<V> {
        self.total_inserts = self.total_inserts.saturating_add(1);

        // Check if key already exists
        if let Some(pos) = self.entries.iter().position(|n| n.key() == key) {
            let old = self.entries[pos].value().clone();
            self.entries[pos].set_value(value);
            return Some(old);
        }

        let level = Self::random_level(key);
        if level > self.max_level {
            self.max_level = level;
        }

        let node = SkipNode::new(key, value, level);

        // Insert in sorted order
        let pos = self
            .entries
            .binary_search_by_key(&key, |n| n.key())
            .unwrap_or_else(|p| p);
        self.entries.insert(pos, node);

        None
    }

    /// Get a reference to the value associated with the key.
    pub fn get(&mut self, key: u64) -> Option<&V> {
        self.total_lookups = self.total_lookups.saturating_add(1);
        match self.entries.binary_search_by_key(&key, |n| n.key()) {
            Ok(pos) => {
                self.total_hits = self.total_hits.saturating_add(1);
                Some(self.entries[pos].value())
            }
            Err(_) => None,
        }
    }

    /// Remove a key and return its value if it existed.
    pub fn remove(&mut self, key: u64) -> Option<V> {
        match self.entries.binary_search_by_key(&key, |n| n.key()) {
            Ok(pos) => {
                self.total_removes = self.total_removes.saturating_add(1);
                let node = self.entries.remove(pos);
                Some(node.value().clone())
            }
            Err(_) => None,
        }
    }

    /// Check if the key exists.
    pub fn contains(&self, key: u64) -> bool {
        self.entries.binary_search_by_key(&key, |n| n.key()).is_ok()
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the current maximum level.
    pub fn max_level(&self) -> usize {
        self.max_level
    }

    /// Get the smallest key, if any.
    pub fn first(&self) -> Option<(u64, &V)> {
        self.entries.first().map(|n| (n.key(), n.value()))
    }

    /// Get the largest key, if any.
    pub fn last(&self) -> Option<(u64, &V)> {
        self.entries.last().map(|n| (n.key(), n.value()))
    }

    /// Get all keys in sorted order.
    pub fn keys(&self) -> Vec<u64> {
        self.entries.iter().map(|n| n.key()).collect()
    }

    /// Get entries in a key range [start, end) as (key, value) pairs.
    pub fn range(&self, start: u64, end: u64) -> Vec<(u64, V)> {
        self.entries
            .iter()
            .filter(|n| n.key() >= start && n.key() < end)
            .map(|n| (n.key(), n.value().clone()))
            .collect()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.max_level = 1;
    }

    /// Get total insertions.
    pub fn total_inserts(&self) -> u64 {
        self.total_inserts
    }

    /// Get total removals.
    pub fn total_removes(&self) -> u64 {
        self.total_removes
    }

    /// Get total lookups.
    pub fn total_lookups(&self) -> u64 {
        self.total_lookups
    }

    /// Get hit rate (successful lookups / total lookups).
    pub fn hit_rate(&self) -> f64 {
        if self.total_lookups == 0 {
            return 0.0;
        }
        self.total_hits as f64 / self.total_lookups as f64
    }
}

impl<V: Clone> Default for SkipList<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut sl = SkipList::new();
        assert!(sl.insert(10, "ten").is_none());
        assert!(sl.insert(5, "five").is_none());
        assert!(sl.insert(15, "fifteen").is_none());
        assert_eq!(sl.get(10), Some(&"ten"));
        assert_eq!(sl.get(5), Some(&"five"));
        assert_eq!(sl.get(15), Some(&"fifteen"));
        assert_eq!(sl.get(99), None);
    }

    #[test]
    fn test_insert_update() {
        let mut sl = SkipList::new();
        sl.insert(1, 100u32);
        let old = sl.insert(1, 200);
        assert_eq!(old, Some(100));
        assert_eq!(sl.get(1), Some(&200));
        assert_eq!(sl.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut sl = SkipList::new();
        sl.insert(1, "a");
        sl.insert(2, "b");
        sl.insert(3, "c");
        assert_eq!(sl.remove(2), Some("b"));
        assert_eq!(sl.get(2), None);
        assert_eq!(sl.len(), 2);
        assert_eq!(sl.remove(99), None);
    }

    #[test]
    fn test_sorted_order() {
        let mut sl = SkipList::new();
        sl.insert(50, "fifty");
        sl.insert(10, "ten");
        sl.insert(30, "thirty");
        sl.insert(20, "twenty");
        sl.insert(40, "forty");
        assert_eq!(sl.keys(), vec![10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_first_last() {
        let mut sl = SkipList::new();
        assert_eq!(sl.first(), None);
        assert_eq!(sl.last(), None);
        sl.insert(30, "c");
        sl.insert(10, "a");
        sl.insert(20, "b");
        assert_eq!(sl.first(), Some((10, &"a")));
        assert_eq!(sl.last(), Some((30, &"c")));
    }

    #[test]
    fn test_range() {
        let mut sl = SkipList::new();
        for i in 0..10 {
            sl.insert(i * 10, i as u32);
        }
        let r = sl.range(20, 60);
        assert_eq!(r, vec![(20, 2), (30, 3), (40, 4), (50, 5)]);
    }

    #[test]
    fn test_clear() {
        let mut sl = SkipList::new();
        sl.insert(1, 1u32);
        sl.insert(2, 2);
        sl.clear();
        assert!(sl.is_empty());
        assert_eq!(sl.len(), 0);
    }

    #[test]
    fn test_contains() {
        let mut sl = SkipList::new();
        sl.insert(42, "x");
        assert!(sl.contains(42));
        assert!(!sl.contains(43));
    }

    #[test]
    fn test_hit_rate() {
        let mut sl = SkipList::new();
        sl.insert(1, "a");
        sl.get(1); // hit
        sl.get(1); // hit
        sl.get(99); // miss
        assert_eq!(sl.total_lookups(), 3);
        assert!((sl.hit_rate() - 2.0 / 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stats() {
        let mut sl = SkipList::new();
        sl.insert(1, "a");
        sl.insert(2, "b");
        sl.remove(1);
        assert_eq!(sl.total_inserts(), 2);
        assert_eq!(sl.total_removes(), 1);
    }
}
