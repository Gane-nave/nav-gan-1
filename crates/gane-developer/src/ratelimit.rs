//! Rate limiting — token-bucket and sliding-window rate limiters
//! for controlling API access per key, per tier, and per endpoint.

use chrono::{DateTime, Duration, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

// ---------------------------------------------------------------------------
// Rate limiter
// ---------------------------------------------------------------------------

/// Per-key rate limiter using a token-bucket algorithm.
pub struct RateLimiter {
    buckets: HashMap<EntityId, TokenBucket>,
    default_capacity: u64,
    default_refill_rate: u64,
}

impl RateLimiter {
    /// Create a new rate limiter with default bucket parameters.
    pub fn new(default_capacity: u64, default_refill_per_second: u64) -> Self {
        Self {
            buckets: HashMap::new(),
            default_capacity,
            default_refill_rate: default_refill_per_second,
        }
    }

    /// Configure a custom bucket for a specific key.
    pub fn configure_key(&mut self, key_id: EntityId, capacity: u64, refill_per_second: u64) {
        let bucket = TokenBucket::new(capacity, refill_per_second);
        debug!(key = %key_id, capacity, refill_per_second, "rate limit configured");
        self.buckets.insert(key_id, bucket);
    }

    /// Try to consume one token for the given key.
    /// Returns Ok(remaining) or Err(retry_after).
    pub fn try_acquire(&mut self, key_id: &EntityId) -> Result<RateLimitResult, RateLimitResult> {
        let bucket = self
            .buckets
            .entry(*key_id)
            .or_insert_with(|| TokenBucket::new(self.default_capacity, self.default_refill_rate));

        bucket.refill();

        if bucket.tokens >= 1 {
            bucket.tokens -= 1;
            bucket.total_consumed += 1;
            Ok(RateLimitResult {
                allowed: true,
                remaining: bucket.tokens,
                limit: bucket.capacity,
                retry_after_ms: 0,
                reset_at: bucket.last_refill + Duration::seconds(1),
            })
        } else {
            let retry_after_ms = 1000u64
                .checked_div(bucket.refill_per_second)
                .unwrap_or(1000);
            warn!(key = %key_id, "rate limited");
            Err(RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: bucket.capacity,
                retry_after_ms,
                reset_at: bucket.last_refill + Duration::seconds(1),
            })
        }
    }

    /// Get current bucket status for a key without consuming tokens.
    pub fn status(&mut self, key_id: &EntityId) -> Option<RateLimitResult> {
        let bucket = self.buckets.get_mut(key_id)?;
        bucket.refill();
        Some(RateLimitResult {
            allowed: bucket.tokens > 0,
            remaining: bucket.tokens,
            limit: bucket.capacity,
            retry_after_ms: 0,
            reset_at: bucket.last_refill + Duration::seconds(1),
        })
    }

    /// Reset a key's bucket to full capacity.
    pub fn reset(&mut self, key_id: &EntityId) -> bool {
        if let Some(bucket) = self.buckets.get_mut(key_id) {
            bucket.tokens = bucket.capacity;
            bucket.last_refill = Utc::now();
            debug!(key = %key_id, "rate limit reset");
            true
        } else {
            false
        }
    }

    /// Total tokens consumed across all keys.
    pub fn total_consumed(&self) -> u64 {
        self.buckets.values().map(|b| b.total_consumed).sum()
    }

    /// Number of tracked keys.
    pub fn tracked_keys(&self) -> usize {
        self.buckets.len()
    }
}

/// Result of a rate limit check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u64,
    pub limit: u64,
    pub retry_after_ms: u64,
    pub reset_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Token bucket
// ---------------------------------------------------------------------------

/// Token bucket implementation for rate limiting.
struct TokenBucket {
    tokens: u64,
    capacity: u64,
    refill_per_second: u64,
    last_refill: DateTime<Utc>,
    total_consumed: u64,
}

impl TokenBucket {
    fn new(capacity: u64, refill_per_second: u64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_per_second,
            last_refill: Utc::now(),
            total_consumed: 0,
        }
    }

    /// Refill tokens based on elapsed time.
    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed_ms = (now - self.last_refill).num_milliseconds().max(0) as u64;
        if elapsed_ms == 0 {
            return;
        }
        let new_tokens = (elapsed_ms * self.refill_per_second) / 1000;
        if new_tokens > 0 {
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
}

// ---------------------------------------------------------------------------
// Sliding window rate limiter
// ---------------------------------------------------------------------------

/// Sliding window counter for tracking requests over a time window.
pub struct SlidingWindowCounter {
    windows: HashMap<EntityId, Vec<DateTime<Utc>>>,
    window_duration: Duration,
    max_requests: u64,
}

impl SlidingWindowCounter {
    /// Create a new sliding window counter.
    pub fn new(window_duration: Duration, max_requests: u64) -> Self {
        Self {
            windows: HashMap::new(),
            window_duration,
            max_requests,
        }
    }

