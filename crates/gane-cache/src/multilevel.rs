//! Multi-level cache — L1 (fast/small) + L2 (slower/larger) with promotion.

/// Cache level identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheLevel {
    L1,
    L2,
}

/// Result of a cache lookup indicating where the value was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupResult<V> {
    /// Cache miss — not found in any level.
    Miss,
    /// Found in L1 (hot cache).
    HitL1(V),
    /// Found in L2 (warm cache) — will be promoted to L1.
    HitL2(V),
}

/// Multi-level cache with two tiers.
pub struct MultiLevelCache {
    /// L1 entries (fast, small).
    l1: Vec<(String, String)>,
    /// L2 entries (slower, larger).
    l2: Vec<(String, String)>,
    /// Max L1 entries.
    l1_capacity: usize,
    /// Max L2 entries.
    l2_capacity: usize,
    /// Stats.
    l1_hits: u64,
    l2_hits: u64,
    misses: u64,
    promotions: u64,
    demotions: u64,
}

impl MultiLevelCache {
    /// Create a new multi-level cache.
    pub fn new(l1_capacity: usize, l2_capacity: usize) -> Self {
        Self {
            l1: Vec::new(),
            l2: Vec::new(),
            l1_capacity,
            l2_capacity,
            l1_hits: 0,
            l2_hits: 0,
            misses: 0,
            promotions: 0,
            demotions: 0,
        }
    }

    /// Look up a key across both levels.
    pub fn get(&mut self, key: &str) -> LookupResult<String> {
        // Check L1 first
        if let Some(pos) = self.l1.iter().position(|(k, _)| k == key) {
            self.l1_hits += 1;
            let value = self.l1[pos].1.clone();
            return LookupResult::HitL1(value);
        }

        // Check L2
        if let Some(pos) = self.l2.iter().position(|(k, _)| k == key) {
            self.l2_hits += 1;
            let (k, v) = self.l2.remove(pos);
            // Promote to L1
            self.promote(k.clone(), v.clone());
            return LookupResult::HitL2(v);
        }

        self.misses += 1;
        LookupResult::Miss
    }

    /// Insert into L1 (may demote to L2 if full).
    pub fn insert(&mut self, key: String, value: String) {
        // Remove from L2 if present (will be moved to L1)
        self.l2.retain(|(k, _)| k != &key);

        // Remove from L1 if present (update in place)
        if let Some(pos) = self.l1.iter().position(|(k, _)| k == &key) {
            self.l1[pos].1 = value;
            return;
        }

        // If L1 is full, demote oldest to L2
        if self.l1.len() >= self.l1_capacity {
            let demoted = self.l1.remove(0);
            self.demote(demoted.0, demoted.1);
        }

        self.l1.push((key, value));
    }

    /// Promote an entry from L2 to L1.
    fn promote(&mut self, key: String, value: String) {
        self.promotions += 1;
        // If L1 full, demote oldest
        if self.l1.len() >= self.l1_capacity {
            let demoted = self.l1.remove(0);
            self.demote(demoted.0, demoted.1);
        }
        self.l1.push((key, value));
    }

    /// Demote an entry from L1 to L2.
    fn demote(&mut self, key: String, value: String) {
        self.demotions += 1;
        // If L2 full, evict oldest
        if self.l2.len() >= self.l2_capacity {
            self.l2.remove(0);
        }
        self.l2.push((key, value));
    }

    /// Invalidate a key from all levels.
    pub fn invalidate(&mut self, key: &str) -> bool {
        let l1_removed = self.l1.iter().position(|(k, _)| k == key).map(|i| {
            self.l1.remove(i);
        });
        let l2_removed = self.l2.iter().position(|(k, _)| k == key).map(|i| {
            self.l2.remove(i);
        });
        l1_removed.is_some() || l2_removed.is_some()
    }

    /// Clear all levels.
    pub fn clear(&mut self) {
        self.l1.clear();
        self.l2.clear();
    }

    /// Get L1 entry count.
    pub fn l1_len(&self) -> usize {
        self.l1.len()
    }

