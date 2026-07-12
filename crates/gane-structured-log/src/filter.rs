//! Log filtering — match log entries against patterns and rules.

use crate::entry::{LogEntry, LogLevel};

/// A filter rule for log entries.
#[derive(Debug, Clone)]
pub enum FilterRule {
    /// Match entries at or above a minimum level.
    MinLevel(LogLevel),
    /// Match entries from a specific source (exact match).
    Source(String),
    /// Match entries whose message contains a substring.
    MessageContains(String),
    /// Match entries that have a specific field key.
    HasField(String),
    /// Logical AND of multiple rules.
    All(Vec<FilterRule>),
    /// Logical OR of multiple rules.
    Any(Vec<FilterRule>),
    /// Logical NOT of a rule.
    Not(Box<FilterRule>),
}

impl FilterRule {
    /// Check if an entry matches this rule.
    pub fn matches(&self, entry: &LogEntry) -> bool {
        match self {
            FilterRule::MinLevel(level) => entry.level >= *level,
            FilterRule::Source(src) => entry.source == *src,
            FilterRule::MessageContains(sub) => entry.message.contains(sub.as_str()),
            FilterRule::HasField(key) => entry.fields.contains_key(key),
            FilterRule::All(rules) => rules.iter().all(|r| r.matches(entry)),
            FilterRule::Any(rules) => rules.iter().any(|r| r.matches(entry)),
            FilterRule::Not(rule) => !rule.matches(entry),
        }
    }
}

/// A filter chain — evaluates a list of rules and determines accept/reject.
pub struct FilterChain {
    /// Rules to accept. If empty, all entries are accepted by default.
    accept_rules: Vec<FilterRule>,
    /// Rules to reject (applied after accept).
    reject_rules: Vec<FilterRule>,
}

impl FilterChain {
    /// Create a new filter chain.
    pub fn new() -> Self {
        Self {
            accept_rules: Vec::new(),
            reject_rules: Vec::new(),
        }
    }

    /// Add an accept rule.
    pub fn accept(mut self, rule: FilterRule) -> Self {
        self.accept_rules.push(rule);
        self
    }

    /// Add a reject rule.
    pub fn reject(mut self, rule: FilterRule) -> Self {
        self.reject_rules.push(rule);
        self
    }

    /// Check if an entry passes the filter chain.
    pub fn allows(&self, entry: &LogEntry) -> bool {
        // Check reject rules first
        for rule in &self.reject_rules {
            if rule.matches(entry) {
                return false;
            }
        }

        // If no accept rules, accept everything
        if self.accept_rules.is_empty() {
            return true;
        }

        // At least one accept rule must match
        self.accept_rules.iter().any(|r| r.matches(entry))
    }

    /// Filter a slice of entries.
    pub fn filter<'a>(&self, entries: &'a [LogEntry]) -> Vec<&'a LogEntry> {
        entries.iter().filter(|e| self.allows(e)).collect()
    }
}

impl Default for FilterChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiter for log entries — suppresses duplicates within a time window.
pub struct LogRateLimiter {
    /// Key → last emission timestamp.
    last_emitted: std::collections::HashMap<String, u64>,
    /// Minimum interval in milliseconds between emissions of the same key.
    interval_ms: u64,
}

impl LogRateLimiter {
    /// Create a rate limiter with the given interval.
    pub fn new(interval_ms: u64) -> Self {
        Self {
            last_emitted: std::collections::HashMap::new(),
            interval_ms,
        }
    }

    /// Check if a log entry with the given key should be emitted.
    /// The key is typically the message or a hash of the entry.
    pub fn should_emit(&mut self, key: &str, timestamp_ms: u64) -> bool {
        if let Some(&last) = self.last_emitted.get(key) {
            if timestamp_ms < last + self.interval_ms {
                return false;
            }
        }
        self.last_emitted.insert(key.to_string(), timestamp_ms);
        true
    }

    /// Number of tracked keys.
    pub fn tracked_keys(&self) -> usize {
        self.last_emitted.len()
    }

