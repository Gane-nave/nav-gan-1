//! Token bucket — classic rate limiter with burst capacity.

/// Token bucket configuration.
#[derive(Debug, Clone)]
pub struct TokenBucketConfig {
    /// Maximum number of tokens (burst capacity).
    pub capacity: u64,
    /// Tokens added per refill interval.
    pub refill_amount: u64,
    /// Refill interval in milliseconds.
    pub refill_interval_ms: u64,
}

impl Default for TokenBucketConfig {
    fn default() -> Self {
        Self {
            capacity: 100,
            refill_amount: 10,
            refill_interval_ms: 1000,
        }
    }
}

/// A token bucket rate limiter.
pub struct TokenBucket {
    config: TokenBucketConfig,
    tokens: u64,
    last_refill_ms: u64,
    total_allowed: u64,
    total_rejected: u64,
}

impl TokenBucket {
    /// Create a new token bucket starting full.
    pub fn new(config: TokenBucketConfig) -> Self {
        let tokens = config.capacity;
        Self {
            config,
            tokens,
            last_refill_ms: 0,
            total_allowed: 0,
            total_rejected: 0,
        }
    }

    /// Try to consume `count` tokens at the given timestamp.
    /// Returns true if tokens were available, false if rejected.
    pub fn try_acquire(&mut self, count: u64, now_ms: u64) -> bool {
        self.refill(now_ms);
        if self.tokens >= count {
            self.tokens -= count;
            self.total_allowed += 1;
            true
        } else {
            self.total_rejected += 1;
            false
        }
    }

    /// Current number of available tokens.
    pub fn available(&self) -> u64 {
        self.tokens
    }

    /// Utilization ratio (0.0 = full, 1.0 = empty).
    pub fn utilization(&self) -> f64 {
        if self.config.capacity == 0 {
            return 1.0;
        }
        1.0 - (self.tokens as f64 / self.config.capacity as f64)
    }

    /// Total requests allowed.
    pub fn total_allowed(&self) -> u64 {
        self.total_allowed
    }

    /// Total requests rejected.
    pub fn total_rejected(&self) -> u64 {
        self.total_rejected
    }

    /// Refill tokens based on elapsed time.
    fn refill(&mut self, now_ms: u64) {
        if self.last_refill_ms == 0 {
            self.last_refill_ms = now_ms;
            return;
        }

        let elapsed = now_ms.saturating_sub(self.last_refill_ms);
        if elapsed >= self.config.refill_interval_ms && self.config.refill_interval_ms > 0 {
            let intervals = elapsed / self.config.refill_interval_ms;
            let new_tokens = intervals * self.config.refill_amount;
            self.tokens = (self.tokens + new_tokens).min(self.config.capacity);
            self.last_refill_ms += intervals * self.config.refill_interval_ms;
        }
    }

    /// Reset the bucket to full capacity.
    pub fn reset(&mut self) {
        self.tokens = self.config.capacity;
        self.last_refill_ms = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_full() {
        let bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            ..Default::default()
        });
        assert_eq!(bucket.available(), 10);
    }

    #[test]
    fn test_acquire_consumes_tokens() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            ..Default::default()
        });
        assert!(bucket.try_acquire(3, 100));
        assert_eq!(bucket.available(), 7);
    }

    #[test]
    fn test_acquire_rejects_when_empty() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 5,
            ..Default::default()
        });
        assert!(bucket.try_acquire(5, 100));
        assert!(!bucket.try_acquire(1, 200));
        assert_eq!(bucket.total_rejected(), 1);
    }

    #[test]
    fn test_refill_adds_tokens() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            refill_amount: 3,
            refill_interval_ms: 1000,
        });
        assert!(bucket.try_acquire(10, 100)); // empty the bucket
        assert_eq!(bucket.available(), 0);

        // After 1 interval, should have 3 tokens
        assert!(bucket.try_acquire(1, 1200));
        assert_eq!(bucket.available(), 2); // 3 refilled - 1 consumed
    }

    #[test]
    fn test_refill_capped_at_capacity() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            refill_amount: 100,
            refill_interval_ms: 1000,
        });
        assert!(bucket.try_acquire(5, 100));
        // After refill, should be capped at 10
        assert!(bucket.try_acquire(1, 1200));
        assert_eq!(bucket.available(), 9); // capped at 10 - 1
    }

    #[test]
    fn test_utilization() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            ..Default::default()
        });
        assert!((bucket.utilization() - 0.0).abs() < f64::EPSILON); // full bucket = 0% utilized
        bucket.try_acquire(5, 100);
        assert!((bucket.utilization() - 0.5).abs() < f64::EPSILON); // half empty = 50% utilized
    }

    #[test]
    fn test_reset() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            ..Default::default()
        });
        bucket.try_acquire(10, 100);
        assert_eq!(bucket.available(), 0);
        bucket.reset();
        assert_eq!(bucket.available(), 10);
    }

    #[test]
    fn test_acquire_more_than_available() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 5,
            ..Default::default()
        });
        assert!(!bucket.try_acquire(6, 100)); // can't acquire more than capacity
    }

    #[test]
    fn test_multiple_refill_intervals() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 10,
            refill_amount: 2,
            refill_interval_ms: 1000,
        });
        assert!(bucket.try_acquire(10, 100)); // empty
                                              // After 3 intervals (3000ms), should have 6 tokens
        assert!(bucket.try_acquire(1, 3200));
        assert_eq!(bucket.available(), 5); // 6 refilled - 1 consumed
    }

    #[test]
    fn test_stats() {
        let mut bucket = TokenBucket::new(TokenBucketConfig {
            capacity: 2,
            ..Default::default()
        });
        bucket.try_acquire(1, 100);
        bucket.try_acquire(1, 200);
        bucket.try_acquire(1, 300); // rejected
        assert_eq!(bucket.total_allowed(), 2);
        assert_eq!(bucket.total_rejected(), 1);
    }
}
