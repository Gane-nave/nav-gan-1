//! Usage tracking — monitors API consumption per key, per endpoint,
//! and per time window for billing, analytics, and quota enforcement.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

// ---------------------------------------------------------------------------
// Usage record
// ---------------------------------------------------------------------------

/// A single API usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: EntityId,
    pub key_id: EntityId,
    pub endpoint: String,
    pub method: HttpMethod,
    pub status_code: u16,
    pub request_bytes: u64,
    pub response_bytes: u64,
    pub latency_ms: f64,
    pub timestamp: DateTime<Utc>,
}

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

// ---------------------------------------------------------------------------
// Usage summary
// ---------------------------------------------------------------------------

/// Aggregated usage statistics for a key or endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub key_id: EntityId,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_request_bytes: u64,
    pub total_response_bytes: u64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub endpoints: HashMap<String, u64>,
}

/// Daily quota tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaStatus {
    pub key_id: EntityId,
    pub daily_limit: u64,
    pub used_today: u64,
    pub remaining: u64,
    pub percentage_used: f64,
    pub resets_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Usage tracker
// ---------------------------------------------------------------------------

/// Tracks API usage across all keys and endpoints.
pub struct UsageTracker {
    records: Vec<UsageRecord>,
    daily_counts: HashMap<EntityId, u64>,
    daily_reset: DateTime<Utc>,
    max_records: usize,
}

impl UsageTracker {
    /// Create a new usage tracker.
    pub fn new(max_records: usize) -> Self {
        Self {
            records: Vec::new(),
            daily_counts: HashMap::new(),
            daily_reset: next_midnight(),
            max_records,
        }
    }

    /// Record a usage event.
    pub fn record(&mut self, event: UsageRecord) {
        self.check_daily_reset();

        *self.daily_counts.entry(event.key_id).or_insert(0) += 1;

        // Ring buffer eviction if at capacity.
        if self.records.len() >= self.max_records {
            self.records.remove(0);
        }

        debug!(
            key = %event.key_id,
            endpoint = %event.endpoint,
            status = event.status_code,
            latency_ms = event.latency_ms,
            "usage recorded"
        );
        self.records.push(event);
    }