    /// Clear all tracked keys.
    pub fn clear(&mut self) {
        self.last_emitted.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::LogEntry;

    #[test]
    fn test_min_level_filter() {
        let rule = FilterRule::MinLevel(LogLevel::Warn);
        let info = LogEntry::new(LogLevel::Info, "info msg");
        let warn = LogEntry::new(LogLevel::Warn, "warn msg");
        let error = LogEntry::new(LogLevel::Error, "error msg");

        assert!(!rule.matches(&info));
        assert!(rule.matches(&warn));
        assert!(rule.matches(&error));
    }

    #[test]
    fn test_source_filter() {
        let rule = FilterRule::Source("db".into());
        let db_entry = LogEntry::new(LogLevel::Info, "query").with_source("db");
        let api_entry = LogEntry::new(LogLevel::Info, "request").with_source("api");

        assert!(rule.matches(&db_entry));
        assert!(!rule.matches(&api_entry));
    }

    #[test]
    fn test_message_contains() {
        let rule = FilterRule::MessageContains("error".into());
        let e1 = LogEntry::new(LogLevel::Info, "an error occurred");
        let e2 = LogEntry::new(LogLevel::Info, "all good");

        assert!(rule.matches(&e1));
        assert!(!rule.matches(&e2));
    }

    #[test]
    fn test_has_field() {
        let rule = FilterRule::HasField("user_id".into());
        let e1 = LogEntry::new(LogLevel::Info, "login").with_int("user_id", 42);
        let e2 = LogEntry::new(LogLevel::Info, "startup");

        assert!(rule.matches(&e1));
        assert!(!rule.matches(&e2));
    }

    #[test]
    fn test_composite_rules() {
        let rule = FilterRule::All(vec![
            FilterRule::MinLevel(LogLevel::Warn),
            FilterRule::Source("api".into()),
        ]);

        let e1 = LogEntry::new(LogLevel::Error, "fail").with_source("api");
        let e2 = LogEntry::new(LogLevel::Info, "ok").with_source("api");
        let e3 = LogEntry::new(LogLevel::Error, "fail").with_source("db");

        assert!(rule.matches(&e1));
        assert!(!rule.matches(&e2)); // level too low
        assert!(!rule.matches(&e3)); // wrong source
    }

    #[test]
    fn test_not_rule() {
        let rule = FilterRule::Not(Box::new(FilterRule::Source("noisy".into())));
        let e1 = LogEntry::new(LogLevel::Info, "msg").with_source("noisy");
        let e2 = LogEntry::new(LogLevel::Info, "msg").with_source("api");

        assert!(!rule.matches(&e1));
        assert!(rule.matches(&e2));
    }

    #[test]
    fn test_filter_chain() {
        let chain = FilterChain::new()
            .accept(FilterRule::MinLevel(LogLevel::Warn))
            .reject(FilterRule::Source("noisy".into()));

        let e1 = LogEntry::new(LogLevel::Error, "fail").with_source("api");
        let e2 = LogEntry::new(LogLevel::Info, "ok").with_source("api");
        let e3 = LogEntry::new(LogLevel::Error, "fail").with_source("noisy");

        assert!(chain.allows(&e1)); // warn+ and not noisy
        assert!(!chain.allows(&e2)); // below warn
        assert!(!chain.allows(&e3)); // rejected
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = LogRateLimiter::new(1000);
        assert!(limiter.should_emit("key1", 100));
        assert!(!limiter.should_emit("key1", 500)); // too soon
        assert!(limiter.should_emit("key1", 1200)); // enough time passed
        assert!(limiter.should_emit("key2", 100)); // different key
        assert_eq!(limiter.tracked_keys(), 2);
    }

    #[test]
    fn test_filter_chain_no_accept_rules() {
        let chain = FilterChain::new().reject(FilterRule::Source("blocked".into()));

        let e1 = LogEntry::new(LogLevel::Debug, "anything").with_source("ok");
        let e2 = LogEntry::new(LogLevel::Debug, "anything").with_source("blocked");

        assert!(chain.allows(&e1)); // no accept rules = accept all
        assert!(!chain.allows(&e2)); // explicitly rejected
    }
}
