//! Retry policies — configurable retry strategies with backoff.

/// Backoff strategy for retries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackoffStrategy {
    /// Fixed delay between retries.
    Fixed,
    /// Linearly increasing delay.
    Linear,
    /// Exponentially increasing delay.
    Exponential,
}

/// Retry policy configuration.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Base delay in milliseconds.
    pub base_delay_ms: u64,
    /// Maximum delay in milliseconds (cap).
    pub max_delay_ms: u64,
    /// Backoff strategy.
    pub strategy: BackoffStrategy,
    /// Multiplier for exponential backoff.
    pub multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
            strategy: BackoffStrategy::Exponential,
            multiplier: 2.0,
        }
    }
}

impl RetryPolicy {
    /// Create a policy with no retries.
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    /// Create a policy with fixed delay.
    pub fn fixed(max_retries: u32, delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms: delay_ms,
            max_delay_ms: delay_ms,
            strategy: BackoffStrategy::Fixed,
            multiplier: 1.0,
        }
    }

    /// Create a policy with exponential backoff.
    pub fn exponential(max_retries: u32, base_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms,
            max_delay_ms,
            strategy: BackoffStrategy::Exponential,
            multiplier: 2.0,
        }
    }

    /// Calculate the delay for a given attempt number (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        let delay = match self.strategy {
            BackoffStrategy::Fixed => self.base_delay_ms,
            BackoffStrategy::Linear => self.base_delay_ms + (self.base_delay_ms * attempt as u64),
            BackoffStrategy::Exponential => {
                let factor = self.multiplier.powi(attempt as i32);
                (self.base_delay_ms as f64 * factor) as u64
            }
        };
        delay.min(self.max_delay_ms)
    }

    /// Whether a retry should be attempted for the given attempt number.
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_retries
    }
}

/// Tracks the state of a retry sequence.
#[derive(Debug)]
pub struct RetryState {
    policy: RetryPolicy,
    attempt: u32,
    total_delay_ms: u64,
    last_error: Option<String>,
}

impl RetryState {
    /// Create a new retry state with the given policy.
    pub fn new(policy: RetryPolicy) -> Self {
        Self {
            policy,
            attempt: 0,
            total_delay_ms: 0,
            last_error: None,
        }
    }

    /// Record a failure and determine if retry should be attempted.
    /// Returns Some(delay_ms) if retry should happen, None if exhausted.
    pub fn record_failure(&mut self, error: &str) -> Option<u64> {
        self.last_error = Some(error.to_string());
        if self.policy.should_retry(self.attempt) {
            let delay = self.policy.delay_for_attempt(self.attempt);
            self.attempt += 1;
            self.total_delay_ms += delay;
            Some(delay)
        } else {
            None
        }
    }

    /// Record a success, resetting the state.
    pub fn record_success(&mut self) {
        self.attempt = 0;
        self.total_delay_ms = 0;
        self.last_error = None;
    }

    /// Current attempt number.
    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Whether retries are exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.attempt >= self.policy.max_retries
    }

    /// Total delay accumulated across all retries.
    pub fn total_delay_ms(&self) -> u64 {
        self.total_delay_ms
    }

    /// Last recorded error message.
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}

/// Retry budget — limits total retries across a time window.
pub struct RetryBudget {
    /// Maximum retries allowed in the window.
    max_retries: u32,
    /// Window duration in milliseconds.
    window_ms: u64,
    /// Timestamps of recent retries.
    retry_times: Vec<u64>,
}

impl RetryBudget {
    /// Create a new retry budget.
    pub fn new(max_retries: u32, window_ms: u64) -> Self {
        Self {
            max_retries,
            window_ms,
            retry_times: Vec::new(),
        }
    }

    /// Check if a retry is allowed at the given timestamp.
    pub fn allow_retry(&mut self, now_ms: u64) -> bool {
        // Evict expired entries
        let cutoff = now_ms.saturating_sub(self.window_ms);
        self.retry_times.retain(|&t| t > cutoff);

        if (self.retry_times.len() as u32) < self.max_retries {
            self.retry_times.push(now_ms);
            true
        } else {
            false
        }
    }

    /// Number of retries used in the current window.
    pub fn used(&self) -> u32 {
        self.retry_times.len() as u32
    }

    /// Remaining retries in the current window.
    pub fn remaining(&self) -> u32 {
        self.max_retries
            .saturating_sub(self.retry_times.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_delay() {
        let policy = RetryPolicy::fixed(3, 500);
        assert_eq!(policy.delay_for_attempt(0), 500);
        assert_eq!(policy.delay_for_attempt(1), 500);
        assert_eq!(policy.delay_for_attempt(2), 500);
    }

    #[test]
    fn test_linear_delay() {
        let policy = RetryPolicy {
            strategy: BackoffStrategy::Linear,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
            ..Default::default()
        };
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 300);
    }

    #[test]
    fn test_exponential_delay() {
        let policy = RetryPolicy::exponential(5, 100, 5000);
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 400);
        assert_eq!(policy.delay_for_attempt(3), 800);
    }

    #[test]
    fn test_delay_capped_at_max() {
        let policy = RetryPolicy::exponential(10, 100, 500);
        assert_eq!(policy.delay_for_attempt(5), 500); // would be 3200, capped
    }

    #[test]
    fn test_should_retry() {
        let policy = RetryPolicy::fixed(3, 100);
        assert!(policy.should_retry(0));
        assert!(policy.should_retry(2));
        assert!(!policy.should_retry(3));
    }

    #[test]
    fn test_no_retry_policy() {
        let policy = RetryPolicy::no_retry();
        assert!(!policy.should_retry(0));
    }

    #[test]
    fn test_retry_state_success_path() {
        let mut state = RetryState::new(RetryPolicy::fixed(3, 100));
        let delay = state.record_failure("err1");
        assert_eq!(delay, Some(100));
        assert_eq!(state.attempt(), 1);

        state.record_success();
        assert_eq!(state.attempt(), 0);
        assert!(!state.is_exhausted());
    }

    #[test]
    fn test_retry_state_exhaustion() {
        let mut state = RetryState::new(RetryPolicy::fixed(2, 100));
        assert!(state.record_failure("err1").is_some());
        assert!(state.record_failure("err2").is_some());
        assert!(state.record_failure("err3").is_none());
        assert!(state.is_exhausted());
        assert_eq!(state.last_error(), Some("err3"));
    }

    #[test]
    fn test_retry_state_total_delay() {
        let mut state = RetryState::new(RetryPolicy::exponential(5, 100, 10_000));
        state.record_failure("e1"); // 100
        state.record_failure("e2"); // 200
        state.record_failure("e3"); // 400
        assert_eq!(state.total_delay_ms(), 700);
    }

    #[test]
    fn test_retry_budget_allows_within_limit() {
        let mut budget = RetryBudget::new(3, 10_000);
        assert!(budget.allow_retry(1000));
        assert!(budget.allow_retry(2000));
        assert!(budget.allow_retry(3000));
        assert!(!budget.allow_retry(4000)); // budget exhausted
        assert_eq!(budget.remaining(), 0);
    }

    #[test]
    fn test_retry_budget_window_expiry() {
        let mut budget = RetryBudget::new(2, 5000);
        assert!(budget.allow_retry(1000));
        assert!(budget.allow_retry(2000));
        assert!(!budget.allow_retry(3000)); // full

        // After window expires (cutoff = 7000 - 5000 = 2000, entries at 1000 and 2000 evicted)
        assert!(budget.allow_retry(7000)); // both 1000 and 2000 expired
        assert_eq!(budget.used(), 1); // only 7000 remains
    }
}
