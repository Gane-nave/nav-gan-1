//! Bloom filter for probabilistic set membership testing.
//!
//! A bloom filter can tell you definitively that an element is NOT in the set,
//! but may report false positives (saying an element IS in the set when it isn't).

use crate::hasher::DualHasher;

/// A probabilistic set membership filter.
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
    hasher: DualHasher,
    inserted: u64,
    queries: u64,
    positives: u64,
    bits_set: u64,
}

impl BloomFilter {
    /// Create a new bloom filter with `m` bits and `k` hash functions.
    pub fn new(num_bits: usize, num_hashes: usize) -> Self {
        Self {
            bits: vec![false; num_bits],
            num_hashes,
            hasher: DualHasher::default_hasher(),
            inserted: 0,
            queries: 0,
            positives: 0,
            bits_set: 0,
        }
    }

    /// Create a bloom filter sized for expected insertions and desired false positive rate.
    /// Uses the formula: m = -n*ln(p) / (ln2)^2, k = (m/n) * ln2
    pub fn with_rate(expected_items: usize, fp_rate: f64) -> Self {
        let fp_rate = fp_rate.max(f64::MIN_POSITIVE);
        let n = expected_items.max(1) as f64;
        let ln2 = core::f64::consts::LN_2;
        let m = (-(n * fp_rate.ln()) / (ln2 * ln2)).ceil() as usize;
        let m = m.max(1);
        let k = ((m as f64 / n) * ln2).ceil() as usize;
        let k = k.max(1);
        Self::new(m, k)
    }

    /// Insert an element into the filter.
    pub fn insert(&mut self, data: &[u8]) {
        let indices = self.hasher.hashes(data, self.num_hashes, self.bits.len());
        for idx in indices {
            if !self.bits[idx] {
                self.bits[idx] = true;
                self.bits_set += 1;
            }
        }
        self.inserted += 1;
    }

    /// Check if an element might be in the set.
    /// Returns `true` if possibly present (may be false positive).
    /// Returns `false` if definitely not present.
    pub fn might_contain(&mut self, data: &[u8]) -> bool {
        self.queries += 1;
        let indices = self.hasher.hashes(data, self.num_hashes, self.bits.len());
        let result = indices.iter().all(|&idx| self.bits[idx]);
        if result {
            self.positives += 1;
        }
        result
    }

    /// Check without updating stats (read-only query).
    pub fn contains_no_track(&self, data: &[u8]) -> bool {
        let indices = self.hasher.hashes(data, self.num_hashes, self.bits.len());
        indices.iter().all(|&idx| self.bits[idx])
    }

    /// Number of bits in the filter.
    pub fn num_bits(&self) -> usize {
        self.bits.len()
    }

    /// Number of hash functions.
    pub fn num_hashes(&self) -> usize {
        self.num_hashes
    }

    /// Total elements inserted.
    pub fn inserted(&self) -> u64 {
        self.inserted
    }

    /// Total queries performed.
    pub fn queries(&self) -> u64 {
        self.queries
    }

    /// Total positive results (true positives + false positives).
    pub fn positives(&self) -> u64 {
        self.positives
    }

    /// Number of bits set to 1.
    pub fn bits_set(&self) -> u64 {
        self.bits_set
    }

    /// Fill ratio: fraction of bits set to 1.
    pub fn fill_ratio(&self) -> f64 {
        if self.bits.is_empty() {
            return 0.0;
        }
        self.bits_set as f64 / self.bits.len() as f64
    }

    /// Estimated false positive rate based on current fill.
    /// Formula: (bits_set / m) ^ k
    pub fn estimated_fp_rate(&self) -> f64 {
        if self.bits.is_empty() {
            return 1.0;
        }
        let fill = self.bits_set as f64 / self.bits.len() as f64;
        fill.powi(self.num_hashes as i32)
    }

    /// Clear all bits and reset stats.
    pub fn clear(&mut self) {
        self.bits.fill(false);
        self.inserted = 0;
        self.queries = 0;
        self.positives = 0;
        self.bits_set = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filter() {
        let bf = BloomFilter::new(100, 3);
        assert_eq!(bf.num_bits(), 100);
        assert_eq!(bf.num_hashes(), 3);
        assert_eq!(bf.inserted(), 0);
        assert_eq!(bf.bits_set(), 0);
    }

    #[test]
    fn test_insert_and_query() {
        let mut bf = BloomFilter::new(1000, 5);
        bf.insert(b"hello");
        assert!(bf.might_contain(b"hello"));
        assert_eq!(bf.inserted(), 1);
        assert_eq!(bf.queries(), 1);
        assert_eq!(bf.positives(), 1);
    }

    #[test]
    fn test_definitely_not_present() {
        let mut bf = BloomFilter::new(10000, 7);
        bf.insert(b"alpha");
        bf.insert(b"beta");
        // Very unlikely to be a false positive with 10000 bits and 7 hashes for 2 items
        let result = bf.might_contain(b"gamma_definitely_not_here_xyz_123");
        // We can't guarantee false, but with these parameters it's extremely unlikely
        assert_eq!(bf.queries(), 1);
        if !result {
            assert_eq!(bf.positives(), 0);
        }
    }

    #[test]
    fn test_with_rate() {
        let bf = BloomFilter::with_rate(100, 0.01);
        assert!(bf.num_bits() > 0);
        assert!(bf.num_hashes() > 0);
        // For 100 items at 1% FP rate, should have ~958 bits and ~7 hashes
        assert!(bf.num_bits() > 500);
    }

    #[test]
    fn test_fill_ratio() {
        let mut bf = BloomFilter::new(100, 3);
        assert!((bf.fill_ratio() - 0.0).abs() < f64::EPSILON);
        bf.insert(b"test");
        assert!(bf.fill_ratio() > 0.0);
        assert!(bf.fill_ratio() <= 1.0);
    }

    #[test]
    fn test_estimated_fp_rate_empty() {
        let bf = BloomFilter::new(100, 3);
        assert!((bf.estimated_fp_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clear() {
        let mut bf = BloomFilter::new(100, 3);
        bf.insert(b"data1");
        bf.insert(b"data2");
        bf.clear();
        assert_eq!(bf.inserted(), 0);
        assert_eq!(bf.bits_set(), 0);
        assert!(!bf.might_contain(b"data1"));
    }

    #[test]
    fn test_contains_no_track() {
        let mut bf = BloomFilter::new(1000, 5);
        bf.insert(b"tracked");
        assert!(bf.contains_no_track(b"tracked"));
        assert_eq!(bf.queries(), 0); // no tracking
    }

    #[test]
    fn test_multiple_inserts_same_key() {
        let mut bf = BloomFilter::new(100, 3);
        bf.insert(b"same");
        let bits_after_first = bf.bits_set();
        bf.insert(b"same");
        assert_eq!(bf.bits_set(), bits_after_first); // no new bits set
        assert_eq!(bf.inserted(), 2); // count still increments
    }

    #[test]
    fn test_estimated_fp_rate_increases_with_fill() {
        let mut bf = BloomFilter::new(100, 3);
        let rate_empty = bf.estimated_fp_rate();
        for i in 0..50u32 {
            bf.insert(&i.to_le_bytes());
        }
        let rate_half = bf.estimated_fp_rate();
        assert!(rate_half > rate_empty);
    }
}
