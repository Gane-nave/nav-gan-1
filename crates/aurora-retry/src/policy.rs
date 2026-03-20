//! Retry policy — configurable retry behavior with conditions.

use crate::backoff::BackoffStrategy;

/// Whether an operation result should be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    /// Do not retry — operation succeeded or is non-retryable.
    NoRetry,
    /// Retry after the specified delay.
    RetryAfter { delay_ms: u64 },
    /// Exhausted all retry attempts.
    Exhausted,
}

impl RetryDecision {
    /// Whether this decision indicates a retry should happen.
    pub fn should_retry(&self) -> bool {
        matches!(self, RetryDecision::RetryAfter { .. })
    }

    /// Get the delay if retrying.
    pub fn delay(&self) -> Option<u64> {
        match self {
            RetryDecision::RetryAfter { delay_ms } => Some(*delay_ms),
            _ => None,
        }
    }
}

/// Configuration for a retry policy.
#[derive(Debug, Clone)]
pub struct RetryPolicyConfig {
    /// Maximum number of retry attempts.
    pub max_attempts: u32,
    /// Backoff strategy.
    pub backoff: BackoffStrategy,
    /// Whether to retry on timeout errors.
    pub retry_on_timeout: bool,
    /// Whether to retry on transient errors.
    pub retry_on_transient: bool,
}

impl Default for RetryPolicyConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff: BackoffStrategy::exponential(100, 2.0, 30_000),
            retry_on_timeout: true,
            retry_on_transient: true,
        }
    }
}

/// Error classification for retry decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Transient error — safe to retry.
    Transient,
    /// Timeout — may be safe to retry.
    Timeout,
    /// Permanent error — do not retry.
    Permanent,
    /// Rate limited — retry after delay.
    RateLimited,
}

impl ErrorKind {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Transient => "transient",
            ErrorKind::Timeout => "timeout",
            ErrorKind::Permanent => "permanent",
            ErrorKind::RateLimited => "rate_limited",
        }
    }
}

/// A retry policy that decides whether and when to retry.
pub struct RetryPolicy {
    config: RetryPolicyConfig,
    current_attempt: u32,
    total_retries: u64,
    total_successes: u64,
    total_exhausted: u64,
}

impl RetryPolicy {
    /// Create a new retry policy.
    pub fn new(config: RetryPolicyConfig) -> Self {
        Self {
            config,
            current_attempt: 0,
            total_retries: 0,
            total_successes: 0,
            total_exhausted: 0,
        }
    }

    /// Record a successful operation.
    pub fn record_success(&mut self) {
        self.total_successes += 1;
        self.current_attempt = 0;
    }

    /// Decide whether to retry based on the error kind.
    pub fn should_retry(&mut self, error: ErrorKind) -> RetryDecision {
        // Check if error kind is retryable
        let retryable = match error {
            ErrorKind::Transient => self.config.retry_on_transient,
            ErrorKind::Timeout => self.config.retry_on_timeout,
            ErrorKind::Permanent => false,
            ErrorKind::RateLimited => true,
        };

        if !retryable {
            return RetryDecision::NoRetry;
        }

        if self.current_attempt >= self.config.max_attempts {
            self.total_exhausted += 1;
            self.current_attempt = 0;
            return RetryDecision::Exhausted;
        }

        let delay = self.config.backoff.delay_ms(self.current_attempt);
        self.current_attempt += 1;
        self.total_retries += 1;

        RetryDecision::RetryAfter { delay_ms: delay }
    }

    /// Reset the current attempt counter.
    pub fn reset(&mut self) {
        self.current_attempt = 0;
    }

    /// Current attempt number.
    pub fn current_attempt(&self) -> u32 {
        self.current_attempt
    }

    /// Total retries issued.
    pub fn total_retries(&self) -> u64 {
        self.total_retries
    }

    /// Total successes recorded.
    pub fn total_successes(&self) -> u64 {
        self.total_successes
    }

    /// Total times retry budget was exhausted.
    pub fn total_exhausted(&self) -> u64 {
        self.total_exhausted
    }

    /// Maximum attempts configured.
    pub fn max_attempts(&self) -> u32 {
        self.config.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_transient() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig::default());
        let decision = policy.should_retry(ErrorKind::Transient);
        assert!(decision.should_retry());
        assert!(decision.delay().is_some());
    }

    #[test]
    fn test_no_retry_permanent() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig::default());
        let decision = policy.should_retry(ErrorKind::Permanent);
        assert!(!decision.should_retry());
        assert_eq!(decision, RetryDecision::NoRetry);
    }

    #[test]
    fn test_exhausted_after_max_attempts() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig {
            max_attempts: 2,
            ..Default::default()
        });
        let d1 = policy.should_retry(ErrorKind::Transient);
        assert!(d1.should_retry());
        let d2 = policy.should_retry(ErrorKind::Transient);
        assert!(d2.should_retry());
        let d3 = policy.should_retry(ErrorKind::Transient);
        assert_eq!(d3, RetryDecision::Exhausted);
        assert_eq!(policy.total_exhausted(), 1);
    }

    #[test]
    fn test_success_resets_attempts() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig {
            max_attempts: 2,
            ..Default::default()
        });
        policy.should_retry(ErrorKind::Transient);
        assert_eq!(policy.current_attempt(), 1);
        policy.record_success();
        assert_eq!(policy.current_attempt(), 0);
        assert_eq!(policy.total_successes(), 1);
    }

    #[test]
    fn test_timeout_retry_disabled() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig {
            retry_on_timeout: false,
            ..Default::default()
        });
        let decision = policy.should_retry(ErrorKind::Timeout);
        assert_eq!(decision, RetryDecision::NoRetry);
    }

    #[test]
    fn test_rate_limited_always_retryable() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig {
            retry_on_transient: false,
            retry_on_timeout: false,
            ..Default::default()
        });
        let decision = policy.should_retry(ErrorKind::RateLimited);
        assert!(decision.should_retry());
    }

    #[test]
    fn test_increasing_delays() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig {
            max_attempts: 5,
            backoff: BackoffStrategy::exponential(100, 2.0, 10_000),
            ..Default::default()
        });
        let d1 = policy.should_retry(ErrorKind::Transient).delay().unwrap();
        let d2 = policy.should_retry(ErrorKind::Transient).delay().unwrap();
        let d3 = policy.should_retry(ErrorKind::Transient).delay().unwrap();
        assert_eq!(d1, 100);
        assert_eq!(d2, 200);
        assert_eq!(d3, 400);
    }

    #[test]
    fn test_error_kind_display() {
        assert_eq!(ErrorKind::Transient.as_str(), "transient");
        assert_eq!(ErrorKind::Permanent.as_str(), "permanent");
        assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
    }

    #[test]
    fn test_policy_stats() {
        let mut policy = RetryPolicy::new(RetryPolicyConfig::default());
        policy.should_retry(ErrorKind::Transient);
        policy.record_success();
        assert_eq!(policy.total_retries(), 1);
        assert_eq!(policy.total_successes(), 1);
    }
}
