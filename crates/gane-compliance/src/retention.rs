//! Data retention policies — automated lifecycle management for stored data.

use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Retention action when data expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionAction {
    Delete,
    Archive,
    Anonymise,
}

/// A data retention policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub id: Uuid,
    pub name: String,
    pub data_category: String,
    pub retention_days: u32,
    pub action: RetentionAction,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

impl RetentionPolicy {
    /// Create a new retention policy.
    pub fn new(
        name: &str,
        data_category: &str,
        retention_days: u32,
        action: RetentionAction,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            data_category: data_category.to_string(),
            retention_days,
            action,
            active: true,
            created_at: Utc::now(),
        }
    }

    /// Check if a given timestamp is expired under this policy.
    pub fn is_expired(&self, created_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        let expiry = created_at + Duration::days(self.retention_days as i64);
        now > expiry
    }
}

/// A tracked data record subject to retention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: Uuid,
    pub data_category: String,
    pub created_at: DateTime<Utc>,
    pub owner_id: Option<Uuid>,
    pub size_bytes: u64,
    pub metadata: std::collections::HashMap<String, String>,
    pub anonymised: bool,
}

/// Result of a retention sweep.
#[derive(Debug, Clone, Default)]
pub struct SweepResult {
    pub deleted: usize,
    pub archived: usize,
    pub anonymised: usize,
    pub bytes_freed: u64,
}

/// Retention manager — enforces data lifecycle policies.
pub struct RetentionManager {
    policies: RwLock<Vec<RetentionPolicy>>,
    records: RwLock<Vec<DataRecord>>,
}

impl RetentionManager {
    /// Create a new retention manager.
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(Vec::new()),
            records: RwLock::new(Vec::new()),
        }
    }

    /// Add a retention policy.
    pub fn add_policy(&self, policy: RetentionPolicy) {
        self.policies.write().push(policy);
    }

    /// Remove a policy by ID.
    pub fn remove_policy(&self, id: Uuid) -> bool {
        let mut policies = self.policies.write();
        let before = policies.len();
        policies.retain(|p| p.id != id);
        policies.len() < before
    }

    /// Register a data record.
    pub fn register_record(&self, record: DataRecord) {
        self.records.write().push(record);
    }

    /// Run a retention sweep — apply all active policies to all records.
    pub fn sweep(&self, now: DateTime<Utc>) -> SweepResult {
        let policies = self.policies.read();
        let mut records = self.records.write();
        let mut result = SweepResult::default();

        let active_policies: Vec<&RetentionPolicy> = policies.iter().filter(|p| p.active).collect();

        // First pass: anonymise records in-place (needs &mut)
        for record in records.iter_mut() {
            if record.anonymised {
                continue; // already anonymised, skip
            }
            for policy in &active_policies {
                if record.data_category == policy.data_category
                    && policy.is_expired(record.created_at, now)
                    && policy.action == RetentionAction::Anonymise
                {
                    record.owner_id = None;
                    record.metadata.clear();
                    record.anonymised = true;
                    result.anonymised += 1;
                    break;
                }
            }
        }

        // Second pass: remove deleted/archived records
        records.retain(|record| {
            for policy in &active_policies {
                if record.data_category == policy.data_category
                    && policy.is_expired(record.created_at, now)
                {
                    match policy.action {
                        RetentionAction::Delete => {
                            result.deleted += 1;
                            result.bytes_freed += record.size_bytes;
                            return false;
                        }
                        RetentionAction::Archive => {
                            result.archived += 1;
                            return false;
                        }
                        RetentionAction::Anonymise => {} // handled above
                    }
                }
            }
            true
        });

        result
    }

    /// Get records that will expire within the next N days.
    pub fn expiring_soon(&self, days: u32, now: DateTime<Utc>) -> Vec<DataRecord> {
        let policies = self.policies.read();
        let records = self.records.read();
        let horizon = now + Duration::days(days as i64);

        records
            .iter()
            .filter(|record| {
                policies.iter().any(|p| {
                    p.active
                        && record.data_category == p.data_category
                        && !p.is_expired(record.created_at, now)
                        && p.is_expired(record.created_at, horizon)
                })
            })
            .cloned()
            .collect()
    }

    /// Get all policies.
    pub fn policies(&self) -> Vec<RetentionPolicy> {
        self.policies.read().clone()
    }

    /// Count records by category.
    pub fn count_by_category(&self, category: &str) -> usize {
        self.records
            .read()
            .iter()
            .filter(|r| r.data_category == category)
            .count()
    }

    /// Total data size across all records.
    pub fn total_size_bytes(&self) -> u64 {
        self.records.read().iter().map(|r| r.size_bytes).sum()
    }

    /// Total record count.
    pub fn record_count(&self) -> usize {
        self.records.read().len()
    }
}

