//! Token-bucket rate limiter with per-client and global limits.
//!
//! Provides configurable rate limiting to protect API endpoints from
//! abuse while allowing legitimate burst traffic.

use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("rate limit exceeded: retry after {retry_after_ms}ms")]
    Exceeded { retry_after_ms: u64 },
}

// ---------------------------------------------------------------------------
// Token bucket
// ---------------------------------------------------------------------------

/// A single token bucket for rate limiting.
#[derive(Debug, Clone)]
struct TokenBucket {
    /// Maximum tokens (burst capacity).
    capacity: u64,
    /// Current available tokens.
    tokens: f64,
    /// Tokens added per second.
    refill_rate: f64,
    /// Last refill timestamp.
    last_refill: DateTime<Utc>,
}

impl TokenBucket {
    fn new(capacity: u64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate,
            last_refill: Utc::now(),
        }
    }

    /// Try to consume one token. Returns true if allowed.
    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Estimated milliseconds until a token is available.
    fn retry_after_ms(&self) -> u64 {
        if self.tokens >= 1.0 {
            return 0;
        }
        let needed = 1.0 - self.tokens;
        let seconds = needed / self.refill_rate;
        (seconds * 1000.0).ceil() as u64
    }

    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).num_milliseconds().max(0) as f64 / 1000.0;
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity as f64);
        self.last_refill = now;
    }
}

// ---------------------------------------------------------------------------
// Rate limiter config
// ---------------------------------------------------------------------------

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window per client.
    pub requests_per_second: f64,
    /// Burst capacity (max tokens).
    pub burst_capacity: u64,
    /// Global rate limit (all clients combined).
    pub global_requests_per_second: f64,
    /// Global burst capacity.
    pub global_burst_capacity: u64,
    /// Time-to-live for idle client buckets.
    pub client_ttl: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10.0,
            burst_capacity: 20,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 2000,
            client_ttl: Duration::minutes(10),
        }
    }
}

// ---------------------------------------------------------------------------
// Rate limiter
// ---------------------------------------------------------------------------

/// Thread-safe rate limiter with per-client and global buckets.
pub struct RateLimiter {
    config: RateLimitConfig,
    clients: RwLock<HashMap<String, (TokenBucket, DateTime<Utc>)>>,
    global: RwLock<TokenBucket>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        let global = TokenBucket::new(
            config.global_burst_capacity,
            config.global_requests_per_second,
        );
        Self {
            config,
            clients: RwLock::new(HashMap::new()),
            global: RwLock::new(global),
        }
    }

    /// Check if a request from the given client is allowed.
    ///
    /// Checks per-client limit first, then global limit. Both tokens are
    /// consumed only when both checks pass, preventing an abusive client
    /// from draining the global bucket on rejected requests.
    pub fn check_rate_limit(&self, client_id: &str) -> Result<(), RateLimitError> {
        // Check per-client limit first (before touching global bucket)
        let mut clients = self.clients.write();
        let entry = clients.entry(client_id.to_string()).or_insert_with(|| {
            let bucket =
                TokenBucket::new(self.config.burst_capacity, self.config.requests_per_second);
            (bucket, Utc::now())
        });

        entry.1 = Utc::now(); // Update last-seen time

        if !entry.0.try_consume() {
            return Err(RateLimitError::Exceeded {
                retry_after_ms: entry.0.retry_after_ms(),
            });
        }

        // Per-client passed — now check global limit
        {
            let mut global = self.global.write();
            if !global.try_consume() {
                // Refund the per-client token since global rejected
                entry.0.tokens += 1.0;
                return Err(RateLimitError::Exceeded {
                    retry_after_ms: global.retry_after_ms(),
                });
            }
        }

        Ok(())
    }

    /// Remove idle client buckets that haven't been used within TTL.
    pub fn cleanup_idle_clients(&self) -> usize {
        let mut clients = self.clients.write();
        let cutoff = Utc::now() - self.config.client_ttl;
        let before = clients.len();
        clients.retain(|_, (_, last_seen)| *last_seen > cutoff);
        before - clients.len()
    }

    /// Number of tracked clients.
    pub fn tracked_client_count(&self) -> usize {
        self.clients.read().len()
    }

    /// Reset all rate limits (useful for testing).
    pub fn reset(&self) {
        let mut clients = self.clients.write();
        clients.clear();
        let mut global = self.global.write();
        *global = TokenBucket::new(
            self.config.global_burst_capacity,
            self.config.global_requests_per_second,
        );
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_requests_within_limit() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 100.0,
            burst_capacity: 10,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 100,
            client_ttl: Duration::minutes(10),
        });

        for _ in 0..10 {
            assert!(limiter.check_rate_limit("client-1").is_ok());
        }
    }

    #[test]
    fn rejects_when_burst_exceeded() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 1.0,
            burst_capacity: 3,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 1000,
            client_ttl: Duration::minutes(10),
        });

        assert!(limiter.check_rate_limit("client-1").is_ok());
        assert!(limiter.check_rate_limit("client-1").is_ok());
        assert!(limiter.check_rate_limit("client-1").is_ok());
        assert!(matches!(
            limiter.check_rate_limit("client-1"),
            Err(RateLimitError::Exceeded { .. })
        ));
    }

    #[test]
    fn different_clients_have_separate_buckets() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 1.0,
            burst_capacity: 2,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 1000,
            client_ttl: Duration::minutes(10),
        });

        assert!(limiter.check_rate_limit("client-1").is_ok());
        assert!(limiter.check_rate_limit("client-1").is_ok());
        // client-1 exhausted
        assert!(limiter.check_rate_limit("client-1").is_err());
        // client-2 still has tokens
        assert!(limiter.check_rate_limit("client-2").is_ok());
    }

    #[test]
    fn global_limit_applies() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 100.0,
            burst_capacity: 100,
            global_requests_per_second: 1.0,
            global_burst_capacity: 3,
            client_ttl: Duration::minutes(10),
        });

        assert!(limiter.check_rate_limit("a").is_ok());
        assert!(limiter.check_rate_limit("b").is_ok());
        assert!(limiter.check_rate_limit("c").is_ok());
        // Global exhausted even though per-client is fine
        assert!(limiter.check_rate_limit("d").is_err());
    }

    #[test]
    fn retry_after_is_positive() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 1.0,
            burst_capacity: 1,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 1000,
            client_ttl: Duration::minutes(10),
        });

        limiter.check_rate_limit("client-1").unwrap();
        match limiter.check_rate_limit("client-1") {
            Err(RateLimitError::Exceeded { retry_after_ms }) => {
                assert!(retry_after_ms > 0);
            }
            _ => panic!("expected rate limit exceeded"),
        }
    }

    #[test]
    fn tracked_client_count() {
        let limiter = RateLimiter::default();
        assert_eq!(limiter.tracked_client_count(), 0);
        limiter.check_rate_limit("a").unwrap();
        limiter.check_rate_limit("b").unwrap();
        assert_eq!(limiter.tracked_client_count(), 2);
    }

    #[test]
    fn reset_clears_all() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 1.0,
            burst_capacity: 1,
            global_requests_per_second: 1000.0,
            global_burst_capacity: 1000,
            client_ttl: Duration::minutes(10),
        });

        limiter.check_rate_limit("client-1").unwrap();
        assert!(limiter.check_rate_limit("client-1").is_err());

        limiter.reset();
        assert!(limiter.check_rate_limit("client-1").is_ok());
        assert_eq!(limiter.tracked_client_count(), 1);
    }
}
