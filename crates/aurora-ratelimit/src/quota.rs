//! Quota management — daily/hourly/monthly usage quotas with tracking and enforcement.

use std::collections::HashMap;

/// Quota period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuotaPeriod {
    /// Hourly quota.
    Hourly,
    /// Daily quota.
    Daily,
    /// Monthly quota.
    Monthly,
}

impl QuotaPeriod {
    /// Duration of the period in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        match self {
            Self::Hourly => 3_600_000,
            Self::Daily => 86_400_000,
            Self::Monthly => 2_592_000_000, // 30 days
        }
    }
}

/// A quota rule.
#[derive(Debug, Clone)]
pub struct QuotaRule {
    /// Quota period.
    pub period: QuotaPeriod,
    /// Maximum usage in the period.
    pub limit: u64,
    /// Whether to hard-reject or soft-warn when exceeded.
    pub hard_limit: bool,
}

impl QuotaRule {
    /// Create a new hard-limit quota rule.
    pub fn hard(period: QuotaPeriod, limit: u64) -> Self {
        Self {
            period,
            limit,
            hard_limit: true,
        }
    }

    /// Create a new soft-limit (warning) quota rule.
    pub fn soft(period: QuotaPeriod, limit: u64) -> Self {
        Self {
            period,
            limit,
            hard_limit: false,
        }
    }
}

/// Result of a quota check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotaResult {
    /// Within quota.
    Allowed,
    /// Over soft limit (warning).
    Warning,
    /// Over hard limit (rejected).
    Exceeded,
}

/// Usage tracker for a single key+period.
#[derive(Debug, Clone)]
struct UsageCounter {
    count: u64,
    period_start_ms: u64,
    period_duration_ms: u64,
}

impl UsageCounter {
    fn new(period_duration_ms: u64, now_ms: u64) -> Self {
        Self {
            count: 0,
            period_start_ms: now_ms,
            period_duration_ms,
        }
    }

    fn maybe_reset(&mut self, now_ms: u64) {
        if now_ms >= self.period_start_ms + self.period_duration_ms {
            self.count = 0;
            self.period_start_ms = now_ms;
        }
    }

    fn increment(&mut self, now_ms: u64) -> u64 {
        self.maybe_reset(now_ms);
        self.count += 1;
        self.count
    }

    fn current(&mut self, now_ms: u64) -> u64 {
        self.maybe_reset(now_ms);
        self.count
    }
}

/// Quota manager — tracks usage across keys and enforces quota rules.
pub struct QuotaManager {
    rules: HashMap<String, Vec<QuotaRule>>,
    usage: HashMap<(String, QuotaPeriod), UsageCounter>,
}