    /// Get L2 entry count.
    pub fn l2_len(&self) -> usize {
        self.l2.len()
    }

    /// Get total entry count.
    pub fn total_len(&self) -> usize {
        self.l1.len() + self.l2.len()
    }

    /// Get L1 hit count.
    pub fn l1_hits(&self) -> u64 {
        self.l1_hits
    }

    /// Get L2 hit count.
    pub fn l2_hits(&self) -> u64 {
        self.l2_hits
    }

    /// Get miss count.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Get promotion count.
    pub fn promotions(&self) -> u64 {
        self.promotions
    }

    /// Get demotion count.
    pub fn demotions(&self) -> u64 {
        self.demotions
    }

    /// Get overall hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.l1_hits + self.l2_hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        (self.l1_hits + self.l2_hits) as f64 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_l1_hit() {
        let mut cache = MultiLevelCache::new(3, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        match cache.get("k1") {
            LookupResult::HitL1(v) => assert_eq!(v, "v1"),
            other => panic!("Expected HitL1, got {other:?}"),
        }
        assert_eq!(cache.l1_hits(), 1);
    }

    #[test]
    fn test_l1_full_demotes_to_l2() {
        let mut cache = MultiLevelCache::new(2, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.insert("k2".to_string(), "v2".to_string());
        cache.insert("k3".to_string(), "v3".to_string()); // demotes k1 to L2
        assert_eq!(cache.l1_len(), 2);
        assert_eq!(cache.l2_len(), 1);
        assert_eq!(cache.demotions(), 1);
    }

    #[test]
    fn test_l2_hit_promotes_to_l1() {
        let mut cache = MultiLevelCache::new(2, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.insert("k2".to_string(), "v2".to_string());
        cache.insert("k3".to_string(), "v3".to_string()); // k1 demoted to L2

        match cache.get("k1") {
            LookupResult::HitL2(v) => assert_eq!(v, "v1"),
            other => panic!("Expected HitL2, got {other:?}"),
        }
        assert_eq!(cache.l2_hits(), 1);
        assert_eq!(cache.promotions(), 1);
        // k1 is now in L1, k2 was demoted
        assert_eq!(cache.l1_len(), 2);
    }

    #[test]
    fn test_miss() {
        let mut cache = MultiLevelCache::new(3, 5);
        assert_eq!(cache.get("missing"), LookupResult::Miss);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_invalidate() {
        let mut cache = MultiLevelCache::new(3, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        assert!(cache.invalidate("k1"));
        assert_eq!(cache.get("k1"), LookupResult::Miss);
        assert!(!cache.invalidate("k1")); // already removed
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = MultiLevelCache::new(3, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.get("k1"); // L1 hit
        cache.get("k1"); // L1 hit
        cache.get("missing"); // miss
                              // 2 hits, 1 miss → 2/3
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_update_existing_key_in_l1() {
        let mut cache = MultiLevelCache::new(3, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.insert("k1".to_string(), "v2".to_string());
        assert_eq!(cache.l1_len(), 1);
        match cache.get("k1") {
            LookupResult::HitL1(v) => assert_eq!(v, "v2"),
            other => panic!("Expected HitL1, got {other:?}"),
        }
    }

    #[test]
    fn test_clear() {
        let mut cache = MultiLevelCache::new(3, 5);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.insert("k2".to_string(), "v2".to_string());
        cache.clear();
        assert_eq!(cache.total_len(), 0);
    }

    #[test]
    fn test_l2_eviction_when_full() {
        let mut cache = MultiLevelCache::new(1, 2);
        cache.insert("k1".to_string(), "v1".to_string());
        cache.insert("k2".to_string(), "v2".to_string()); // k1 → L2
        cache.insert("k3".to_string(), "v3".to_string()); // k2 → L2
        cache.insert("k4".to_string(), "v4".to_string()); // k3 → L2, but L2 full(2), evicts k1
        assert_eq!(cache.l2_len(), 2);
        // k1 should have been evicted from L2
        assert_eq!(cache.get("k1"), LookupResult::Miss);
    }
}
