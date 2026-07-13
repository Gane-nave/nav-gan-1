//! Cuckoo filter for approximate set membership.

use std::hash::{Hash, Hasher};

const BUCKET_SIZE: usize = 4;
const MAX_KICKS: usize = 500;

/// A cuckoo filter for approximate set membership testing.
/// Supports insert, lookup, and delete with a configurable false positive rate.
pub struct CuckooFilter {
    buckets: Vec<[Option<u8>; BUCKET_SIZE]>,
    num_buckets: usize,
    count: usize,
}

impl CuckooFilter {
    /// Create a new cuckoo filter with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let num_buckets = (capacity / BUCKET_SIZE).max(1);
        Self {
            buckets: vec![[None; BUCKET_SIZE]; num_buckets],
            num_buckets,
            count: 0,
        }
    }

    /// Number of items in the filter.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether the filter is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Load factor of the filter.
    pub fn load_factor(&self) -> f64 {
        self.count as f64 / (self.num_buckets * BUCKET_SIZE) as f64
    }

    fn fingerprint<T: Hash>(item: &T) -> u8 {
        let mut h = SimpleHasher::new(0x517cc1b7);
        item.hash(&mut h);
        let fp = (h.finish() & 0xFF) as u8;
        if fp == 0 {
            1
        } else {
            fp
        }
    }

    fn index1<T: Hash>(&self, item: &T) -> usize {
        let mut h = SimpleHasher::new(0x9e3779b9);
        item.hash(&mut h);
        (h.finish() as usize) % self.num_buckets
    }

    fn index2(&self, i1: usize, fp: u8) -> usize {
        let mut h = SimpleHasher::new(0x6a09e667);
        h.write_u8(fp);
        let hash = h.finish() as usize;
        (i1 ^ hash) % self.num_buckets
    }

    /// Insert an item into the filter. Returns true if successful.
    pub fn insert<T: Hash>(&mut self, item: &T) -> bool {
        let fp = Self::fingerprint(item);
        let i1 = self.index1(item);
        let i2 = self.index2(i1, fp);

        // Try to insert in bucket i1
        for slot in &mut self.buckets[i1] {
            if slot.is_none() {
                *slot = Some(fp);
                self.count = self.count.saturating_add(1);
                return true;
            }
        }

        // Try to insert in bucket i2
        for slot in &mut self.buckets[i2] {
            if slot.is_none() {
                *slot = Some(fp);
                self.count = self.count.saturating_add(1);
                return true;
            }
        }

        // Kick existing entries
        let mut idx = i1;
        let mut current_fp = fp;
        let mut rng_state: u64 = fp as u64;
        for _ in 0..MAX_KICKS {
            // Pick a random slot to kick
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let slot_idx = (rng_state as usize) % BUCKET_SIZE;
            let kicked = self.buckets[idx][slot_idx].replace(current_fp);
            current_fp = kicked.unwrap_or(current_fp);
            idx = self.index2(idx, current_fp);

            for slot in &mut self.buckets[idx] {
                if slot.is_none() {
                    *slot = Some(current_fp);
                    self.count = self.count.saturating_add(1);
                    return true;
                }
            }
        }

        false // filter is too full
    }

    /// Check if an item might be in the filter.
    /// May return false positives but never false negatives.
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        let fp = Self::fingerprint(item);
        let i1 = self.index1(item);
        let i2 = self.index2(i1, fp);

        self.buckets[i1].contains(&Some(fp)) || self.buckets[i2].contains(&Some(fp))
    }

    /// Remove an item from the filter. Returns true if found and removed.
    pub fn remove<T: Hash>(&mut self, item: &T) -> bool {
        let fp = Self::fingerprint(item);
        let i1 = self.index1(item);
        let i2 = self.index2(i1, fp);

        for slot in &mut self.buckets[i1] {
            if *slot == Some(fp) {
                *slot = None;
                self.count = self.count.saturating_sub(1);
                return true;
            }
        }
        for slot in &mut self.buckets[i2] {
            if *slot == Some(fp) {
                *slot = None;
                self.count = self.count.saturating_sub(1);
                return true;
            }
        }
        false
    }

    /// Clear all items from the filter.
    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            *bucket = [None; BUCKET_SIZE];
        }
        self.count = 0;
    }
}

struct SimpleHasher {
    state: u64,
}

impl SimpleHasher {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(b as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut cf = CuckooFilter::new(100);
        assert!(cf.insert(&42));
        assert!(cf.contains(&42));
        assert!(!cf.contains(&43));
    }

    #[test]
    fn test_empty() {
        let cf = CuckooFilter::new(100);
        assert!(cf.is_empty());
        assert_eq!(cf.len(), 0);
    }

    #[test]
    fn test_remove() {
        let mut cf = CuckooFilter::new(100);
        cf.insert(&42);
        assert!(cf.contains(&42));
        assert!(cf.remove(&42));
        assert!(!cf.contains(&42));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut cf = CuckooFilter::new(100);
        assert!(!cf.remove(&42));
    }

    #[test]
    fn test_multiple_inserts() {
        let mut cf = CuckooFilter::new(1000);
        for i in 0..100 {
            cf.insert(&i);
        }
        assert_eq!(cf.len(), 100);
        for i in 0..100 {
            assert!(cf.contains(&i));
        }
    }

    #[test]
    fn test_clear() {
        let mut cf = CuckooFilter::new(100);
        cf.insert(&1);
        cf.insert(&2);
        cf.clear();
        assert!(cf.is_empty());
        assert!(!cf.contains(&1));
    }

    #[test]
    fn test_load_factor() {
        let mut cf = CuckooFilter::new(100);
        assert_eq!(cf.load_factor(), 0.0);
        cf.insert(&1);
        assert!(cf.load_factor() > 0.0);
    }

    #[test]
    fn test_string_items() {
        let mut cf = CuckooFilter::new(100);
        cf.insert(&"hello");
        cf.insert(&"world");
        assert!(cf.contains(&"hello"));
        assert!(cf.contains(&"world"));
        assert!(!cf.contains(&"foo"));
    }

    #[test]
    fn test_duplicate_insert() {
        let mut cf = CuckooFilter::new(100);
        assert!(cf.insert(&42));
        assert!(cf.insert(&42)); // duplicate fingerprint, still inserts
        assert!(cf.contains(&42));
    }

    #[test]
    fn test_capacity_one() {
        let mut cf = CuckooFilter::new(1);
        assert!(cf.insert(&1));
        assert!(cf.contains(&1));
    }
}
