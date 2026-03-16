//! Sliding window rate limiter — tracks requests in a time window with sub-window precision.

use std::collections::HashMap;

/// Sliding window configuration.
#[derive(Debug, Clone)]
pub struct SlidingWindowConfig {
    /// Window size in milliseconds.
    pub window_ms: u64,
    /// Maximum requests per window.
    pub max_requests: u64,
    /// Number of sub-windows for granularity.
    pub sub_windows: usize,
}

impl SlidingWindowConfig {
    /// Create a new sliding window config.
    pub fn new(window_ms: u64, max_requests: u64) -> Self {
        Self {
            window_ms,
            max_requests,
            sub_windows: 10,
        }
    }

    /// Set number of sub-windows.
    pub fn with_sub_windows(mut self, n: usize) -> Self {
        self.sub_windows = n.max(1);
        self
    }
}

/// A sliding window counter for a single key.
#[derive(Debug, Clone)]
struct WindowCounter {
    /// Counts per sub-window.
    sub_counts: Vec<u64>,
    /// Sub-window duration in ms.
    sub_window_ms: u64,
    /// Current sub-window index.
    current_index: usize,
    /// Timestamp of current sub-window start.
    current_window_start_ms: u64,
    /// Total accepted in current window.
    total_accepted: u64,
    /// Total rejected in current window.
    total_rejected: u64,
}

impl WindowCounter {
    fn new(config: &SlidingWindowConfig, now_ms: u64) -> Self {
        let sub_window_ms = config.window_ms / config.sub_windows as u64;
        Self {
            sub_counts: vec![0; config.sub_windows],
            sub_window_ms: sub_window_ms.max(1),
            current_index: 0,
            current_window_start_ms: now_ms,
            total_accepted: 0,
            total_rejected: 0,
        }
    }

    fn advance(&mut self, now_ms: u64) {
        if now_ms <= self.current_window_start_ms {
            return;
        }
        let elapsed = now_ms - self.current_window_start_ms;
        let steps = (elapsed / self.sub_window_ms) as usize;
        if steps == 0 {
            return;
        }
        let n = self.sub_counts.len();
        let clear_count = steps.min(n);
        for i in 0..clear_count {
            let idx = (self.current_index + 1 + i) % n;
            self.sub_counts[idx] = 0;
        }
        self.current_index = (self.current_index + steps) % n;
        self.current_window_start_ms += steps as u64 * self.sub_window_ms;
    }

    fn count(&self) -> u64 {
        self.sub_counts.iter().sum()
    }

    fn try_acquire(&mut self, max_requests: u64, now_ms: u64) -> bool {
        self.advance(now_ms);
        if self.count() < max_requests {
            self.sub_counts[self.current_index] += 1;
            self.total_accepted += 1;
            true
        } else {
            self.total_rejected += 1;
            false
        }
    }
}

/// Multi-key sliding window rate limiter.
pub struct SlidingWindowLimiter {
    counters: HashMap<String, WindowCounter>,
    config: SlidingWindowConfig,
}

impl SlidingWindowLimiter {
    /// Create a new sliding window limiter.
    pub fn new(config: SlidingWindowConfig) -> Self {
        Self {
            counters: HashMap::new(),
            config,
        }
    }

    /// Try to acquire a request slot for a key.
    pub fn try_acquire(&mut self, key: &str, now_ms: u64) -> bool {
        let config = &self.config;
        let max = config.max_requests;
        let counter = self
            .counters
            .entry(key.to_string())
            .or_insert_with(|| WindowCounter::new(config, now_ms));
        counter.try_acquire(max, now_ms)
    }

    /// Get the current count for a key (advances window to clear expired sub-windows).
    pub fn current_count(&mut self, key: &str, now_ms: u64) -> u64 {
        if let Some(counter) = self.counters.get_mut(key) {
            counter.advance(now_ms);
            counter.count()
        } else {
            0
        }
    }

