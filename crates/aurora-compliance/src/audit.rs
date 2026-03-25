//! Audit logging — immutable, tamper-evident log of system actions for compliance.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditCategory {
    DataAccess,
    DataModification,
    DataDeletion,
    Authentication,
    Authorization,
    Configuration,
    Export,
    Consent,
    System,
}

/// Severity of audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AuditSeverity {
    Info,
    Warning,
    Critical,
}

/// An immutable audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub category: AuditCategory,
    pub severity: AuditSeverity,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub details: String,
    pub ip_address: Option<String>,
    pub checksum: u64,
}

impl AuditEntry {
    /// Compute a simple checksum from the entry fields for tamper detection.
    fn compute_checksum(
        timestamp: DateTime<Utc>,
        actor: &str,
        action: &str,
        resource: &str,
    ) -> u64 {
        let input = format!("{}{}{}{}", timestamp, actor, action, resource);
        input
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
    }

    /// Create a new audit entry with computed checksum.
    pub fn new(
        category: AuditCategory,
        severity: AuditSeverity,
        actor: &str,
        action: &str,
        resource: &str,
        details: &str,
    ) -> Self {
        let timestamp = Utc::now();
        let checksum = Self::compute_checksum(timestamp, actor, action, resource);
        Self {
            id: Uuid::new_v4(),
            timestamp,
            category,
            severity,
            actor: actor.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            details: details.to_string(),
            ip_address: None,
            checksum,
        }
    }

    /// Verify the entry's checksum hasn't been tampered with.
    pub fn verify(&self) -> bool {
        let expected =
            Self::compute_checksum(self.timestamp, &self.actor, &self.action, &self.resource);
        self.checksum == expected
    }
}

/// Audit log — append-only, tamper-evident audit trail.
pub struct AuditLog {
    entries: RwLock<Vec<AuditEntry>>,
    max_entries: usize,
}

impl AuditLog {
    /// Create a new audit log with a retention limit.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
            max_entries,
        }
    }

    /// Record an audit entry.
    pub fn record(&self, entry: AuditEntry) {
        let mut entries = self.entries.write();
        entries.push(entry);
        if entries.len() > self.max_entries {
            let drain_count = (self.max_entries / 10).max(1);
            entries.drain(..drain_count);
        }
    }

    /// Query entries by category.
    pub fn query_by_category(&self, category: AuditCategory) -> Vec<AuditEntry> {
        self.entries
            .read()
            .iter()
            .filter(|e| e.category == category)
            .cloned()
            .collect()
    }

    /// Query entries by actor.
    pub fn query_by_actor(&self, actor: &str) -> Vec<AuditEntry> {
        self.entries
            .read()
            .iter()
            .filter(|e| e.actor == actor)
            .cloned()
            .collect()
    }

    /// Query entries in a time range.
    pub fn query_by_time(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<AuditEntry> {
        self.entries
            .read()
            .iter()
            .filter(|e| (from..=to).contains(&e.timestamp))
            .cloned()
            .collect()
    }

    /// Query entries by severity (at or above given level).
    pub fn query_by_severity(&self, min_severity: AuditSeverity) -> Vec<AuditEntry> {
        self.entries
            .read()
            .iter()
            .filter(|e| e.severity >= min_severity)
            .cloned()
            .collect()
    }

    /// Verify integrity of all entries.
    pub fn verify_integrity(&self) -> (usize, usize) {
        let entries = self.entries.read();
        let total = entries.len();
        let valid = entries.iter().filter(|e| e.verify()).count();
        (valid, total)
    }

    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.read().len()
    }

    /// Check if the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.read().is_empty()
    }

    /// Export all entries as JSON string.
    pub fn export_json(&self) -> String {
        serde_json::to_string(&*self.entries.read()).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info_entry(actor: &str, action: &str) -> AuditEntry {
        AuditEntry::new(
            AuditCategory::DataAccess,
            AuditSeverity::Info,
            actor,
            action,
            "users/123",
            "Read user profile",
        )
    }

    #[test]
    fn test_record_and_query() {
        let log = AuditLog::new(1000);
        log.record(info_entry("admin", "read"));
        log.record(info_entry("admin", "update"));
        log.record(info_entry("user1", "read"));
        assert_eq!(log.len(), 3);
        assert_eq!(log.query_by_actor("admin").len(), 2);
        assert_eq!(log.query_by_actor("user1").len(), 1);
    }

    #[test]
    fn test_checksum_verification() {
        let entry = info_entry("admin", "read");
        assert!(entry.verify());
    }

    #[test]
    fn test_tamper_detection() {
        let mut entry = info_entry("admin", "read");
        entry.actor = "hacker".to_string();
        assert!(!entry.verify());
    }

    #[test]
    fn test_query_by_category() {
        let log = AuditLog::new(1000);
        log.record(AuditEntry::new(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "user1",
            "login",
            "session",
            "Success",
        ));
        log.record(info_entry("user1", "read"));
        assert_eq!(
            log.query_by_category(AuditCategory::Authentication).len(),
            1
        );
        assert_eq!(log.query_by_category(AuditCategory::DataAccess).len(), 1);
    }

    #[test]
    fn test_query_by_severity() {
        let log = AuditLog::new(1000);
        log.record(AuditEntry::new(
            AuditCategory::DataDeletion,
            AuditSeverity::Critical,
            "admin",
            "delete",
            "users/*",
            "Bulk delete",
        ));
        log.record(info_entry("user1", "read"));
        let critical = log.query_by_severity(AuditSeverity::Critical);
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].action, "delete");
    }

    #[test]
    fn test_integrity_check() {
        let log = AuditLog::new(1000);
        log.record(info_entry("a", "read"));
        log.record(info_entry("b", "write"));
        let (valid, total) = log.verify_integrity();
        assert_eq!(valid, 2);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_eviction() {
        let log = AuditLog::new(5);
        for i in 0..10 {
            log.record(info_entry(&format!("user{i}"), "read"));
        }
        assert!(log.len() <= 5);
    }

    #[test]
    fn test_export_json() {
        let log = AuditLog::new(1000);
        log.record(info_entry("admin", "read"));
        let json = log.export_json();
        assert!(json.contains("admin"));
        assert!(json.contains("read"));
    }

    #[test]
    fn test_empty_log() {
        let log = AuditLog::new(1000);
        assert!(log.is_empty());
        assert_eq!(log.len(), 0);
        assert!(log.query_by_actor("nobody").is_empty());
    }

    #[test]
    fn test_ip_address() {
        let mut entry = info_entry("admin", "login");
        entry.ip_address = Some("192.168.1.1".to_string());
        assert_eq!(entry.ip_address, Some("192.168.1.1".to_string()));
    }
}
