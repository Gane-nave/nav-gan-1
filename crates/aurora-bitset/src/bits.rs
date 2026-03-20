/// A compact bit set backed by a vector of u64 words.
#[derive(Debug, Clone)]
pub struct BitSet {
    words: Vec<u64>,
    capacity: usize,
    count: usize,
    total_sets: u64,
    total_clears: u64,
}

impl BitSet {
    /// Create a new bit set with the given capacity (number of bits).
    pub fn new(capacity: usize) -> Self {
        let num_words = capacity.div_ceil(64);
        Self {
            words: vec![0u64; num_words],
            capacity,
            count: 0,
            total_sets: 0,
            total_clears: 0,
        }
    }

    /// Set a bit at the given index. Returns true if the bit was previously unset.
    pub fn set(&mut self, index: usize) -> bool {
        if index >= self.capacity {
            return false;
        }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        let mask = 1u64 << bit_idx;
        let was_unset = self.words[word_idx] & mask == 0;
        self.words[word_idx] |= mask;
        self.total_sets = self.total_sets.saturating_add(1);
        if was_unset {
            self.count += 1;
        }
        was_unset
    }

    /// Clear a bit at the given index. Returns true if the bit was previously set.
    pub fn clear_bit(&mut self, index: usize) -> bool {
        if index >= self.capacity {
            return false;
        }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        let mask = 1u64 << bit_idx;
        let was_set = self.words[word_idx] & mask != 0;
        self.words[word_idx] &= !mask;
        self.total_clears = self.total_clears.saturating_add(1);
        if was_set {
            self.count -= 1;
        }
        was_set
    }

    /// Test if a bit is set at the given index.
    pub fn test(&self, index: usize) -> bool {
        if index >= self.capacity {
            return false;
        }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.words[word_idx] & (1u64 << bit_idx) != 0
    }

    /// Get the number of set bits.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get the capacity (total number of bits).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the fill ratio (set bits / capacity).
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.count as f64 / self.capacity as f64
    }

    /// Clear all bits.
    pub fn clear_all(&mut self) {
        for word in &mut self.words {
            *word = 0;
        }
        self.count = 0;
    }

    /// Set all bits.
    pub fn set_all(&mut self) {
        for word in &mut self.words {
            *word = u64::MAX;
        }
        // Mask off any extra bits beyond capacity
        let remainder = self.capacity % 64;
        if remainder > 0 {
            let last_idx = self.words.len() - 1;
            self.words[last_idx] = (1u64 << remainder) - 1;
        }
        self.count = self.capacity;
    }

    /// Get the total number of set operations.
    pub fn total_sets(&self) -> u64 {
        self.total_sets
    }

    /// Get the total number of clear operations.
    pub fn total_clears(&self) -> u64 {
        self.total_clears
    }

    /// Get the raw words for set operations.
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Get the first set bit index, or None if empty.
    pub fn first_set(&self) -> Option<usize> {
        for (i, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                let index = i * 64 + bit;
                if index < self.capacity {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Get the last set bit index, or None if empty.
    pub fn last_set(&self) -> Option<usize> {
        for (i, &word) in self.words.iter().enumerate().rev() {
            if word != 0 {
                let bit = 63 - word.leading_zeros() as usize;
                let index = i * 64 + bit;
                if index < self.capacity {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Iterate over all set bit indices.
    pub fn iter_set(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.count);
        for (i, &word) in self.words.iter().enumerate() {
            if word == 0 {
                continue;
            }
            let mut w = word;
            while w != 0 {
                let bit = w.trailing_zeros() as usize;
                let index = i * 64 + bit;
                if index < self.capacity {
                    result.push(index);
                }
                w &= w - 1; // clear lowest set bit
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bitset() {
        let bs = BitSet::new(100);
        assert_eq!(bs.capacity(), 100);
        assert_eq!(bs.count(), 0);
        assert_eq!(bs.fill_ratio(), 0.0);
    }

    #[test]
    fn test_set_and_test() {
        let mut bs = BitSet::new(128);
        assert!(!bs.test(10));
        assert!(bs.set(10)); // was unset
        assert!(bs.test(10));
        assert!(!bs.set(10)); // already set
        assert_eq!(bs.count(), 1);
    }

    #[test]
    fn test_clear_bit() {
        let mut bs = BitSet::new(64);
        bs.set(5);
        assert!(bs.test(5));
        assert!(bs.clear_bit(5)); // was set
        assert!(!bs.test(5));
        assert!(!bs.clear_bit(5)); // already clear
        assert_eq!(bs.count(), 0);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut bs = BitSet::new(10);
        assert!(!bs.set(10));
        assert!(!bs.set(100));
        assert!(!bs.test(10));
        assert!(!bs.clear_bit(10));
    }

    #[test]
    fn test_fill_ratio() {
        let mut bs = BitSet::new(100);
        for i in 0..50 {
            bs.set(i);
        }
        assert!((bs.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clear_all() {
        let mut bs = BitSet::new(64);
        for i in 0..64 {
            bs.set(i);
        }
        assert_eq!(bs.count(), 64);
        bs.clear_all();
        assert_eq!(bs.count(), 0);
        assert!(!bs.test(0));
    }

    #[test]
    fn test_set_all() {
        let mut bs = BitSet::new(100);
        bs.set_all();
        assert_eq!(bs.count(), 100);
        assert!(bs.test(0));
        assert!(bs.test(99));
        assert!(!bs.test(100)); // out of bounds
    }

    #[test]
    fn test_first_last_set() {
        let mut bs = BitSet::new(200);
        assert_eq!(bs.first_set(), None);
        assert_eq!(bs.last_set(), None);
        bs.set(50);
        bs.set(150);
        assert_eq!(bs.first_set(), Some(50));
        assert_eq!(bs.last_set(), Some(150));
    }

    #[test]
    fn test_iter_set() {
        let mut bs = BitSet::new(100);
        bs.set(3);
        bs.set(10);
        bs.set(63);
        bs.set(64);
        assert_eq!(bs.iter_set(), vec![3, 10, 63, 64]);
    }

    #[test]
    fn test_stats() {
        let mut bs = BitSet::new(64);
        bs.set(0);
        bs.set(1);
        bs.clear_bit(0);
        assert_eq!(bs.total_sets(), 2);
        assert_eq!(bs.total_clears(), 1);
    }

    #[test]
    fn test_zero_capacity() {
        let bs = BitSet::new(0);
        assert_eq!(bs.capacity(), 0);
        assert_eq!(bs.count(), 0);
        assert_eq!(bs.fill_ratio(), 0.0);
    }
}
