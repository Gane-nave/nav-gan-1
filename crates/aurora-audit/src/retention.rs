//! Data retention — manage lifecycle policies for audit data, auto-purge expired records.

/// Retention policy action when data expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionAction {
    /// Delete the data permanently.
    Delete,
    /// Archive the data (move to cold storage).
    Archive,
    /// Anonymize the data (remove PII).
    Anonymize,
}

/// A retention policy.
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Policy identifier.
    pub id: String,
    /// Data category this policy applies to.
    pub data_category: String,
    /// Retention duration in milliseconds.
    pub retention_ms: u64,
    /// Action to take when data expires.
    pub action: RetentionAction,
    /// Whether the policy is active.
    pub active: bool,
    /// Description.
    pub description: String,
}

impl RetentionPolicy {
    /// Create a new retention policy.
    pub fn new(id: &str, data_category: &str, retention_ms: u64, action: RetentionAction) -> Self {
        Self {
            id: id.to_string(),
            data_category: data_category.to_string(),
            retention_ms,
            action,
            active: true,
            description: String::new(),
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Check if a timestamp has expired according to this policy.
    pub fn is_expired(&self, created_ms: u64, now_ms: u64) -> bool {
        now_ms >= created_ms + self.retention_ms
    }

    /// Get remaining time before expiry.
    pub fn remaining_ms(&self, created_ms: u64, now_ms: u64) -> u64 {
        let expiry = created_ms + self.retention_ms;
        expiry.saturating_sub(now_ms)
    }

    /// Retention period in days.
    pub fn retention_days(&self) -> f64 {
        self.retention_ms as f64 / 86_400_000.0
    }
}

/// A data record subject to retention.
#[derive(Debug, Clone)]
pub struct DataRecord {
    /// Record identifier.
    pub id: String,
    /// Data category.
    pub category: String,
    /// Creation timestamp (epoch millis).
    pub created_ms: u64,
    /// Whether the record has been archived.
    pub archived: bool,
    /// Whether the record has been anonymized.
    pub anonymized: bool,
}

impl DataRecord {
    /// Create a new data record.
    pub fn new(id: &str, category: &str, created_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            category: category.to_string(),
            created_ms,
            archived: false,
            anonymized: false,
        }
    }
}

/// Retention manager — applies policies to data records.
pub struct RetentionManager {
    policies: Vec<RetentionPolicy>,
    records: Vec<DataRecord>,
    deleted_count: u64,
    archived_count: u64,
    anonymized_count: u64,
}

impl RetentionManager {
    /// Create a new retention manager.
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
            records: Vec::new(),
            deleted_count: 0,
            archived_count: 0,
            anonymized_count: 0,
        }
    }

    /// Add a retention policy.
    pub fn add_policy(&mut self, policy: RetentionPolicy) {
        self.policies.push(policy);
    }

    /// Add a data record.
    pub fn add_record(&mut self, record: DataRecord) {
        self.records.push(record);
    }

    /// Find the matching policy for a data category.
    fn find_policy(&self, category: &str) -> Option<&RetentionPolicy> {
        self.policies
            .iter()
            .find(|p| p.active && p.data_category == category)
    }

    /// Apply retention policies — process expired records.
    /// Returns the number of records affected.
    pub fn apply_policies(&mut self, now_ms: u64) -> u64 {
        let mut affected = 0u64;
        let policies: Vec<(String, u64, RetentionAction)> = self
            .policies
            .iter()
            .filter(|p| p.active)
            .map(|p| (p.data_category.clone(), p.retention_ms, p.action))
            .collect();

        let mut to_delete = Vec::new();
        for (i, record) in self.records.iter_mut().enumerate() {
            for (cat, ret_ms, action) in &policies {
                if record.category == *cat && now_ms >= record.created_ms + ret_ms {
                    match action {
                        RetentionAction::Delete => {
                            to_delete.push(i);
                            self.deleted_count += 1;
                            affected += 1;
                        }
                        RetentionAction::Archive => {
                            if !record.archived {
                                record.archived = true;
                                self.archived_count += 1;
                                affected += 1;
                            }
                        }
                        RetentionAction::Anonymize => {
                            if !record.anonymized {
                                record.anonymized = true;
                                self.anonymized_count += 1;
                                affected += 1;
                            }
                        }
                    }
                    break;
                }
            }
        }

        // Remove deleted records (in reverse to preserve indices)
        for i in to_delete.into_iter().rev() {
            self.records.remove(i);
        }
        affected
    }

    /// Get expired records without applying actions.
    pub fn expired_records(&self, now_ms: u64) -> Vec<&DataRecord> {
        self.records
            .iter()
            .filter(|r| {
                self.find_policy(&r.category)
                    .is_some_and(|p| p.is_expired(r.created_ms, now_ms))
            })
            .collect()
    }

    /// Get record count.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Get policy count.
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Total deleted count.
    pub fn deleted_count(&self) -> u64 {
        self.deleted_count
    }

    /// Total archived count.
    pub fn archived_count(&self) -> u64 {
        self.archived_count
    }

    /// Total anonymized count.
    pub fn anonymized_count(&self) -> u64 {
        self.anonymized_count
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

    #[test]
    fn test_policy_expiry() {
        let policy = RetentionPolicy::new("p1", "logs", 86_400_000, RetentionAction::Delete);
        assert!(!policy.is_expired(0, 43_200_000)); // 12h < 24h
        assert!(policy.is_expired(0, 86_400_000)); // exactly 24h
        assert!(policy.is_expired(0, 100_000_000)); // past 24h
    }

    #[test]
    fn test_policy_remaining() {
        let policy = RetentionPolicy::new("p1", "logs", 86_400_000, RetentionAction::Delete);
        assert_eq!(policy.remaining_ms(0, 43_200_000), 43_200_000);
        assert_eq!(policy.remaining_ms(0, 86_400_000), 0);
    }

    #[test]
    fn test_retention_delete() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "logs",
            1000,
            RetentionAction::Delete,
        ));
        mgr.add_record(DataRecord::new("r1", "logs", 0));
        mgr.add_record(DataRecord::new("r2", "logs", 500));
        mgr.add_record(DataRecord::new("r3", "logs", 900));

        assert_eq!(mgr.record_count(), 3);

        // At t=1000, r1 should expire
        let affected = mgr.apply_policies(1000);
        assert!(affected >= 1);
        assert!(mgr.deleted_count() >= 1);
    }

    #[test]
    fn test_retention_archive() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "metrics",
            1000,
            RetentionAction::Archive,
        ));
        mgr.add_record(DataRecord::new("r1", "metrics", 0));

        mgr.apply_policies(2000);
        assert_eq!(mgr.archived_count(), 1);
        assert_eq!(mgr.record_count(), 1); // Not deleted, just archived
        assert!(mgr.records[0].archived);
    }

    #[test]
    fn test_retention_anonymize() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "user_data",
            1000,
            RetentionAction::Anonymize,
        ));
        mgr.add_record(DataRecord::new("r1", "user_data", 0));

        mgr.apply_policies(2000);
        assert_eq!(mgr.anonymized_count(), 1);
        assert!(mgr.records[0].anonymized);
    }

    #[test]
    fn test_retention_no_expiry() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "logs",
            10000,
            RetentionAction::Delete,
        ));
        mgr.add_record(DataRecord::new("r1", "logs", 0));

        let affected = mgr.apply_policies(5000);
        assert_eq!(affected, 0);
        assert_eq!(mgr.record_count(), 1);
    }

    #[test]
    fn test_expired_records_query() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "logs",
            1000,
            RetentionAction::Delete,
        ));
        mgr.add_record(DataRecord::new("r1", "logs", 0));
        mgr.add_record(DataRecord::new("r2", "logs", 900));

        let expired = mgr.expired_records(1000);
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].id, "r1");
    }

    #[test]
    fn test_policy_days() {
        let policy = RetentionPolicy::new("p1", "logs", 86_400_000 * 30, RetentionAction::Delete);
        assert!((policy.retention_days() - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_retention_mixed_categories() {
        let mut mgr = RetentionManager::new();
        mgr.add_policy(RetentionPolicy::new(
            "p1",
            "logs",
            1000,
            RetentionAction::Delete,
        ));
        mgr.add_policy(RetentionPolicy::new(
            "p2",
            "metrics",
            5000,
            RetentionAction::Archive,
        ));
        mgr.add_record(DataRecord::new("r1", "logs", 0));
        mgr.add_record(DataRecord::new("r2", "metrics", 0));

        mgr.apply_policies(2000);
        assert_eq!(mgr.deleted_count(), 1);
        assert_eq!(mgr.archived_count(), 0); // metrics not expired yet
        assert_eq!(mgr.record_count(), 1); // only metrics remains
    }
}
