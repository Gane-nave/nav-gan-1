//! Rate limiting — token bucket and sliding window algorithms for
//! protecting backend services from excessive traffic.

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rate limit strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitStrategy {
    /// Token bucket: allows bursts up to capacity, refills at fixed rate.
    TokenBucket,
    /// Sliding window: counts requests in a rolling time window.
    SlidingWindow,
}

/// Rate limit configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub strategy: LimitStrategy,
    /// Maximum requests per window (sliding) or bucket capacity (token).
    pub max_requests: u64,
    /// Window duration in seconds.
    pub window_s: u64,
    /// Refill rate (tokens per second) for token bucket.
    pub refill_rate: f64,
}

/// Result of a rate limit check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitResult {
    /// Request is allowed. Contains remaining quota.
    Allowed(u64),
    /// Request is denied. Contains retry-after in seconds.
    Denied(u64),
}

/// Internal state for a token bucket.
struct TokenBucketState {
    tokens: f64,
    last_refill: DateTime<Utc>,
    capacity: f64,
    refill_rate: f64,
}

/// Internal state for a sliding window.
struct SlidingWindowState {
    timestamps: Vec<DateTime<Utc>>,
    max_requests: u64,
    window_s: u64,
}

/// Per-key limiter state.
enum LimiterState {
    TokenBucket(TokenBucketState),
    SlidingWindow(SlidingWindowState),
}

/// Rate limiter that tracks per-key request rates.
pub struct RateLimiter {
    config: RateLimitConfig,
    states: Mutex<HashMap<String, LimiterState>>,
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            states: Mutex::new(HashMap::new()),
        }
    }

    /// Check if a request from the given key is allowed.
    pub fn check(&self, key: &str) -> LimitResult {
        self.check_at(key, Utc::now())
    }

    /// Check with explicit timestamp (for testing).
    pub fn check_at(&self, key: &str, now: DateTime<Utc>) -> LimitResult {
        let mut states = self.states.lock();
        let state = states
            .entry(key.to_string())
            .or_insert_with(|| self.create_state(now));

        match state {
            LimiterState::TokenBucket(ref mut tb) => {
                // Refill tokens
                let elapsed = (now - tb.last_refill).num_milliseconds().max(0) as f64 / 1000.0;
                tb.tokens = (tb.tokens + elapsed * tb.refill_rate).min(tb.capacity);
                tb.last_refill = now;

                if tb.tokens >= 1.0 {
                    tb.tokens -= 1.0;
                    LimitResult::Allowed(tb.tokens as u64)
                } else {
                    let wait = ((1.0 - tb.tokens) / tb.refill_rate).ceil() as u64;
                    LimitResult::Denied(wait.max(1))
                }
            }
            LimiterState::SlidingWindow(ref mut sw) => {
                // Remove expired timestamps
                let window_start = now - chrono::Duration::seconds(sw.window_s as i64);
                sw.timestamps.retain(|t| *t >= window_start);

                if (sw.timestamps.len() as u64) < sw.max_requests {
                    sw.timestamps.push(now);
                    let remaining = sw.max_requests - sw.timestamps.len() as u64;
                    LimitResult::Allowed(remaining)
                } else {
                    let oldest = sw.timestamps.first().copied().unwrap_or(now);
                    let retry_after = ((oldest + chrono::Duration::seconds(sw.window_s as i64))
                        - now)
                        .num_seconds()
                        .max(1) as u64;
                    LimitResult::Denied(retry_after)
                }
            }
        }
    }

    /// Reset the state for a key.
    pub fn reset(&self, key: &str) {
        self.states.lock().remove(key);
    }

    /// Reset all state.
    pub fn reset_all(&self) {
        self.states.lock().clear();
    }

    /// Number of tracked keys.
    pub fn tracked_keys(&self) -> usize {
        self.states.lock().len()
    }

    fn create_state(&self, now: DateTime<Utc>) -> LimiterState {
        match self.config.strategy {
            LimitStrategy::TokenBucket => LimiterState::TokenBucket(TokenBucketState {
                tokens: self.config.max_requests as f64,
                last_refill: now,
                capacity: self.config.max_requests as f64,
                refill_rate: self.config.refill_rate,
            }),
            LimitStrategy::SlidingWindow => LimiterState::SlidingWindow(SlidingWindowState {
                timestamps: Vec::new(),
                max_requests: self.config.max_requests,
                window_s: self.config.window_s,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_token_bucket_allows_burst() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::TokenBucket,
            max_requests: 5,
            window_s: 60,
            refill_rate: 1.0,
        });

        let now = Utc::now();
        for i in 0..5 {
            match limiter.check_at("user1", now) {
                LimitResult::Allowed(rem) => assert_eq!(rem, 4 - i),
                LimitResult::Denied(_) => panic!("should be allowed for burst"),
            }
        }

        // 6th request should be denied
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));
    }

    #[test]
    fn test_token_bucket_refills() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::TokenBucket,
            max_requests: 2,
            window_s: 60,
            refill_rate: 1.0,
        });

        let now = Utc::now();
        // Use both tokens
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));

        // Wait 2 seconds → 2 tokens refilled
        let later = now + Duration::seconds(2);
        assert!(matches!(
            limiter.check_at("user1", later),
            LimitResult::Allowed(_)
        ));
    }

    #[test]
    fn test_sliding_window_allows_within_limit() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::SlidingWindow,
            max_requests: 3,
            window_s: 60,
            refill_rate: 0.0,
        });

        let now = Utc::now();
        for _ in 0..3 {
            assert!(matches!(
                limiter.check_at("user1", now),
                LimitResult::Allowed(_)
            ));
        }
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));
    }

    #[test]
    fn test_sliding_window_expires() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::SlidingWindow,
            max_requests: 2,
            window_s: 10,
            refill_rate: 0.0,
        });

        let now = Utc::now();
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));

        // After window expires, should be allowed again
        let later = now + Duration::seconds(11);
        assert!(matches!(
            limiter.check_at("user1", later),
            LimitResult::Allowed(_)
        ));
    }

    #[test]
    fn test_per_key_isolation() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::TokenBucket,
            max_requests: 1,
            window_s: 60,
            refill_rate: 0.1,
        });

        let now = Utc::now();
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));
        // Different key should still be allowed
        assert!(matches!(
            limiter.check_at("user2", now),
            LimitResult::Allowed(_)
        ));
    }

    #[test]
    fn test_reset() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::TokenBucket,
            max_requests: 1,
            window_s: 60,
            refill_rate: 0.1,
        });

        let now = Utc::now();
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Denied(_)
        ));
        limiter.reset("user1");
        assert!(matches!(
            limiter.check_at("user1", now),
            LimitResult::Allowed(_)
        ));
    }

    #[test]
    fn test_tracked_keys() {
        let limiter = RateLimiter::new(RateLimitConfig {
            strategy: LimitStrategy::SlidingWindow,
            max_requests: 10,
            window_s: 60,
            refill_rate: 0.0,
        });
        limiter.check("a");
        limiter.check("b");
        limiter.check("c");
        assert_eq!(limiter.tracked_keys(), 3);
        limiter.reset_all();
        assert_eq!(limiter.tracked_keys(), 0);
    }

    #[test]
    fn test_serialization() {
        let config = RateLimitConfig {
            strategy: LimitStrategy::TokenBucket,
            max_requests: 100,
            window_s: 60,
            refill_rate: 10.0,
        };
        let json = serde_json::to_string(&config).unwrap();
        let de: RateLimitConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(de.max_requests, 100);
        assert_eq!(de.strategy, LimitStrategy::TokenBucket);
    }
}