    /// Get usage summary for a key within a time range.
    pub fn summary(
        &self,
        key_id: &EntityId,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> UsageSummary {
        let matching: Vec<&UsageRecord> = self
            .records
            .iter()
            .filter(|r| r.key_id == *key_id && (from..=to).contains(&r.timestamp))
            .collect();

        let total_requests = matching.len() as u64;
        let successful = matching.iter().filter(|r| r.status_code < 400).count() as u64;
        let failed = total_requests - successful;
        let total_req_bytes: u64 = matching.iter().map(|r| r.request_bytes).sum();
        let total_resp_bytes: u64 = matching.iter().map(|r| r.response_bytes).sum();

        let avg_latency = if matching.is_empty() {
            0.0
        } else {
            matching.iter().map(|r| r.latency_ms).sum::<f64>() / matching.len() as f64
        };

        let p95_latency = percentile_latency(&matching, 95.0);

        let mut endpoints: HashMap<String, u64> = HashMap::new();
        for r in &matching {
            *endpoints.entry(r.endpoint.clone()).or_insert(0) += 1;
        }

        UsageSummary {
            key_id: *key_id,
            period_start: from,
            period_end: to,
            total_requests,
            successful_requests: successful,
            failed_requests: failed,
            total_request_bytes: total_req_bytes,
            total_response_bytes: total_resp_bytes,
            avg_latency_ms: avg_latency,
            p95_latency_ms: p95_latency,
            endpoints,
        }
    }

    /// Get daily quota status for a key.
    pub fn quota_status(&mut self, key_id: &EntityId, daily_limit: u64) -> QuotaStatus {
        self.check_daily_reset();
        let used = *self.daily_counts.get(key_id).unwrap_or(&0);
        let remaining = daily_limit.saturating_sub(used);
        let pct = if daily_limit > 0 {
            (used as f64 / daily_limit as f64) * 100.0
        } else {
            0.0
        };

        QuotaStatus {
            key_id: *key_id,
            daily_limit,
            used_today: used,
            remaining,
            percentage_used: pct,
            resets_at: self.daily_reset,
        }
    }

    /// Check if a key has exceeded its daily quota.
    pub fn is_over_quota(&mut self, key_id: &EntityId, daily_limit: u64) -> bool {
        self.check_daily_reset();
        let used = *self.daily_counts.get(key_id).unwrap_or(&0);
        used >= daily_limit
    }

    /// Total records stored.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Get the top N endpoints by request count in a time range.
    pub fn top_endpoints(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: usize,
    ) -> Vec<(String, u64)> {
        let mut counts: HashMap<String, u64> = HashMap::new();
        for r in &self.records {
            if (from..=to).contains(&r.timestamp) {
                *counts.entry(r.endpoint.clone()).or_insert(0) += 1;
            }
        }
        let mut sorted: Vec<(String, u64)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(limit);
        sorted
    }

    /// Get error rate for a key in a time range.
    pub fn error_rate(&self, key_id: &EntityId, from: DateTime<Utc>, to: DateTime<Utc>) -> f64 {
        let matching: Vec<&UsageRecord> = self
            .records
            .iter()
            .filter(|r| r.key_id == *key_id && (from..=to).contains(&r.timestamp))
            .collect();

        if matching.is_empty() {
            return 0.0;
        }

        let errors = matching.iter().filter(|r| r.status_code >= 400).count();
        errors as f64 / matching.len() as f64
    }

    /// Reset daily counts if past midnight.
    fn check_daily_reset(&mut self) {
        if Utc::now() >= self.daily_reset {
            self.daily_counts.clear();
            self.daily_reset = next_midnight();
            debug!("daily usage counters reset");
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn next_midnight() -> DateTime<Utc> {
    let now = Utc::now();
    (now + Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .map(|dt| dt.and_utc())
        .unwrap_or(now + Duration::days(1))
}

fn percentile_latency(records: &[&UsageRecord], pct: f64) -> f64 {
    if records.is_empty() {
        return 0.0;
    }
    let mut latencies: Vec<f64> = records.iter().map(|r| r.latency_ms).collect();
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = ((pct / 100.0) * (latencies.len() as f64 - 1.0)).ceil() as usize;
    let idx = idx.min(latencies.len() - 1);
    latencies[idx]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(key_id: EntityId, endpoint: &str, status: u16, latency: f64) -> UsageRecord {
        UsageRecord {
            id: EntityId::new(),
            key_id,
            endpoint: endpoint.to_string(),
            method: HttpMethod::Get,
            status_code: status,
            request_bytes: 100,
            response_bytes: 500,
            latency_ms: latency,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn record_and_count() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();

        tracker.record(make_record(key, "/position", 200, 10.0));
        tracker.record(make_record(key, "/route", 200, 20.0));
        assert_eq!(tracker.record_count(), 2);
    }

    #[test]
    fn ring_buffer_eviction() {
        let mut tracker = UsageTracker::new(3);
        let key = EntityId::new();

        tracker.record(make_record(key, "/a", 200, 1.0));
        tracker.record(make_record(key, "/b", 200, 2.0));
        tracker.record(make_record(key, "/c", 200, 3.0));
        tracker.record(make_record(key, "/d", 200, 4.0));

        assert_eq!(tracker.record_count(), 3);
        // First record (/a) should have been evicted.
        assert!(tracker.records.iter().all(|r| r.endpoint != "/a"));
    }

    #[test]
    fn summary_computes_stats() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();
        let from = Utc::now() - Duration::hours(1);

        tracker.record(make_record(key, "/position", 200, 10.0));
        tracker.record(make_record(key, "/position", 200, 20.0));
        tracker.record(make_record(key, "/route", 500, 30.0));

        let summary = tracker.summary(&key, from, Utc::now() + Duration::seconds(1));
        assert_eq!(summary.total_requests, 3);
        assert_eq!(summary.successful_requests, 2);
        assert_eq!(summary.failed_requests, 1);
        assert!((summary.avg_latency_ms - 20.0).abs() < 0.01);
        assert_eq!(summary.endpoints.len(), 2);
        assert_eq!(summary.endpoints["/position"], 2);
    }

    #[test]
    fn quota_tracking() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();

        tracker.record(make_record(key, "/a", 200, 1.0));
        tracker.record(make_record(key, "/b", 200, 1.0));

        let quota = tracker.quota_status(&key, 10);
        assert_eq!(quota.used_today, 2);
        assert_eq!(quota.remaining, 8);
        assert!((quota.percentage_used - 20.0).abs() < 0.01);
    }

    #[test]
    fn over_quota_detection() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();

        tracker.record(make_record(key, "/a", 200, 1.0));
        tracker.record(make_record(key, "/b", 200, 1.0));
        tracker.record(make_record(key, "/c", 200, 1.0));

        assert!(!tracker.is_over_quota(&key, 5));
        assert!(tracker.is_over_quota(&key, 3));
        assert!(tracker.is_over_quota(&key, 2));
    }

    #[test]
    fn top_endpoints() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();
        let from = Utc::now() - Duration::hours(1);

        tracker.record(make_record(key, "/position", 200, 1.0));
        tracker.record(make_record(key, "/position", 200, 1.0));
        tracker.record(make_record(key, "/position", 200, 1.0));
        tracker.record(make_record(key, "/route", 200, 1.0));
        tracker.record(make_record(key, "/health", 200, 1.0));

        let top = tracker.top_endpoints(from, Utc::now() + Duration::seconds(1), 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "/position");
        assert_eq!(top[0].1, 3);
    }

    #[test]
    fn error_rate_computation() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();
        let from = Utc::now() - Duration::hours(1);

        tracker.record(make_record(key, "/a", 200, 1.0));
        tracker.record(make_record(key, "/a", 200, 1.0));
        tracker.record(make_record(key, "/a", 500, 1.0));
        tracker.record(make_record(key, "/a", 429, 1.0));

        let rate = tracker.error_rate(&key, from, Utc::now() + Duration::seconds(1));
        assert!((rate - 0.5).abs() < 0.01); // 2 errors out of 4.
    }

    #[test]
    fn error_rate_no_records() {
        let tracker = UsageTracker::new(1000);
        let key = EntityId::new();
        let from = Utc::now() - Duration::hours(1);
        assert_eq!(
            tracker.error_rate(&key, from, Utc::now() + Duration::seconds(1)),
            0.0
        );
    }

    #[test]
    fn summary_different_key_isolated() {
        let mut tracker = UsageTracker::new(1000);
        let key_a = EntityId::new();
        let key_b = EntityId::new();
        let from = Utc::now() - Duration::hours(1);

        tracker.record(make_record(key_a, "/a", 200, 1.0));
        tracker.record(make_record(key_a, "/b", 200, 1.0));
        tracker.record(make_record(key_b, "/c", 200, 1.0));

        let sum_a = tracker.summary(&key_a, from, Utc::now() + Duration::seconds(1));
        assert_eq!(sum_a.total_requests, 2);

        let sum_b = tracker.summary(&key_b, from, Utc::now() + Duration::seconds(1));
        assert_eq!(sum_b.total_requests, 1);
    }

    #[test]
    fn p95_latency_computation() {
        let mut tracker = UsageTracker::new(1000);
        let key = EntityId::new();
        let from = Utc::now() - Duration::hours(1);

        // Add 20 records with increasing latency.
        for i in 1..=20 {
            tracker.record(make_record(key, "/a", 200, i as f64));
        }

        let summary = tracker.summary(&key, from, Utc::now() + Duration::seconds(1));
        // p95 should be near 19 or 20.
        assert!(summary.p95_latency_ms >= 19.0);
    }
}
