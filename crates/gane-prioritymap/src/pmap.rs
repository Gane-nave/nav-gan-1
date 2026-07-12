//! Priority map implementation — a priority queue with key-based updates.

use std::collections::HashMap;

/// An entry in the priority map.
#[derive(Debug, Clone)]
struct PMapEntry {
    key: String,
    priority: i64,
}

/// A priority map that combines a priority queue with key-based lookup and update.
///
/// Supports O(log n) insert/remove-min and O(1) priority lookup by key.
/// Uses a binary heap backed by a HashMap for key indexing.
#[derive(Debug)]
pub struct PriorityMap {
    /// Heap storage (min-heap by priority).
    heap: Vec<PMapEntry>,
    /// Map from key to index in the heap.
    index: HashMap<String, usize>,
    /// Total operations performed.
    ops: u64,
}

impl PriorityMap {
    /// Create a new empty priority map.
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            index: HashMap::new(),
            ops: 0,
        }
    }

    /// Insert or update a key with the given priority.
    /// Returns `true` if the key was newly inserted, `false` if updated.
    pub fn insert(&mut self, key: &str, priority: i64) -> bool {
        self.ops = self.ops.saturating_add(1);
        if let Some(&idx) = self.index.get(key) {
            // Update existing
            let old_priority = self.heap[idx].priority;
            self.heap[idx].priority = priority;
            if priority < old_priority {
                self.sift_up(idx);
            } else {
                self.sift_down(idx);
            }
            false
        } else {
            // Insert new
            let idx = self.heap.len();
            self.heap.push(PMapEntry {
                key: key.to_string(),
                priority,
            });
            self.index.insert(key.to_string(), idx);
            self.sift_up(idx);
            true
        }
    }

    /// Remove and return the key with the minimum priority.
    pub fn pop_min(&mut self) -> Option<(String, i64)> {
        if self.heap.is_empty() {
            return None;
        }
        self.ops = self.ops.saturating_add(1);
        let last = self.heap.len() - 1;
        self.swap(0, last);
        let entry = self.heap.pop().unwrap();
        self.index.remove(&entry.key);
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        Some((entry.key, entry.priority))
    }

    /// Peek at the minimum priority entry without removing it.
    pub fn peek_min(&self) -> Option<(&str, i64)> {
        self.heap.first().map(|e| (e.key.as_str(), e.priority))
    }

    /// Get the priority of a key.
    pub fn get(&self, key: &str) -> Option<i64> {
        self.index.get(key).map(|&idx| self.heap[idx].priority)
    }

    /// Remove a key from the map. Returns its priority if found.
    pub fn remove(&mut self, key: &str) -> Option<i64> {
        let idx = *self.index.get(key)?;
        self.ops = self.ops.saturating_add(1);
        let last = self.heap.len() - 1;
        self.swap(idx, last);
        let entry = self.heap.pop().unwrap();
        self.index.remove(&entry.key);
        if idx < self.heap.len() {
            self.sift_up(idx);
            self.sift_down(idx);
        }
        Some(entry.priority)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.index.contains_key(key)
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Total operations performed.
    pub fn total_ops(&self) -> u64 {
        self.ops
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.heap.clear();
        self.index.clear();
    }

    /// Get all keys sorted by priority.
    pub fn sorted_keys(&self) -> Vec<(String, i64)> {
        let mut entries: Vec<(String, i64)> = self
            .heap
            .iter()
            .map(|e| (e.key.clone(), e.priority))
            .collect();
        entries.sort_by_key(|(_, p)| *p);
        entries
    }

    /// Decrease the priority of a key (only if new priority is lower).
    /// Returns `true` if priority was decreased.
    pub fn decrease_priority(&mut self, key: &str, new_priority: i64) -> bool {
        if let Some(&idx) = self.index.get(key) {
            if new_priority < self.heap[idx].priority {
                self.heap[idx].priority = new_priority;
                self.sift_up(idx);
                return true;
            }
        }
        false
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.heap[idx].priority < self.heap[parent].priority {
                self.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.heap.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;

            if left < len && self.heap[left].priority < self.heap[smallest].priority {
                smallest = left;
            }
            if right < len && self.heap[right].priority < self.heap[smallest].priority {
                smallest = right;
            }

            if smallest != idx {
                self.swap(idx, smallest);
                idx = smallest;
            } else {
                break;
            }
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        self.heap.swap(a, b);
        let key_a = self.heap[a].key.clone();
        let key_b = self.heap[b].key.clone();
        self.index.insert(key_a, a);
        self.index.insert(key_b, b);
    }
}

impl Default for PriorityMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pm = PriorityMap::new();
        assert!(pm.is_empty());
        assert_eq!(pm.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut pm = PriorityMap::new();
        assert!(pm.insert("a", 10));
        assert_eq!(pm.len(), 1);
        assert!(pm.contains("a"));
    }

    #[test]
    fn test_insert_update() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 10);
        assert!(!pm.insert("a", 5)); // update returns false
        assert_eq!(pm.get("a"), Some(5));
    }

    #[test]
    fn test_pop_min() {
        let mut pm = PriorityMap::new();
        pm.insert("c", 30);
        pm.insert("a", 10);
        pm.insert("b", 20);
        let (key, pri) = pm.pop_min().unwrap();
        assert_eq!(key, "a");
        assert_eq!(pri, 10);
    }

    #[test]
    fn test_pop_min_order() {
        let mut pm = PriorityMap::new();
        pm.insert("c", 3);
        pm.insert("a", 1);
        pm.insert("b", 2);
        assert_eq!(pm.pop_min().unwrap().0, "a");
        assert_eq!(pm.pop_min().unwrap().0, "b");
        assert_eq!(pm.pop_min().unwrap().0, "c");
        assert!(pm.pop_min().is_none());
    }

    #[test]
    fn test_peek_min() {
        let mut pm = PriorityMap::new();
        pm.insert("b", 20);
        pm.insert("a", 10);
        let (key, pri) = pm.peek_min().unwrap();
        assert_eq!(key, "a");
        assert_eq!(pri, 10);
        assert_eq!(pm.len(), 2); // not removed
    }

    #[test]
    fn test_get() {
        let mut pm = PriorityMap::new();
        pm.insert("x", 42);
        assert_eq!(pm.get("x"), Some(42));
        assert_eq!(pm.get("y"), None);
    }

    #[test]
    fn test_remove() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 10);
        pm.insert("b", 20);
        assert_eq!(pm.remove("a"), Some(10));
        assert!(!pm.contains("a"));
        assert_eq!(pm.len(), 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut pm = PriorityMap::new();
        assert_eq!(pm.remove("x"), None);
    }

    #[test]
    fn test_decrease_priority() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 10);
        assert!(pm.decrease_priority("a", 5));
        assert_eq!(pm.get("a"), Some(5));
        assert!(!pm.decrease_priority("a", 20)); // higher, no change
        assert_eq!(pm.get("a"), Some(5));
    }

    #[test]
    fn test_sorted_keys() {
        let mut pm = PriorityMap::new();
        pm.insert("c", 30);
        pm.insert("a", 10);
        pm.insert("b", 20);
        let sorted = pm.sorted_keys();
        assert_eq!(sorted[0].0, "a");
        assert_eq!(sorted[1].0, "b");
        assert_eq!(sorted[2].0, "c");
    }

    #[test]
    fn test_clear() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 1);
        pm.insert("b", 2);
        pm.clear();
        assert!(pm.is_empty());
    }

    #[test]
    fn test_default() {
        let pm = PriorityMap::default();
        assert!(pm.is_empty());
    }

    #[test]
    fn test_negative_priorities() {
        let mut pm = PriorityMap::new();
        pm.insert("a", -10);
        pm.insert("b", -5);
        pm.insert("c", 0);
        assert_eq!(pm.pop_min().unwrap(), ("a".to_string(), -10));
    }

    #[test]
    fn test_same_priority() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 1);
        pm.insert("b", 1);
        pm.insert("c", 1);
        assert_eq!(pm.len(), 3);
        pm.pop_min();
        assert_eq!(pm.len(), 2);
    }

    #[test]
    fn test_ops_counter() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 1);
        pm.insert("b", 2);
        pm.pop_min();
        assert_eq!(pm.total_ops(), 3);
    }

    #[test]
    fn test_update_maintains_heap() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 100);
        pm.insert("b", 50);
        pm.insert("c", 75);
        // Update a to be lowest
        pm.insert("a", 1);
        assert_eq!(pm.peek_min().unwrap().0, "a");
    }

    #[test]
    fn test_remove_min_then_insert() {
        let mut pm = PriorityMap::new();
        pm.insert("a", 1);
        pm.insert("b", 2);
        pm.pop_min();
        pm.insert("c", 0);
        assert_eq!(pm.peek_min().unwrap().0, "c");
    }
}
