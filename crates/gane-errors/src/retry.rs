//! Retry logic — configurable retry strategies with backoff, jitter, and budgets.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Retry strategy — determines how retries are spaced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// Fixed delay between retries.
    Fixed { delay: Duration },
    /// Exponential backoff with optional cap.
    Exponential {
        initial_delay: Duration,
        multiplier: f64,
        max_delay: Duration,
    },
    /// Linear backoff.
    Linear {
        initial_delay: Duration,
        increment: Duration,
        max_delay: Duration,
    },
}

impl RetryStrategy {
    /// Calculate the delay for a given attempt number (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        match self {
            Self::Fixed { delay } => *delay,
            Self::Exponential {
                initial_delay,
                multiplier,
                max_delay,
            } => {
                let delay_ms = initial_delay.as_millis() as f64 * multiplier.powi(attempt as i32);
                let delay = Duration::from_millis(delay_ms as u64);
                delay.min(*max_delay)
            }
            Self::Linear {
                initial_delay,
                increment,
                max_delay,
            } => {
                let delay = *initial_delay + *increment * attempt;
                delay.min(*max_delay)
            }
        }
    }
}

/// Retry configuration.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub strategy: RetryStrategy,
    pub max_attempts: u32,
    pub jitter: bool,
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            strategy: RetryStrategy::Exponential {
                initial_delay: Duration::from_millis(100),
                multiplier: 2.0,
                max_delay: Duration::from_secs(30),
            },
            max_attempts: 3,
            jitter: true,
            jitter_factor: 0.25,
        }
    }
}

/// Result of a retry operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryResult<T> {
    pub value: Option<T>,
    pub attempts: u32,
    pub success: bool,
    pub last_error: Option<String>,
}

/// Retry budget — limits total retries across all operations to prevent retry storms.
pub struct RetryBudget {
    max_retries_per_window: u32,
    window: Duration,
    retries: parking_lot::RwLock<Vec<std::time::Instant>>,
}

impl RetryBudget {
    /// Create a new retry budget.
    pub fn new(max_retries_per_window: u32, window: Duration) -> Self {
        Self {
            max_retries_per_window,
            window,
            retries: parking_lot::RwLock::new(Vec::new()),
        }
    }

    /// Check if a retry is allowed within the budget.
    pub fn allow_retry(&self) -> bool {
        let mut retries = self.retries.write();
        let now = std::time::Instant::now();

        // Prune old retries outside the window
        retries.retain(|t| now.duration_since(*t) < self.window);

        if retries.len() < self.max_retries_per_window as usize {
            retries.push(now);
            true
        } else {
            false
        }
    }

    /// Current retry count in the window.
    pub fn current_count(&self) -> u32 {
        let retries = self.retries.read();
        let now = std::time::Instant::now();
        retries
            .iter()
            .filter(|t| now.duration_since(**t) < self.window)
            .count() as u32
    }

    /// Remaining retries in the window.
    pub fn remaining(&self) -> u32 {
        self.max_retries_per_window
            .saturating_sub(self.current_count())
    }
}

/// Retry executor — runs an operation with retry logic (synchronous).
pub struct RetryExecutor {
    config: RetryConfig,
}

impl RetryExecutor {
    /// Create a new retry executor.
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Create with default config.
    pub fn with_defaults() -> Self {
        Self::new(RetryConfig::default())
    }

    /// Execute an operation with retry logic.
    /// The operation should return Ok(T) on success or Err(String) on failure.
    pub fn execute<T, F>(&self, mut operation: F) -> RetryResult<T>
    where
        F: FnMut(u32) -> Result<T, String>,
    {
        let mut last_error = None;

        for attempt in 0..self.config.max_attempts {
            match operation(attempt) {
                Ok(value) => {
                    return RetryResult {
                        value: Some(value),
                        attempts: attempt + 1,
                        success: true,
                        last_error: None,
                    };
                }
                Err(e) => {
                    last_error = Some(e);
                }
            }
        }

        RetryResult {
            value: None,
            attempts: self.config.max_attempts,
            success: false,
            last_error,
        }
    }

