//! Token bucket rate limiter — classic algorithm with configurable capacity and refill rate.

use std::collections::HashMap;

/// Token bucket configuration.
#[derive(Debug, Clone)]
pub struct BucketConfig {
    /// Maximum number of tokens.
    pub capacity: u64,
    /// Tokens added per second.
    pub refill_rate: f64,
    /// Initial token count (defaults to capacity if None).
    pub initial_tokens: Option<u64>,
}

impl BucketConfig {
    /// Create a new bucket configuration.
    pub fn new(capacity: u64, refill_rate: f64) -> Self {
        Self {
            capacity,
            refill_rate,
            initial_tokens: None,
        }
    }

    /// Set initial tokens.
    pub fn with_initial_tokens(mut self, tokens: u64) -> Self {
        self.initial_tokens = Some(tokens);
        self
    }
}

/// A single token bucket.
#[derive(Debug, Clone)]
pub struct TokenBucket {
    /// Current token count (fractional for precise refill).
    tokens: f64,
    /// Maximum capacity.
    capacity: u64,
    /// Tokens per second.
    refill_rate: f64,
    /// Last refill timestamp (epoch millis).
    last_refill_ms: u64,
    /// Total requests accepted.
    accepted: u64,
    /// Total requests rejected.
    rejected: u64,
}

impl TokenBucket {
    /// Create a new token bucket.
    pub fn new(config: &BucketConfig, now_ms: u64) -> Self {
        let initial = config.initial_tokens.unwrap_or(config.capacity);
        Self {
            tokens: initial as f64,
            capacity: config.capacity,
            refill_rate: config.refill_rate,
            last_refill_ms: now_ms,
            accepted: 0,
            rejected: 0,
        }
    }

    /// Try to consume one token. Returns true if allowed.
    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        self.try_acquire_n(1, now_ms)
    }

    /// Try to consume N tokens. Returns true if allowed.
    pub fn try_acquire_n(&mut self, n: u64, now_ms: u64) -> bool {
        self.refill(now_ms);
        let cost = n as f64;
        if self.tokens >= cost {
            self.tokens -= cost;
            self.accepted += n;
            true
        } else {
            self.rejected += n;
            false
        }
    }

    /// Refill tokens based on elapsed time.
    fn refill(&mut self, now_ms: u64) {
        if now_ms <= self.last_refill_ms {
            return;
        }
        let elapsed_secs = (now_ms - self.last_refill_ms) as f64 / 1000.0;
        let new_tokens = elapsed_secs * self.refill_rate;
        self.tokens = (self.tokens + new_tokens).min(self.capacity as f64);
        self.last_refill_ms = now_ms;
    }

    /// Current available tokens.
    pub fn available_tokens(&self) -> u64 {
        self.tokens as u64
    }

    /// Acceptance rate (0.0–1.0).
    pub fn acceptance_rate(&self) -> f64 {
        let total = self.accepted + self.rejected;
        if total == 0 {
            return 1.0;
        }
        self.accepted as f64 / total as f64
    }

    /// Total accepted requests.
    pub fn accepted_count(&self) -> u64 {
        self.accepted
    }

    /// Total rejected requests.
    pub fn rejected_count(&self) -> u64 {
        self.rejected
    }

    /// Reset the bucket to full capacity.
    pub fn reset(&mut self, now_ms: u64) {
        self.tokens = self.capacity as f64;
        self.last_refill_ms = now_ms;
        self.accepted = 0;
        self.rejected = 0;
    }
}

/// Multi-key token bucket rate limiter.
pub struct BucketLimiter {
    buckets: HashMap<String, TokenBucket>,
    default_config: BucketConfig,
}

impl BucketLimiter {
    /// Create a new bucket limiter with default config.
    pub fn new(default_config: BucketConfig) -> Self {
        Self {
            buckets: HashMap::new(),
            default_config,
        }
    }