impl Default for RetentionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(category: &str, days_ago: i64, size: u64) -> DataRecord {
        DataRecord {
            id: Uuid::new_v4(),
            data_category: category.to_string(),
            created_at: Utc::now() - Duration::days(days_ago),
            owner_id: None,
            size_bytes: size,
            metadata: std::collections::HashMap::new(),
            anonymised: false,
        }
    }

    #[test]
    fn test_policy_expiration() {
        let policy = RetentionPolicy::new("logs", "system_logs", 30, RetentionAction::Delete);
        let old = Utc::now() - Duration::days(31);
        let recent = Utc::now() - Duration::days(5);
        assert!(policy.is_expired(old, Utc::now()));
        assert!(!policy.is_expired(recent, Utc::now()));
    }

    #[test]
    fn test_sweep_deletes_expired() {
        let mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "logs",
            "logs",
            30,
            RetentionAction::Delete,
        ));
        mgr.register_record(make_record("logs", 31, 1000));
        mgr.register_record(make_record("logs", 5, 500));
        mgr.register_record(make_record("other", 100, 200));

        let result = mgr.sweep(Utc::now());
        assert_eq!(result.deleted, 1);
        assert_eq!(result.bytes_freed, 1000);
        assert_eq!(mgr.record_count(), 2);
    }

    #[test]
    fn test_sweep_archives() {
        let mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "archive_nav",
            "nav_data",
            7,
            RetentionAction::Archive,
        ));
        mgr.register_record(make_record("nav_data", 10, 2000));
        let result = mgr.sweep(Utc::now());
        assert_eq!(result.archived, 1);
        assert_eq!(mgr.record_count(), 0);
    }

    #[test]
    fn test_inactive_policy_ignored() {
        let mgr = RetentionManager::new();
        let mut policy = RetentionPolicy::new("logs", "logs", 1, RetentionAction::Delete);
        policy.active = false;
        mgr.add_policy(policy);
        mgr.register_record(make_record("logs", 100, 500));
        let result = mgr.sweep(Utc::now());
        assert_eq!(result.deleted, 0);
        assert_eq!(mgr.record_count(), 1);
    }

    #[test]
    fn test_expiring_soon() {
        let mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "logs",
            "logs",
            30,
            RetentionAction::Delete,
        ));
        mgr.register_record(make_record("logs", 28, 100)); // expires in ~2 days
        mgr.register_record(make_record("logs", 5, 100)); // expires in ~25 days

        let soon = mgr.expiring_soon(7, Utc::now());
        assert_eq!(soon.len(), 1);
    }

    #[test]
    fn test_remove_policy() {
        let mgr = RetentionManager::new();
        let policy = RetentionPolicy::new("test", "test", 30, RetentionAction::Delete);
        let id = policy.id;
        mgr.add_policy(policy);
        assert!(mgr.remove_policy(id));
        assert!(!mgr.remove_policy(id));
        assert!(mgr.policies().is_empty());
    }

    #[test]
    fn test_count_by_category() {
        let mgr = RetentionManager::new();
        mgr.register_record(make_record("logs", 1, 100));
        mgr.register_record(make_record("logs", 2, 200));
        mgr.register_record(make_record("nav", 1, 300));
        assert_eq!(mgr.count_by_category("logs"), 2);
        assert_eq!(mgr.count_by_category("nav"), 1);
        assert_eq!(mgr.count_by_category("none"), 0);
    }

    #[test]
    fn test_total_size() {
        let mgr = RetentionManager::new();
        mgr.register_record(make_record("a", 1, 100));
        mgr.register_record(make_record("b", 1, 200));
        assert_eq!(mgr.total_size_bytes(), 300);
    }

    #[test]
    fn test_anonymise_keeps_record() {
        let mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "anon",
            "pii",
            7,
            RetentionAction::Anonymise,
        ));
        let mut rec = make_record("pii", 10, 500);
        rec.owner_id = Some(Uuid::new_v4());
        rec.metadata.insert("name".to_string(), "John".to_string());
        mgr.register_record(rec);

        let result = mgr.sweep(Utc::now());
        assert_eq!(result.anonymised, 1);
        assert_eq!(mgr.record_count(), 1); // record kept

        // Verify data was actually anonymised
        let records = mgr.records.read();
        assert!(records[0].anonymised);
        assert!(records[0].owner_id.is_none());
        assert!(records[0].metadata.is_empty());

        // Subsequent sweep should NOT re-count
        drop(records);
        let result2 = mgr.sweep(Utc::now());
        assert_eq!(result2.anonymised, 0);
    }
}
