//! Retry budget — controls retry storms by limiting retry rate.

/// A retry budget that limits the number of retries within a time window.
pub struct RetryBudget {
    /// Maximum retries allowed in the window.
    max_retries: u64,
    /// Window duration in milliseconds.
    window_ms: u64,
    /// Retry timestamps within the current window.
    retry_times: Vec<u64>,
    /// Total retries attempted (lifetime).
    total_retries: u64,
    /// Total retries rejected (over budget).
    total_rejected: u64,
}

impl RetryBudget {
    /// Create a new retry budget.
    pub fn new(max_retries: u64, window_ms: u64) -> Self {
        Self {
            max_retries,
            window_ms,
            retry_times: Vec::new(),
            total_retries: 0,
            total_rejected: 0,
        }
    }

    /// Try to use a retry from the budget at the given timestamp.
    /// Returns true if the retry is allowed, false if over budget.
    pub fn try_retry(&mut self, now_ms: u64) -> bool {
        self.expire(now_ms);

        if self.retry_times.len() as u64 >= self.max_retries {
            self.total_rejected += 1;
            return false;
        }

        self.retry_times.push(now_ms);
        self.total_retries += 1;
        true
    }

    /// Current number of retries used in the window.
    pub fn used(&self) -> u64 {
        self.retry_times.len() as u64
    }

    /// Remaining retries available.
    pub fn remaining(&self, now_ms: u64) -> u64 {
        let expired_count = self
            .retry_times
            .iter()
            .filter(|&&t| now_ms.saturating_sub(t) >= self.window_ms)
            .count() as u64;
        let active = self.retry_times.len() as u64 - expired_count;
        self.max_retries.saturating_sub(active)
    }

    /// Utilization ratio (0.0 = no retries used, 1.0 = fully consumed).
    pub fn utilization(&self) -> f64 {
        if self.max_retries == 0 {
            return 1.0;
        }
        self.retry_times.len() as f64 / self.max_retries as f64
    }

    /// Total retries attempted (lifetime).
    pub fn total_retries(&self) -> u64 {
        self.total_retries
    }

    /// Total retries rejected (lifetime).
    pub fn total_rejected(&self) -> u64 {
        self.total_rejected
    }

    /// Reset the budget.
    pub fn reset(&mut self) {
        self.retry_times.clear();
    }

    /// Remove expired retry timestamps.
    fn expire(&mut self, now_ms: u64) {
        self.retry_times
            .retain(|&t| now_ms.saturating_sub(t) < self.window_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_allows_retries() {
        let mut budget = RetryBudget::new(3, 10_000);
        assert!(budget.try_retry(1000));
        assert!(budget.try_retry(2000));
        assert!(budget.try_retry(3000));
        assert_eq!(budget.used(), 3);
    }

    #[test]
    fn test_budget_rejects_over_limit() {
        let mut budget = RetryBudget::new(2, 10_000);
        assert!(budget.try_retry(1000));
        assert!(budget.try_retry(2000));
        assert!(!budget.try_retry(3000)); // over budget
        assert_eq!(budget.total_rejected(), 1);
    }

    #[test]
    fn test_budget_expires_old_retries() {
        let mut budget = RetryBudget::new(2, 5000);
        assert!(budget.try_retry(1000));
        assert!(budget.try_retry(2000));
        assert!(!budget.try_retry(3000)); // full

        // At t=7000, retry at t=1000 should be expired (7000-1000=6000 >= 5000)
        assert!(budget.try_retry(7000));
    }

    #[test]
    fn test_budget_remaining() {
        let mut budget = RetryBudget::new(5, 10_000);
        budget.try_retry(1000);
        budget.try_retry(2000);
        assert_eq!(budget.remaining(3000), 3);
    }

    #[test]
    fn test_budget_utilization() {
        let mut budget = RetryBudget::new(4, 10_000);
        assert!((budget.utilization() - 0.0).abs() < f64::EPSILON);
        budget.try_retry(1000);
        budget.try_retry(2000);
        assert!((budget.utilization() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_budget_zero_max() {
        let mut budget = RetryBudget::new(0, 10_000);
        assert!(!budget.try_retry(1000));
        assert!((budget.utilization() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_budget_reset() {
        let mut budget = RetryBudget::new(3, 10_000);
        budget.try_retry(1000);
        budget.try_retry(2000);
        budget.reset();
        assert_eq!(budget.used(), 0);
        assert!(budget.try_retry(3000));
    }

    #[test]
    fn test_budget_stats() {
        let mut budget = RetryBudget::new(2, 10_000);
        budget.try_retry(1000);
        budget.try_retry(2000);
        budget.try_retry(3000); // rejected
        assert_eq!(budget.total_retries(), 2);
        assert_eq!(budget.total_rejected(), 1);
    }
}