    /// Try to acquire a token for a given key.
    pub fn try_acquire(&mut self, key: &str, now_ms: u64) -> bool {
        let config = &self.default_config;
        let bucket = self
            .buckets
            .entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(config, now_ms));
        bucket.try_acquire(now_ms)
    }

    /// Try to acquire N tokens for a given key.
    pub fn try_acquire_n(&mut self, key: &str, n: u64, now_ms: u64) -> bool {
        let config = &self.default_config;
        let bucket = self
            .buckets
            .entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(config, now_ms));
        bucket.try_acquire_n(n, now_ms)
    }

    /// Get the bucket for a specific key.
    pub fn get_bucket(&self, key: &str) -> Option<&TokenBucket> {
        self.buckets.get(key)
    }

    /// Get the total number of tracked keys.
    pub fn key_count(&self) -> usize {
        self.buckets.len()
    }

    /// Remove all buckets.
    pub fn clear(&mut self) {
        self.buckets.clear();
    }

    /// Remove a specific key's bucket.
    pub fn remove_key(&mut self, key: &str) -> bool {
        self.buckets.remove(key).is_some()
    }

    /// Prune keys with zero rejected requests (idle buckets).
    pub fn prune_idle(&mut self) -> usize {
        let before = self.buckets.len();
        self.buckets.retain(|_, b| b.accepted > 0 || b.rejected > 0);
        before - self.buckets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_basic_acquire() {
        let config = BucketConfig::new(10, 1.0);
        let mut bucket = TokenBucket::new(&config, 0);

        for _ in 0..10 {
            assert!(bucket.try_acquire(0));
        }
        assert!(!bucket.try_acquire(0));
        assert_eq!(bucket.accepted_count(), 10);
        assert_eq!(bucket.rejected_count(), 1);
    }

    #[test]
    fn test_bucket_refill() {
        let config = BucketConfig::new(10, 5.0);
        let mut bucket = TokenBucket::new(&config, 0);

        // Drain all tokens
        for _ in 0..10 {
            bucket.try_acquire(0);
        }
        assert!(!bucket.try_acquire(0));

        // After 1 second, 5 tokens should refill
        assert!(bucket.try_acquire(1000));
        assert_eq!(bucket.available_tokens(), 4);
    }

    #[test]
    fn test_bucket_capacity_cap() {
        let config = BucketConfig::new(10, 100.0);
        let mut bucket = TokenBucket::new(&config, 0);

        // Even after a long time, tokens should not exceed capacity
        bucket.refill(10_000);
        assert_eq!(bucket.available_tokens(), 10);
    }

    #[test]
    fn test_bucket_acquire_n() {
        let config = BucketConfig::new(20, 1.0);
        let mut bucket = TokenBucket::new(&config, 0);

        assert!(bucket.try_acquire_n(15, 0));
        assert!(!bucket.try_acquire_n(10, 0));
        assert!(bucket.try_acquire_n(5, 0));
    }

    #[test]
    fn test_bucket_acceptance_rate() {
        let config = BucketConfig::new(5, 0.0);
        let mut bucket = TokenBucket::new(&config, 0);

        for _ in 0..5 {
            bucket.try_acquire(0);
        }
        for _ in 0..5 {
            bucket.try_acquire(0);
        }
        assert!((bucket.acceptance_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bucket_reset() {
        let config = BucketConfig::new(10, 1.0);
        let mut bucket = TokenBucket::new(&config, 0);

        for _ in 0..10 {
            bucket.try_acquire(0);
        }
        assert!(!bucket.try_acquire(0));

        bucket.reset(1000);
        assert_eq!(bucket.available_tokens(), 10);
        assert_eq!(bucket.accepted_count(), 0);
        assert_eq!(bucket.rejected_count(), 0);
    }

    #[test]
    fn test_bucket_initial_tokens() {
        let config = BucketConfig::new(100, 1.0).with_initial_tokens(5);
        let bucket = TokenBucket::new(&config, 0);
        assert_eq!(bucket.available_tokens(), 5);
    }

    #[test]
    fn test_bucket_limiter_multi_key() {
        let config = BucketConfig::new(3, 0.0);
        let mut limiter = BucketLimiter::new(config);

        assert!(limiter.try_acquire("user1", 0));
        assert!(limiter.try_acquire("user2", 0));
        assert_eq!(limiter.key_count(), 2);

        for _ in 0..2 {
            limiter.try_acquire("user1", 0);
        }
        assert!(!limiter.try_acquire("user1", 0));
        assert!(limiter.try_acquire("user2", 0));
    }

    #[test]
    fn test_bucket_limiter_remove() {
        let config = BucketConfig::new(10, 1.0);
        let mut limiter = BucketLimiter::new(config);
        limiter.try_acquire("k1", 0);
        assert_eq!(limiter.key_count(), 1);
        assert!(limiter.remove_key("k1"));
        assert_eq!(limiter.key_count(), 0);
    }
}
