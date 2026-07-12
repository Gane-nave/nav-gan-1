//! Backoff strategies — exponential, linear, constant, and with jitter.

/// A backoff strategy for computing retry delays.
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// Constant delay between retries.
    Constant { delay_ms: u64 },
    /// Linearly increasing delay.
    Linear {
        initial_ms: u64,
        increment_ms: u64,
        max_ms: u64,
    },
    /// Exponentially increasing delay.
    Exponential {
        initial_ms: u64,
        multiplier: f64,
        max_ms: u64,
    },
}

impl BackoffStrategy {
    /// Create a constant backoff.
    pub fn constant(delay_ms: u64) -> Self {
        BackoffStrategy::Constant { delay_ms }
    }

    /// Create a linear backoff.
    pub fn linear(initial_ms: u64, increment_ms: u64, max_ms: u64) -> Self {
        BackoffStrategy::Linear {
            initial_ms,
            increment_ms,
            max_ms,
        }
    }

    /// Create an exponential backoff.
    pub fn exponential(initial_ms: u64, multiplier: f64, max_ms: u64) -> Self {
        BackoffStrategy::Exponential {
            initial_ms,
            multiplier,
            max_ms,
        }
    }

    /// Compute the delay for the nth attempt (0-indexed).
    pub fn delay_ms(&self, attempt: u32) -> u64 {
        match self {
            BackoffStrategy::Constant { delay_ms } => *delay_ms,
            BackoffStrategy::Linear {
                initial_ms,
                increment_ms,
                max_ms,
            } => {
                let delay = initial_ms.saturating_add(increment_ms.saturating_mul(attempt as u64));
                delay.min(*max_ms)
            }
            BackoffStrategy::Exponential {
                initial_ms,
                multiplier,
                max_ms,
            } => {
                let delay = (*initial_ms as f64) * multiplier.powi(attempt as i32);
                (delay as u64).min(*max_ms)
            }
        }
    }
}

/// Adds deterministic jitter to a backoff delay.
pub fn add_jitter(delay_ms: u64, jitter_factor: f64, seed: u64) -> u64 {
    if jitter_factor <= 0.0 {
        return delay_ms;
    }
    let jitter_factor = jitter_factor.min(1.0);
    // Simple deterministic jitter using seed
    let jitter_range = (delay_ms as f64 * jitter_factor) as u64;
    if jitter_range == 0 {
        return delay_ms;
    }
    let jitter = seed % (jitter_range + 1);
    delay_ms
        .saturating_sub(jitter_range / 2)
        .saturating_add(jitter)
}

/// Iterator that yields backoff delays for successive attempts.
pub struct BackoffIter {
    strategy: BackoffStrategy,
    attempt: u32,
    max_attempts: u32,
}

impl BackoffIter {
    /// Create a new backoff iterator.
    pub fn new(strategy: BackoffStrategy, max_attempts: u32) -> Self {
        Self {
            strategy,
            attempt: 0,
            max_attempts,
        }
    }

    /// Remaining attempts.
    pub fn remaining(&self) -> u32 {
        self.max_attempts.saturating_sub(self.attempt)
    }
}

impl Iterator for BackoffIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.attempt >= self.max_attempts {
            return None;
        }
        let delay = self.strategy.delay_ms(self.attempt);
        self.attempt += 1;
        Some(delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_backoff() {
        let strategy = BackoffStrategy::constant(100);
        assert_eq!(strategy.delay_ms(0), 100);
        assert_eq!(strategy.delay_ms(5), 100);
        assert_eq!(strategy.delay_ms(100), 100);
    }

    #[test]
    fn test_linear_backoff() {
        let strategy = BackoffStrategy::linear(100, 50, 500);
        assert_eq!(strategy.delay_ms(0), 100);
        assert_eq!(strategy.delay_ms(1), 150);
        assert_eq!(strategy.delay_ms(2), 200);
        assert_eq!(strategy.delay_ms(10), 500); // capped at max
    }

    #[test]
    fn test_exponential_backoff() {
        let strategy = BackoffStrategy::exponential(100, 2.0, 10000);
        assert_eq!(strategy.delay_ms(0), 100);
        assert_eq!(strategy.delay_ms(1), 200);
        assert_eq!(strategy.delay_ms(2), 400);
        assert_eq!(strategy.delay_ms(3), 800);
        assert_eq!(strategy.delay_ms(10), 10000); // capped at max
    }

    #[test]
    fn test_exponential_capped() {
        let strategy = BackoffStrategy::exponential(1000, 3.0, 5000);
        assert_eq!(strategy.delay_ms(0), 1000);
        assert_eq!(strategy.delay_ms(1), 3000);
        assert_eq!(strategy.delay_ms(2), 5000); // 9000 capped to 5000
    }

    #[test]
    fn test_jitter() {
        let delay = add_jitter(1000, 0.5, 42);
        assert!(
            (750..=1250).contains(&delay),
            "jitter out of range: {delay}"
        );
    }

    #[test]
    fn test_jitter_zero_factor() {
        let delay = add_jitter(1000, 0.0, 42);
        assert_eq!(delay, 1000);
    }

    #[test]
    fn test_backoff_iter() {
        let iter = BackoffIter::new(BackoffStrategy::constant(100), 3);
        let delays: Vec<u64> = iter.collect();
        assert_eq!(delays, vec![100, 100, 100]);
    }

    #[test]
    fn test_backoff_iter_remaining() {
        let mut iter = BackoffIter::new(BackoffStrategy::constant(100), 5);
        assert_eq!(iter.remaining(), 5);
        iter.next();
        assert_eq!(iter.remaining(), 4);
    }

    #[test]
    fn test_linear_overflow_protection() {
        let strategy = BackoffStrategy::linear(u64::MAX - 10, 100, u64::MAX);
        let delay = strategy.delay_ms(1);
        assert_eq!(delay, u64::MAX); // saturating_add caps at MAX
    }
}