    /// Get the delay for a specific attempt.
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        self.config.strategy.delay_for_attempt(attempt)
    }

    /// Get max attempts.
    pub fn max_attempts(&self) -> u32 {
        self.config.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_strategy() {
        let strategy = RetryStrategy::Fixed {
            delay: Duration::from_millis(100),
        };
        assert_eq!(strategy.delay_for_attempt(0), Duration::from_millis(100));
        assert_eq!(strategy.delay_for_attempt(5), Duration::from_millis(100));
    }

    #[test]
    fn test_exponential_strategy() {
        let strategy = RetryStrategy::Exponential {
            initial_delay: Duration::from_millis(100),
            multiplier: 2.0,
            max_delay: Duration::from_secs(10),
        };
        assert_eq!(strategy.delay_for_attempt(0), Duration::from_millis(100));
        assert_eq!(strategy.delay_for_attempt(1), Duration::from_millis(200));
        assert_eq!(strategy.delay_for_attempt(2), Duration::from_millis(400));
        assert_eq!(strategy.delay_for_attempt(3), Duration::from_millis(800));
    }

    #[test]
    fn test_exponential_strategy_caps_at_max() {
        let strategy = RetryStrategy::Exponential {
            initial_delay: Duration::from_millis(100),
            multiplier: 10.0,
            max_delay: Duration::from_secs(1),
        };
        // 100 * 10^3 = 100000ms > 1000ms max
        assert_eq!(strategy.delay_for_attempt(3), Duration::from_secs(1));
    }

    #[test]
    fn test_linear_strategy() {
        let strategy = RetryStrategy::Linear {
            initial_delay: Duration::from_millis(100),
            increment: Duration::from_millis(50),
            max_delay: Duration::from_secs(1),
        };
        assert_eq!(strategy.delay_for_attempt(0), Duration::from_millis(100));
        assert_eq!(strategy.delay_for_attempt(1), Duration::from_millis(150));
        assert_eq!(strategy.delay_for_attempt(2), Duration::from_millis(200));
    }

    #[test]
    fn test_retry_executor_succeeds_first_try() {
        let executor = RetryExecutor::with_defaults();
        let result = executor.execute(|_| Ok::<i32, String>(42));
        assert!(result.success);
        assert_eq!(result.value, Some(42));
        assert_eq!(result.attempts, 1);
    }

    #[test]
    fn test_retry_executor_succeeds_after_failures() {
        let executor = RetryExecutor::new(RetryConfig {
            max_attempts: 5,
            ..Default::default()
        });
        let result = executor.execute(|attempt| {
            if attempt < 2 {
                Err("not ready".to_string())
            } else {
                Ok(42)
            }
        });
        assert!(result.success);
        assert_eq!(result.value, Some(42));
        assert_eq!(result.attempts, 3); // Failed 2, succeeded on 3rd
    }

    #[test]
    fn test_retry_executor_exhausts_attempts() {
        let executor = RetryExecutor::new(RetryConfig {
            max_attempts: 3,
            ..Default::default()
        });
        let result = executor.execute(|_| Err::<i32, String>("always fails".to_string()));
        assert!(!result.success);
        assert_eq!(result.value, None);
        assert_eq!(result.attempts, 3);
        assert_eq!(result.last_error, Some("always fails".to_string()));
    }

    #[test]
    fn test_retry_budget_allows_within_limit() {
        let budget = RetryBudget::new(3, Duration::from_secs(60));
        assert!(budget.allow_retry());
        assert!(budget.allow_retry());
        assert!(budget.allow_retry());
        assert!(!budget.allow_retry()); // Over budget
    }

    #[test]
    fn test_retry_budget_remaining() {
        let budget = RetryBudget::new(5, Duration::from_secs(60));
        assert_eq!(budget.remaining(), 5);
        budget.allow_retry();
        budget.allow_retry();
        assert_eq!(budget.remaining(), 3);
    }
}