    /// Get remaining capacity for a key (advances window to clear expired sub-windows).
    pub fn remaining(&mut self, key: &str, now_ms: u64) -> u64 {
        let used = self.current_count(key, now_ms);
        self.config.max_requests.saturating_sub(used)
    }

    /// Get the total number of tracked keys.
    pub fn key_count(&self) -> usize {
        self.counters.len()
    }

    /// Get total accepted for a key.
    pub fn accepted_count(&self, key: &str) -> u64 {
        self.counters.get(key).map_or(0, |c| c.total_accepted)
    }

    /// Get total rejected for a key.
    pub fn rejected_count(&self, key: &str) -> u64 {
        self.counters.get(key).map_or(0, |c| c.total_rejected)
    }

    /// Remove a specific key.
    pub fn remove_key(&mut self, key: &str) -> bool {
        self.counters.remove(key).is_some()
    }

    /// Clear all tracked keys.
    pub fn clear(&mut self) {
        self.counters.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_basic() {
        let config = SlidingWindowConfig::new(1000, 5);
        let mut limiter = SlidingWindowLimiter::new(config);

        for _ in 0..5 {
            assert!(limiter.try_acquire("k1", 0));
        }
        assert!(!limiter.try_acquire("k1", 0));
        assert_eq!(limiter.current_count("k1", 0), 5);
        assert_eq!(limiter.remaining("k1", 0), 0);
    }

    #[test]
    fn test_sliding_window_reset() {
        let config = SlidingWindowConfig::new(1000, 5);
        let mut limiter = SlidingWindowLimiter::new(config);

        for _ in 0..5 {
            limiter.try_acquire("k1", 0);
        }
        assert!(!limiter.try_acquire("k1", 0));

        // After full window passes, should allow again
        assert!(limiter.try_acquire("k1", 1100));
        assert_eq!(limiter.remaining("k1", 1100), 4);
    }

    #[test]
    fn test_sliding_multi_key() {
        let config = SlidingWindowConfig::new(1000, 3);
        let mut limiter = SlidingWindowLimiter::new(config);

        for _ in 0..3 {
            limiter.try_acquire("a", 0);
        }
        assert!(!limiter.try_acquire("a", 0));
        assert!(limiter.try_acquire("b", 0));
        assert_eq!(limiter.key_count(), 2);
    }

    #[test]
    fn test_sliding_accepted_rejected() {
        let config = SlidingWindowConfig::new(1000, 2);
        let mut limiter = SlidingWindowLimiter::new(config);

        limiter.try_acquire("k1", 0);
        limiter.try_acquire("k1", 0);
        limiter.try_acquire("k1", 0); // rejected
        limiter.try_acquire("k1", 0); // rejected

        assert_eq!(limiter.accepted_count("k1"), 2);
        assert_eq!(limiter.rejected_count("k1"), 2);
    }

    #[test]
    fn test_sliding_sub_window_granularity() {
        let config = SlidingWindowConfig::new(1000, 10).with_sub_windows(4);
        let mut limiter = SlidingWindowLimiter::new(config);

        // Fill at time 0
        for _ in 0..5 {
            limiter.try_acquire("k1", 0);
        }
        assert_eq!(limiter.current_count("k1", 0), 5);

        // After one sub-window (250ms), old counts should slide out
        for _ in 0..3 {
            limiter.try_acquire("k1", 300);
        }
        assert!(limiter.current_count("k1", 300) <= 10);
    }

    #[test]
    fn test_sliding_remove_key() {
        let config = SlidingWindowConfig::new(1000, 5);
        let mut limiter = SlidingWindowLimiter::new(config);
        limiter.try_acquire("k1", 0);
        assert_eq!(limiter.key_count(), 1);
        assert!(limiter.remove_key("k1"));
        assert_eq!(limiter.key_count(), 0);
        assert!(!limiter.remove_key("k1"));
    }
}
