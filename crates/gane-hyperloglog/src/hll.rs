//! HyperLogLog for cardinality estimation.

use std::hash::{Hash, Hasher};

/// HyperLogLog probabilistic cardinality estimator.
pub struct HyperLogLog {
    registers: Vec<u8>,
    precision: u8,
    num_registers: usize,
}

impl HyperLogLog {
    /// Create a new HyperLogLog with the given precision (4..=16).
    /// Higher precision = more accuracy but more memory.
    pub fn new(precision: u8) -> Self {
        let precision = precision.clamp(4, 16);
        let num_registers = 1 << precision;
        Self {
            registers: vec![0; num_registers],
            precision,
            num_registers,
        }
    }

    /// Add an item to the estimator.
    pub fn add<T: Hash>(&mut self, item: &T) {
        let hash = Self::hash_item(item);
        let index = (hash >> (64 - self.precision)) as usize;
        let remaining = (hash << self.precision) | (1 << (self.precision - 1));
        let rank = remaining.leading_zeros() as u8 + 1;
        if rank > self.registers[index] {
            self.registers[index] = rank;
        }
    }

    /// Estimate the number of distinct items added.
    pub fn estimate(&self) -> f64 {
        let m = self.num_registers as f64;
        let alpha = self.alpha();

        let sum: f64 = self
            .registers
            .iter()
            .map(|&r| 2.0_f64.powi(-(r as i32)))
            .sum();
        let raw = alpha * m * m / sum;

        // Small range correction
        if raw <= 2.5 * m {
            let zeros = self.registers.iter().filter(|&&r| r == 0).count();
            if zeros > 0 {
                m * (m / zeros as f64).ln()
            } else {
                raw
            }
        } else if raw <= (1u64 << 32) as f64 / 30.0 {
            raw
        } else {
            // Large range correction
            let two32 = (1u64 << 32) as f64;
            -two32 * (1.0 - raw / two32).ln()
        }
    }

    /// Merge another HyperLogLog into this one.
    /// Both must have the same precision.
    pub fn merge(&mut self, other: &HyperLogLog) {
        if self.precision != other.precision {
            return;
        }
        for i in 0..self.num_registers {
            if other.registers[i] > self.registers[i] {
                self.registers[i] = other.registers[i];
            }
        }
    }

    /// Clear all registers.
    pub fn clear(&mut self) {
        for r in &mut self.registers {
            *r = 0;
        }
    }

    /// Number of registers.
    pub fn num_registers(&self) -> usize {
        self.num_registers
    }

    /// Precision bits.
    pub fn precision(&self) -> u8 {
        self.precision
    }

    /// Memory usage in bytes.
    pub fn memory_bytes(&self) -> usize {
        self.registers.len()
    }

    fn alpha(&self) -> f64 {
        match self.num_registers {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / self.num_registers as f64),
        }
    }

    fn hash_item<T: Hash>(item: &T) -> u64 {
        let mut h = FnvHasher::new();
        item.hash(&mut h);
        h.finish()
    }
}

struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
}

impl Hasher for FnvHasher {
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
    fn test_new() {
        let hll = HyperLogLog::new(10);
        assert_eq!(hll.precision(), 10);
        assert_eq!(hll.num_registers(), 1024);
    }

    #[test]
    fn test_empty_estimate() {
        let hll = HyperLogLog::new(10);
        assert_eq!(hll.estimate(), 0.0);
    }

    #[test]
    fn test_single_item() {
        let mut hll = HyperLogLog::new(10);
        hll.add(&42);
        assert!(hll.estimate() >= 0.5);
        assert!(hll.estimate() <= 2.0);
    }

    #[test]
    fn test_multiple_items() {
        let mut hll = HyperLogLog::new(14);
        for i in 0..1000 {
            hll.add(&i);
        }
        let est = hll.estimate();
        assert!(est > 800.0, "estimate {} too low", est);
        assert!(est < 1200.0, "estimate {} too high", est);
    }

    #[test]
    fn test_duplicate_items() {
        let mut hll = HyperLogLog::new(14);
        for _ in 0..1000 {
            hll.add(&42);
        }
        assert!(hll.estimate() < 5.0);
    }

    #[test]
    fn test_merge() {
        let mut hll1 = HyperLogLog::new(10);
        let mut hll2 = HyperLogLog::new(10);
        for i in 0..500 {
            hll1.add(&i);
        }
        for i in 500..1000 {
            hll2.add(&i);
        }
        hll1.merge(&hll2);
        let est = hll1.estimate();
        assert!(est > 700.0, "merged estimate {} too low", est);
        assert!(est < 1500.0, "merged estimate {} too high", est);
    }

    #[test]
    fn test_clear() {
        let mut hll = HyperLogLog::new(10);
        hll.add(&1);
        hll.add(&2);
        hll.clear();
        assert_eq!(hll.estimate(), 0.0);
    }

    #[test]
    fn test_memory_bytes() {
        let hll = HyperLogLog::new(10);
        assert_eq!(hll.memory_bytes(), 1024);
    }

    #[test]
    fn test_precision_clamping() {
        let hll_low = HyperLogLog::new(1);
        assert_eq!(hll_low.precision(), 4);
        let hll_high = HyperLogLog::new(20);
        assert_eq!(hll_high.precision(), 16);
    }

    #[test]
    fn test_string_items() {
        let mut hll = HyperLogLog::new(14);
        for i in 0..500 {
            hll.add(&format!("item_{}", i));
        }
        let est = hll.estimate();
        assert!(est > 300.0);
        assert!(est < 800.0);
    }
}