impl QuotaManager {
    /// Create a new quota manager.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            usage: HashMap::new(),
        }
    }

    /// Add a quota rule for a key pattern.
    pub fn add_rule(&mut self, key: &str, rule: QuotaRule) {
        self.rules.entry(key.to_string()).or_default().push(rule);
    }

    /// Check and consume quota for a key. Returns the strictest result.
    /// Only increments counters if no hard limit is exceeded.
    pub fn check_and_consume(&mut self, key: &str, now_ms: u64) -> QuotaResult {
        let rules = match self.rules.get(key) {
            Some(r) => r.clone(),
            None => return QuotaResult::Allowed,
        };

        // Phase 1: Check all rules WITHOUT incrementing
        let mut result = QuotaResult::Allowed;
        for rule in &rules {
            let counter = self
                .usage
                .entry((key.to_string(), rule.period))
                .or_insert_with(|| UsageCounter::new(rule.period.duration_ms(), now_ms));
            let count = counter.current(now_ms);
            // After increment, count would be count+1
            if count + 1 > rule.limit {
                if rule.hard_limit {
                    return QuotaResult::Exceeded;
                }
                result = QuotaResult::Warning;
            }
        }

        // Phase 2: All hard limits passed — now increment all counters
        for rule in &rules {
            if let Some(counter) = self.usage.get_mut(&(key.to_string(), rule.period)) {
                counter.increment(now_ms);
            }
        }
        result
    }

    /// Check quota without consuming. Returns the strictest result.
    pub fn check(&mut self, key: &str, now_ms: u64) -> QuotaResult {
        let rules = match self.rules.get(key) {
            Some(r) => r.clone(),
            None => return QuotaResult::Allowed,
        };

        let mut result = QuotaResult::Allowed;
        for rule in &rules {
            let counter = self
                .usage
                .entry((key.to_string(), rule.period))
                .or_insert_with(|| UsageCounter::new(rule.period.duration_ms(), now_ms));
            let count = counter.current(now_ms);
            if count >= rule.limit {
                if rule.hard_limit {
                    return QuotaResult::Exceeded;
                }
                result = QuotaResult::Warning;
            }
        }
        result
    }

    /// Get current usage for a key and period.
    pub fn usage(&mut self, key: &str, period: QuotaPeriod, now_ms: u64) -> u64 {
        self.usage
            .get_mut(&(key.to_string(), period))
            .map_or(0, |c| c.current(now_ms))
    }

    /// Get remaining quota for a key and period.
    pub fn remaining(&mut self, key: &str, period: QuotaPeriod, now_ms: u64) -> Option<u64> {
        let limit = self
            .rules
            .get(key)?
            .iter()
            .find(|r| r.period == period)?
            .limit;
        let used = self.usage(key, period, now_ms);
        Some(limit.saturating_sub(used))
    }

    /// Reset usage for a key.
    pub fn reset_usage(&mut self, key: &str) {
        self.usage.retain(|(k, _), _| k != key);
    }

    /// Get total number of tracked keys.
    pub fn tracked_key_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_within_limit() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("api_key_1", QuotaRule::hard(QuotaPeriod::Hourly, 100));

        for _ in 0..100 {
            assert_eq!(mgr.check_and_consume("api_key_1", 0), QuotaResult::Allowed);
        }
    }

    #[test]
    fn test_quota_exceeded() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("api_key_1", QuotaRule::hard(QuotaPeriod::Hourly, 5));

        for _ in 0..5 {
            mgr.check_and_consume("api_key_1", 0);
        }
        assert_eq!(mgr.check_and_consume("api_key_1", 0), QuotaResult::Exceeded);
    }

    #[test]
    fn test_quota_soft_warning() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("api_key_1", QuotaRule::soft(QuotaPeriod::Daily, 3));

        for _ in 0..3 {
            mgr.check_and_consume("api_key_1", 0);
        }
        assert_eq!(mgr.check_and_consume("api_key_1", 0), QuotaResult::Warning);
    }

    #[test]
    fn test_quota_period_reset() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Hourly, 2));

        mgr.check_and_consume("k", 0);
        mgr.check_and_consume("k", 0);
        assert_eq!(mgr.check_and_consume("k", 0), QuotaResult::Exceeded);

        // After 1 hour, should reset
        assert_eq!(mgr.check_and_consume("k", 3_700_000), QuotaResult::Allowed);
    }

    #[test]
    fn test_quota_remaining() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Daily, 10));

        mgr.check_and_consume("k", 0);
        mgr.check_and_consume("k", 0);
        mgr.check_and_consume("k", 0);

        assert_eq!(mgr.remaining("k", QuotaPeriod::Daily, 0), Some(7));
    }

    #[test]
    fn test_quota_no_rule() {
        let mut mgr = QuotaManager::new();
        assert_eq!(mgr.check_and_consume("unknown", 0), QuotaResult::Allowed);
    }

    #[test]
    fn test_quota_multiple_periods() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Hourly, 10));
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Daily, 100));

        for _ in 0..10 {
            mgr.check_and_consume("k", 0);
        }
        // Hourly limit hit
        assert_eq!(mgr.check_and_consume("k", 0), QuotaResult::Exceeded);
    }

    #[test]
    fn test_quota_reset_usage() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Hourly, 2));

        mgr.check_and_consume("k", 0);
        mgr.check_and_consume("k", 0);
        assert_eq!(mgr.check_and_consume("k", 0), QuotaResult::Exceeded);

        mgr.reset_usage("k");
        assert_eq!(mgr.check_and_consume("k", 0), QuotaResult::Allowed);
    }

    #[test]
    fn test_quota_check_without_consume() {
        let mut mgr = QuotaManager::new();
        mgr.add_rule("k", QuotaRule::hard(QuotaPeriod::Hourly, 5));

        for _ in 0..5 {
            mgr.check_and_consume("k", 0);
        }
        // check should see it's at limit
        assert_eq!(mgr.check("k", 0), QuotaResult::Exceeded);
    }
}