    /// Record a request and check if it's allowed.
    pub fn record(&mut self, key_id: &EntityId) -> bool {
        let now = Utc::now();
        let cutoff = now - self.window_duration;

        let timestamps = self.windows.entry(*key_id).or_default();

        // Prune old entries.
        timestamps.retain(|t| *t > cutoff);

        if (timestamps.len() as u64) < self.max_requests {
            timestamps.push(now);
            true
        } else {
            false
        }
    }

    /// Current count for a key within the window.
    pub fn current_count(&mut self, key_id: &EntityId) -> u64 {
        let cutoff = Utc::now() - self.window_duration;
        if let Some(timestamps) = self.windows.get_mut(key_id) {
            timestamps.retain(|t| *t > cutoff);
            timestamps.len() as u64
        } else {
            0
        }
    }

    /// Remaining requests allowed for a key.
    pub fn remaining(&mut self, key_id: &EntityId) -> u64 {
        self.max_requests.saturating_sub(self.current_count(key_id))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_bucket_allows_up_to_capacity() {
        let mut limiter = RateLimiter::new(3, 1);
        let key = EntityId::new();

        // Should allow 3 requests (capacity).
        assert!(limiter.try_acquire(&key).is_ok());
        assert!(limiter.try_acquire(&key).is_ok());
        assert!(limiter.try_acquire(&key).is_ok());

        // 4th should be rate limited.
        let result = limiter.try_acquire(&key);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(!err.allowed);
        assert_eq!(err.remaining, 0);
    }

    #[test]
    fn custom_key_configuration() {
        let mut limiter = RateLimiter::new(10, 1);
        let key = EntityId::new();

        // Custom config: only 1 token.
        limiter.configure_key(key, 1, 1);

        assert!(limiter.try_acquire(&key).is_ok());
        assert!(limiter.try_acquire(&key).is_err());
    }

    #[test]
    fn reset_refills_bucket() {
        let mut limiter = RateLimiter::new(2, 1);
        let key = EntityId::new();

        limiter.try_acquire(&key).unwrap();
        limiter.try_acquire(&key).unwrap();
        assert!(limiter.try_acquire(&key).is_err());

        limiter.reset(&key);
        assert!(limiter.try_acquire(&key).is_ok());
    }

    #[test]
    fn status_without_consuming() {
        let mut limiter = RateLimiter::new(5, 1);
        let key = EntityId::new();
        limiter.configure_key(key, 5, 1);

        // Consume one.
        limiter.try_acquire(&key).unwrap();

        let status = limiter.status(&key).unwrap();
        assert!(status.allowed);
        assert_eq!(status.remaining, 4);
        assert_eq!(status.limit, 5);
    }

    #[test]
    fn total_consumed_tracks_all_keys() {
        let mut limiter = RateLimiter::new(10, 1);
        let key_a = EntityId::new();
        let key_b = EntityId::new();

        limiter.try_acquire(&key_a).unwrap();
        limiter.try_acquire(&key_a).unwrap();
        limiter.try_acquire(&key_b).unwrap();

        assert_eq!(limiter.total_consumed(), 3);
        assert_eq!(limiter.tracked_keys(), 2);
    }

    #[test]
    fn sliding_window_enforces_limit() {
        let mut counter = SlidingWindowCounter::new(Duration::minutes(1), 3);
        let key = EntityId::new();

        assert!(counter.record(&key));
        assert!(counter.record(&key));
        assert!(counter.record(&key));
        assert!(!counter.record(&key)); // 4th denied.
    }

    #[test]
    fn sliding_window_counts_and_remaining() {
        let mut counter = SlidingWindowCounter::new(Duration::minutes(1), 5);
        let key = EntityId::new();

        counter.record(&key);
        counter.record(&key);

        assert_eq!(counter.current_count(&key), 2);
        assert_eq!(counter.remaining(&key), 3);
    }

    #[test]
    fn sliding_window_unknown_key() {
        let mut counter = SlidingWindowCounter::new(Duration::minutes(1), 10);
        let key = EntityId::new();
        assert_eq!(counter.current_count(&key), 0);
        assert_eq!(counter.remaining(&key), 10);
    }

    #[test]
    fn reset_nonexistent_key_returns_false() {
        let mut limiter = RateLimiter::new(10, 1);
        assert!(!limiter.reset(&EntityId::new()));
    }

    #[test]
    fn remaining_reports_correct_after_acquire() {
        let mut limiter = RateLimiter::new(5, 0);
        let key = EntityId::new();

        let r = limiter.try_acquire(&key).unwrap();
        assert_eq!(r.remaining, 4);

        let r = limiter.try_acquire(&key).unwrap();
        assert_eq!(r.remaining, 3);
    }
}
