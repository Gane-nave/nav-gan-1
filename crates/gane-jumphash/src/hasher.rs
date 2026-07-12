//! Jump consistent hashing implementation.

use std::hash::{Hash, Hasher};

/// Compute the jump consistent hash for a key into `num_buckets` buckets.
/// Returns a bucket index in [0, num_buckets).
/// Uses the algorithm from Lamping & Veach (Google, 2014).
pub fn jump_hash(mut key: u64, num_buckets: u32) -> u32 {
    if num_buckets == 0 {
        return 0;
    }
    let mut b: i64 = -1;
    let mut j: i64 = 0;
    while j < num_buckets as i64 {
        b = j;
        key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b.wrapping_add(1)) as f64
            * ((1i64 << 31) as f64 / ((key >> 33).wrapping_add(1)) as f64)) as i64;
    }
    b as u32
}

/// A jump consistent hasher that maps arbitrary hashable keys to buckets.
pub struct JumpHasher {
    num_buckets: u32,
}

impl JumpHasher {
    /// Create a new jump hasher with the given number of buckets.
    pub fn new(num_buckets: u32) -> Self {
        Self {
            num_buckets: num_buckets.max(1),
        }
    }

    /// Hash a key to a bucket index.
    pub fn hash_key<T: Hash>(&self, key: &T) -> u32 {
        let mut h = SimpleHash::new();
        key.hash(&mut h);
        jump_hash(h.finish(), self.num_buckets)
    }

    /// Number of buckets.
    pub fn num_buckets(&self) -> u32 {
        self.num_buckets
    }

    /// Resize to a new number of buckets.
    pub fn resize(&mut self, new_buckets: u32) {
        self.num_buckets = new_buckets.max(1);
    }

    /// Compute the distribution of keys across buckets for a range of keys.
    pub fn distribution(&self, num_keys: u64) -> Vec<u64> {
        let mut counts = vec![0u64; self.num_buckets as usize];
        for key in 0..num_keys {
            let bucket = jump_hash(key, self.num_buckets) as usize;
            counts[bucket] = counts[bucket].saturating_add(1);
        }
        counts
    }
}

struct SimpleHash {
    state: u64,
}

impl SimpleHash {
    fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
}

impl Hasher for SimpleHash {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(0x100000001b3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_hash_deterministic() {
        let a = jump_hash(42, 10);
        let b = jump_hash(42, 10);
        assert_eq!(a, b);
    }

    #[test]
    fn test_jump_hash_range() {
        for key in 0..1000 {
            let bucket = jump_hash(key, 10);
            assert!(bucket < 10);
        }
    }

    #[test]
    fn test_jump_hash_single_bucket() {
        for key in 0..100 {
            assert_eq!(jump_hash(key, 1), 0);
        }
    }

    #[test]
    fn test_jump_hash_zero_buckets() {
        assert_eq!(jump_hash(42, 0), 0);
    }

    #[test]
    fn test_hasher_new() {
        let h = JumpHasher::new(10);
        assert_eq!(h.num_buckets(), 10);
    }

    #[test]
    fn test_hasher_hash_key() {
        let h = JumpHasher::new(10);
        let bucket = h.hash_key(&"test");
        assert!(bucket < 10);
    }

    #[test]
    fn test_hasher_resize() {
        let mut h = JumpHasher::new(10);
        h.resize(20);
        assert_eq!(h.num_buckets(), 20);
    }

    #[test]
    fn test_distribution_uniformity() {
        let h = JumpHasher::new(4);
        let dist = h.distribution(10000);
        for &count in &dist {
            assert!(count > 1000, "bucket count {} too low", count);
            assert!(count < 4000, "bucket count {} too high", count);
        }
    }

    #[test]
    fn test_hasher_min_buckets() {
        let h = JumpHasher::new(0);
        assert_eq!(h.num_buckets(), 1);
    }

    #[test]
    fn test_monotonicity() {
        // When adding a bucket, keys should only move to the new bucket
        let mut moved_to_new = 0;
        let mut moved_elsewhere = 0;
        for key in 0..10000_u64 {
            let old = jump_hash(key, 10);
            let new = jump_hash(key, 11);
            if old != new {
                if new == 10 {
                    moved_to_new += 1;
                } else {
                    moved_elsewhere += 1;
                }
            }
        }
        assert_eq!(moved_elsewhere, 0, "keys moved to wrong bucket");
        assert!(moved_to_new > 0, "no keys moved to new bucket");
    }
}
