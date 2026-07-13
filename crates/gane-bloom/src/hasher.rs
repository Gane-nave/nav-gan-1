//! Simple hash functions for the bloom filter.
//! Uses FNV-1a style hashing with dual-hash technique for multiple hash functions.

/// A dual-hash generator that produces `k` hash values from two base hashes.
/// Uses the formula: h(i) = h1 + i * h2 (mod m).
#[derive(Debug, Clone)]
pub struct DualHasher {
    seed1: u64,
    seed2: u64,
}

impl DualHasher {
    /// Create a new dual hasher with given seeds.
    pub fn new(seed1: u64, seed2: u64) -> Self {
        Self { seed1, seed2 }
    }

    /// Default hasher with well-distributed seeds.
    pub fn default_hasher() -> Self {
        Self::new(0x517cc1b727220a95, 0x6c62272e07bb0142)
    }

    /// Compute a FNV-1a-like hash of the input bytes with a given seed.
    fn fnv_hash(data: &[u8], seed: u64) -> u64 {
        let mut hash = seed ^ 0xcbf29ce484222325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    /// Generate `k` hash indices in [0, m) for the given data.
    pub fn hashes(&self, data: &[u8], k: usize, m: usize) -> Vec<usize> {
        if m == 0 {
            return vec![0; k];
        }
        let h1 = Self::fnv_hash(data, self.seed1);
        let h2 = Self::fnv_hash(data, self.seed2);
        (0..k)
            .map(|i| {
                let combined = h1.wrapping_add((i as u64).wrapping_mul(h2));
                (combined % m as u64) as usize
            })
            .collect()
    }

    /// Get the two base hashes for diagnostics.
    pub fn base_hashes(&self, data: &[u8]) -> (u64, u64) {
        (
            Self::fnv_hash(data, self.seed1),
            Self::fnv_hash(data, self.seed2),
        )
    }

    /// Seeds used by this hasher.
    pub fn seeds(&self) -> (u64, u64) {
        (self.seed1, self.seed2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hasher() {
        let h = DualHasher::default_hasher();
        let (s1, s2) = h.seeds();
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_fnv_hash_deterministic() {
        let data = b"hello";
        let h1 = DualHasher::fnv_hash(data, 42);
        let h2 = DualHasher::fnv_hash(data, 42);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_different_seeds_produce_different_hashes() {
        let data = b"hello";
        let h1 = DualHasher::fnv_hash(data, 1);
        let h2 = DualHasher::fnv_hash(data, 2);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hashes_count() {
        let h = DualHasher::default_hasher();
        let indices = h.hashes(b"test", 5, 100);
        assert_eq!(indices.len(), 5);
        for &idx in &indices {
            assert!(idx < 100);
        }
    }

    #[test]
    fn test_hashes_within_bounds() {
        let h = DualHasher::default_hasher();
        let indices = h.hashes(b"data", 10, 7);
        for &idx in &indices {
            assert!(idx < 7);
        }
    }

    #[test]
    fn test_hashes_zero_m() {
        let h = DualHasher::default_hasher();
        let indices = h.hashes(b"data", 3, 0);
        assert_eq!(indices.len(), 3);
        assert!(indices.iter().all(|&i| i == 0));
    }

    #[test]
    fn test_base_hashes() {
        let h = DualHasher::default_hasher();
        let (h1, h2) = h.base_hashes(b"test");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_different_data_produces_different_hashes() {
        let h = DualHasher::default_hasher();
        let i1 = h.hashes(b"alpha", 3, 1000);
        let i2 = h.hashes(b"beta", 3, 1000);
        assert_ne!(i1, i2);
    }
}
