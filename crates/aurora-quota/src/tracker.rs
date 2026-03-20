//! Usage tracker — monitors resource consumption over time.

use std::collections::HashMap;

/// A usage sample at a point in time.
#[derive(Debug, Clone)]
pub struct UsageSample {
    /// Timestamp in epoch milliseconds.
    pub timestamp_ms: u64,
    /// Usage value.
    pub value: u64,
}

/// Tracks resource usage over time for a named resource.
pub struct UsageTracker {
    /// Resource name.
    name: String,
    /// Current usage value.
    current: u64,
    /// Peak usage observed.
    peak: u64,
    /// Historical samples.
    samples: Vec<UsageSample>,
    /// Maximum number of samples to retain.
    max_samples: usize,
    /// Total increments (lifetime).
    total_increments: u64,
    /// Total decrements (lifetime).
    total_decrements: u64,
}

impl UsageTracker {
    /// Create a new usage tracker.
    pub fn new(name: &str, max_samples: usize) -> Self {
        Self {
            name: name.to_string(),
            current: 0,
            peak: 0,
            samples: Vec::new(),
            max_samples,
            total_increments: 0,
            total_decrements: 0,
        }
    }

    /// Increment usage.
    pub fn increment(&mut self, amount: u64, timestamp_ms: u64) {
        self.current += amount;
        self.total_increments += amount;
        if self.current > self.peak {
            self.peak = self.current;
        }
        self.record_sample(timestamp_ms);
    }

    /// Decrement usage.
    pub fn decrement(&mut self, amount: u64, timestamp_ms: u64) {
        self.current = self.current.saturating_sub(amount);
        self.total_decrements += amount;
        self.record_sample(timestamp_ms);
    }

    /// Set usage to an absolute value.
    pub fn set(&mut self, value: u64, timestamp_ms: u64) {
        self.current = value;
        if value > self.peak {
            self.peak = value;
        }
        self.record_sample(timestamp_ms);
    }

    /// Current usage value.
    pub fn current(&self) -> u64 {
        self.current
    }

    /// Peak usage observed.
    pub fn peak(&self) -> u64 {
        self.peak
    }

    /// Resource name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Average usage over collected samples.
    pub fn average(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.samples.iter().map(|s| s.value).sum();
        sum as f64 / self.samples.len() as f64
    }

    /// Total lifetime increments.
    pub fn total_increments(&self) -> u64 {
        self.total_increments
    }

    /// Total lifetime decrements.
    pub fn total_decrements(&self) -> u64 {
        self.total_decrements
    }

    /// Number of samples.
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Reset tracker to zero.
    pub fn reset(&mut self) {
        self.current = 0;
        self.peak = 0;
        self.samples.clear();
        self.total_increments = 0;
        self.total_decrements = 0;
    }

    fn record_sample(&mut self, timestamp_ms: u64) {
        self.samples.push(UsageSample {
            timestamp_ms,
            value: self.current,
        });
        while self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
    }
}

/// Manages multiple usage trackers.
pub struct UsageRegistry {
    trackers: HashMap<String, UsageTracker>,
    default_max_samples: usize,
}

impl UsageRegistry {
    /// Create a new registry.
    pub fn new(default_max_samples: usize) -> Self {
        Self {
            trackers: HashMap::new(),
            default_max_samples,
        }
    }

    /// Register a new tracker.
    pub fn register(&mut self, name: &str) {
        self.trackers.insert(
            name.to_string(),
            UsageTracker::new(name, self.default_max_samples),
        );
    }

    /// Get a tracker by name.
    pub fn get(&self, name: &str) -> Option<&UsageTracker> {
        self.trackers.get(name)
    }

    /// Get a mutable tracker by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut UsageTracker> {
        self.trackers.get_mut(name)
    }

    /// Number of registered trackers.
    pub fn count(&self) -> usize {
        self.trackers.len()
    }

    /// All tracker names.
    pub fn names(&self) -> Vec<&str> {
        self.trackers.keys().map(|k| k.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_new() {
        let tracker = UsageTracker::new("cpu", 100);
        assert_eq!(tracker.current(), 0);
        assert_eq!(tracker.peak(), 0);
        assert_eq!(tracker.name(), "cpu");
    }

    #[test]
    fn test_increment() {
        let mut tracker = UsageTracker::new("mem", 100);
        tracker.increment(50, 1000);
        assert_eq!(tracker.current(), 50);
        tracker.increment(30, 2000);
        assert_eq!(tracker.current(), 80);
        assert_eq!(tracker.peak(), 80);
    }

    #[test]
    fn test_decrement() {
        let mut tracker = UsageTracker::new("mem", 100);
        tracker.increment(100, 1000);
        tracker.decrement(30, 2000);
        assert_eq!(tracker.current(), 70);
        assert_eq!(tracker.peak(), 100); // peak unchanged
    }

    #[test]
    fn test_decrement_saturating() {
        let mut tracker = UsageTracker::new("mem", 100);
        tracker.increment(10, 1000);
        tracker.decrement(50, 2000); // should not underflow
        assert_eq!(tracker.current(), 0);
    }

    #[test]
    fn test_set_absolute() {
        let mut tracker = UsageTracker::new("cpu", 100);
        tracker.set(75, 1000);
        assert_eq!(tracker.current(), 75);
        assert_eq!(tracker.peak(), 75);
    }

    #[test]
    fn test_average() {
        let mut tracker = UsageTracker::new("cpu", 100);
        tracker.set(10, 1000);
        tracker.set(20, 2000);
        tracker.set(30, 3000);
        assert!((tracker.average() - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sample_retention() {
        let mut tracker = UsageTracker::new("cpu", 3);
        tracker.set(10, 1000);
        tracker.set(20, 2000);
        tracker.set(30, 3000);
        tracker.set(40, 4000); // evicts oldest
        assert_eq!(tracker.sample_count(), 3);
    }

    #[test]
    fn test_total_counters() {
        let mut tracker = UsageTracker::new("io", 100);
        tracker.increment(100, 1000);
        tracker.increment(50, 2000);
        tracker.decrement(30, 3000);
        assert_eq!(tracker.total_increments(), 150);
        assert_eq!(tracker.total_decrements(), 30);
    }

    #[test]
    fn test_reset() {
        let mut tracker = UsageTracker::new("io", 100);
        tracker.increment(100, 1000);
        tracker.reset();
        assert_eq!(tracker.current(), 0);
        assert_eq!(tracker.peak(), 0);
        assert_eq!(tracker.sample_count(), 0);
    }

    #[test]
    fn test_registry() {
        let mut reg = UsageRegistry::new(100);
        reg.register("cpu");
        reg.register("mem");

        assert_eq!(reg.count(), 2);
        assert!(reg.get("cpu").is_some());

        reg.get_mut("cpu").unwrap().increment(50, 1000);
        assert_eq!(reg.get("cpu").unwrap().current(), 50);
    }
}
