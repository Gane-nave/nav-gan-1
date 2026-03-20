//! Conflict-free Replicated Data Types.

use std::collections::HashMap;

/// G-Counter: grow-only counter (increment only).
#[derive(Debug, Clone)]
pub struct GCounter {
    counts: HashMap<String, u64>,
}

impl GCounter {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        let entry = self.counts.entry(node_id.to_string()).or_insert(0);
        *entry = entry.saturating_add(1);
    }

    pub fn increment_by(&mut self, node_id: &str, amount: u64) {
        let entry = self.counts.entry(node_id.to_string()).or_insert(0);
        *entry = entry.saturating_add(amount);
    }

    pub fn value(&self) -> u64 {
        self.counts
            .values()
            .copied()
            .fold(0u64, |a, b| a.saturating_add(b))
    }

    pub fn merge(&mut self, other: &GCounter) {
        for (node, &count) in &other.counts {
            let entry = self.counts.entry(node.clone()).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    pub fn node_count(&self, node_id: &str) -> u64 {
        self.counts.get(node_id).copied().unwrap_or(0)
    }
}

impl Default for GCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// PN-Counter: positive-negative counter (increment and decrement).
#[derive(Debug, Clone)]
pub struct PNCounter {
    positive: GCounter,
    negative: GCounter,
}

impl PNCounter {
    pub fn new() -> Self {
        Self {
            positive: GCounter::new(),
            negative: GCounter::new(),
        }
    }

    pub fn increment(&mut self, node_id: &str) {
        self.positive.increment(node_id);
    }

    pub fn decrement(&mut self, node_id: &str) {
        self.negative.increment(node_id);
    }

    pub fn value(&self) -> i64 {
        let p = self.positive.value() as i64;
        let n = self.negative.value() as i64;
        p.saturating_sub(n)
    }

    pub fn merge(&mut self, other: &PNCounter) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }
}

impl Default for PNCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// LWW-Register: Last-Writer-Wins register.
#[derive(Debug, Clone)]
pub struct LWWRegister<V: Clone> {
    value: Option<V>,
    timestamp: u64,
}

impl<V: Clone> LWWRegister<V> {
    pub fn new() -> Self {
        Self {
            value: None,
            timestamp: 0,
        }
    }

    pub fn set(&mut self, value: V, timestamp: u64) {
        if timestamp >= self.timestamp {
            self.value = Some(value);
            self.timestamp = timestamp;
        }
    }

    pub fn get(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn merge(&mut self, other: &LWWRegister<V>) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<V: Clone> Default for LWWRegister<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// OR-Set: Observed-Remove Set.
#[derive(Debug, Clone)]
pub struct ORSet<V: Eq + std::hash::Hash + Clone> {
    elements: HashMap<V, Vec<u64>>,
    next_tag: u64,
}

impl<V: Eq + std::hash::Hash + Clone> ORSet<V> {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            next_tag: 0,
        }
    }

    pub fn insert(&mut self, value: V) -> u64 {
        let tag = self.next_tag;
        self.next_tag = self.next_tag.saturating_add(1);
        self.elements.entry(value).or_default().push(tag);
        tag
    }

    pub fn remove(&mut self, value: &V) {
        self.elements.remove(value);
    }

    pub fn contains(&self, value: &V) -> bool {
        self.elements
            .get(value)
            .is_some_and(|tags| !tags.is_empty())
    }

    pub fn len(&self) -> usize {
        self.elements
            .values()
            .filter(|tags| !tags.is_empty())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn values(&self) -> Vec<&V> {
        self.elements
            .iter()
            .filter(|(_, tags)| !tags.is_empty())
            .map(|(v, _)| v)
            .collect()
    }

    pub fn merge(&mut self, other: &ORSet<V>) {
        for (value, tags) in &other.elements {
            let entry = self.elements.entry(value.clone()).or_default();
            for &tag in tags {
                if !entry.contains(&tag) {
                    entry.push(tag);
                }
            }
        }
        self.next_tag = self.next_tag.max(other.next_tag);
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

impl<V: Eq + std::hash::Hash + Clone> Default for ORSet<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcounter_increment() {
        let mut c = GCounter::new();
        c.increment("a");
        c.increment("a");
        c.increment("b");
        assert_eq!(c.value(), 3);
        assert_eq!(c.node_count("a"), 2);
    }

    #[test]
    fn test_gcounter_merge() {
        let mut c1 = GCounter::new();
        let mut c2 = GCounter::new();
        c1.increment_by("a", 5);
        c2.increment_by("a", 3);
        c2.increment_by("b", 2);
        c1.merge(&c2);
        assert_eq!(c1.value(), 7); // max(5,3) + 2
    }

    #[test]
    fn test_pncounter() {
        let mut c = PNCounter::new();
        c.increment("a");
        c.increment("a");
        c.decrement("a");
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn test_pncounter_merge() {
        let mut c1 = PNCounter::new();
        let mut c2 = PNCounter::new();
        c1.increment("a");
        c1.increment("a");
        c2.decrement("a");
        c1.merge(&c2);
        assert_eq!(c1.value(), 1);
    }

    #[test]
    fn test_lww_register() {
        let mut r = LWWRegister::new();
        r.set("first", 1);
        r.set("second", 2);
        assert_eq!(r.get(), Some(&"second"));
    }

    #[test]
    fn test_lww_register_old_write() {
        let mut r = LWWRegister::new();
        r.set("new", 10);
        r.set("old", 5);
        assert_eq!(r.get(), Some(&"new"));
    }

    #[test]
    fn test_lww_merge() {
        let mut r1 = LWWRegister::new();
        let mut r2 = LWWRegister::new();
        r1.set("old", 1);
        r2.set("new", 5);
        r1.merge(&r2);
        assert_eq!(r1.get(), Some(&"new"));
    }

    #[test]
    fn test_orset_insert_remove() {
        let mut s = ORSet::new();
        s.insert("hello");
        assert!(s.contains(&"hello"));
        s.remove(&"hello");
        assert!(!s.contains(&"hello"));
    }

    #[test]
    fn test_orset_len() {
        let mut s = ORSet::new();
        s.insert(1);
        s.insert(2);
        s.insert(3);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_orset_merge() {
        let mut s1 = ORSet::new();
        let mut s2 = ORSet::new();
        s1.insert("a");
        s2.insert("b");
        s1.merge(&s2);
        assert!(s1.contains(&"a"));
        assert!(s1.contains(&"b"));
    }
}
