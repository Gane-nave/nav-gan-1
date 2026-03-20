//! Audit trail — immutable log of all system events with tamper detection.

use std::collections::HashMap;

/// Severity level for audit events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    /// Informational event.
    Info,
    /// Warning event.
    Warning,
    /// Critical event requiring attention.
    Critical,
    /// Security-related event.
    Security,
}

/// An audit event category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    /// Authentication events.
    Authentication,
    /// Authorization events.
    Authorization,
    /// Data access events.
    DataAccess,
    /// Configuration changes.
    ConfigChange,
    /// System operations.
    SystemOp,
    /// User actions.
    UserAction,
    /// API calls.
    ApiCall,
    /// Security events.
    SecurityEvent,
}

/// An audit log entry.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// Unique entry ID.
    pub id: u64,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
    /// Event category.
    pub category: AuditCategory,
    /// Severity level.
    pub severity: AuditSeverity,
    /// Actor (who performed the action).
    pub actor: String,
    /// Action performed.
    pub action: String,
    /// Resource affected.
    pub resource: String,
    /// Outcome description.
    pub outcome: String,
    /// Whether the action succeeded.
    pub success: bool,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
    /// Hash of the previous entry (chain integrity).
    pub prev_hash: u64,
    /// Hash of this entry.
    pub hash: u64,
}

impl AuditEntry {
    /// Compute the hash for this entry.
    fn compute_hash(&self) -> u64 {
        let mut hash: u64 = self.prev_hash;
        hash = hash.wrapping_mul(31).wrapping_add(self.id);
        hash = hash.wrapping_mul(31).wrapping_add(self.timestamp_ms);
        hash = hash.wrapping_mul(31).wrapping_add(self.category as u64);
        hash = hash.wrapping_mul(31).wrapping_add(self.severity as u64);
        for b in self.actor.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as u64);
        }
        for b in self.action.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as u64);
        }
        for b in self.resource.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as u64);
        }
        for b in self.outcome.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(b as u64);
        }
        hash = hash.wrapping_mul(31).wrapping_add(self.success as u64);
        // Include sorted metadata keys for deterministic hashing
        let mut meta_keys: Vec<&String> = self.metadata.keys().collect();
        meta_keys.sort();
        for k in meta_keys {
            for b in k.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(b as u64);
            }
            if let Some(v) = self.metadata.get(k) {
                for b in v.bytes() {
                    hash = hash.wrapping_mul(31).wrapping_add(b as u64);
                }
            }
        }
        hash
    }

    /// Verify the integrity of this entry.
    pub fn verify_integrity(&self) -> bool {
        self.hash == self.compute_hash()
    }
}

/// Audit trail — append-only log with chain integrity.
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
    next_id: u64,
    last_hash: u64,
    /// Per-category counts.
    category_counts: HashMap<AuditCategory, u64>,
}

impl AuditTrail {
    /// Create a new audit trail.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            last_hash: 0,
            category_counts: HashMap::new(),
        }
    }

    /// Log a new audit event.
    #[allow(clippy::too_many_arguments)]
    pub fn log(
        &mut self,
        category: AuditCategory,
        severity: AuditSeverity,
        actor: &str,
        action: &str,
        resource: &str,
        outcome: &str,
        success: bool,
        timestamp_ms: u64,
    ) -> u64 {
        self.log_with_metadata(
            category,
            severity,
            actor,
            action,
            resource,
            outcome,
            success,
            timestamp_ms,
            HashMap::new(),
        )
    }

    /// Log a new audit event with metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn log_with_metadata(
        &mut self,
        category: AuditCategory,
        severity: AuditSeverity,
        actor: &str,
        action: &str,
        resource: &str,
        outcome: &str,
        success: bool,
        timestamp_ms: u64,
        metadata: HashMap<String, String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut entry = AuditEntry {
            id,
            timestamp_ms,
            category,
            severity,
            actor: actor.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            outcome: outcome.to_string(),
            success,
            metadata,
            prev_hash: self.last_hash,
            hash: 0,
        };
        entry.hash = entry.compute_hash();
        self.last_hash = entry.hash;

        *self.category_counts.entry(category).or_insert(0) += 1;
        self.entries.push(entry);
        id
    }

    /// Get an entry by ID.
    pub fn get(&self, id: u64) -> Option<&AuditEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Get all entries.
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    /// Get entries by category.
    pub fn entries_by_category(&self, category: AuditCategory) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Get entries by severity.
    pub fn entries_by_severity(&self, severity: AuditSeverity) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.severity == severity)
            .collect()
    }

    /// Get entries by actor.
    pub fn entries_by_actor(&self, actor: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }

    /// Get entries in a time range.
    pub fn entries_in_range(&self, start_ms: u64, end_ms: u64) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| (start_ms..=end_ms).contains(&e.timestamp_ms))
            .collect()
    }

    /// Get failed operations.
    pub fn failed_entries(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }

    /// Total entry count.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Count by category.
    pub fn count_by_category(&self, category: AuditCategory) -> u64 {
        self.category_counts.get(&category).copied().unwrap_or(0)
    }

    /// Verify chain integrity of the entire trail.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev_hash: u64 = 0;
        for entry in &self.entries {
            if entry.prev_hash != prev_hash {
                return false;
            }
            if !entry.verify_integrity() {
                return false;
            }
            prev_hash = entry.hash;
        }
        true
    }

    /// Security event count.
    pub fn security_event_count(&self) -> u64 {
        self.entries
            .iter()
            .filter(|e| e.severity == AuditSeverity::Security)
            .count() as u64
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_basic() {
        let mut trail = AuditTrail::new();
        let id = trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "user1",
            "login",
            "/auth",
            "Login successful",
            true,
            1000,
        );

        assert_eq!(id, 1);
        assert_eq!(trail.count(), 1);
        let entry = trail.get(1).unwrap();
        assert_eq!(entry.actor, "user1");
        assert!(entry.success);
    }

    #[test]
    fn test_audit_chain_integrity() {
        let mut trail = AuditTrail::new();
        for i in 0..10 {
            trail.log(
                AuditCategory::ApiCall,
                AuditSeverity::Info,
                &format!("user{i}"),
                "GET",
                "/api/v1",
                "OK",
                true,
                i * 1000,
            );
        }
        assert!(trail.verify_chain_integrity());
    }

    #[test]
    fn test_audit_tamper_detection() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::DataAccess,
            AuditSeverity::Info,
            "admin",
            "read",
            "/data/users",
            "OK",
            true,
            1000,
        );
        trail.log(
            AuditCategory::DataAccess,
            AuditSeverity::Info,
            "admin",
            "write",
            "/data/users",
            "OK",
            true,
            2000,
        );

        // Tamper with entry
        trail.entries[0].actor = "hacker".to_string();
        assert!(!trail.verify_chain_integrity());
    }

    #[test]
    fn test_audit_query_by_category() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "u1",
            "login",
            "/auth",
            "ok",
            true,
            0,
        );
        trail.log(
            AuditCategory::DataAccess,
            AuditSeverity::Info,
            "u1",
            "read",
            "/data",
            "ok",
            true,
            1,
        );
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Warning,
            "u2",
            "login",
            "/auth",
            "failed",
            false,
            2,
        );

        assert_eq!(
            trail
                .entries_by_category(AuditCategory::Authentication)
                .len(),
            2
        );
        assert_eq!(
            trail.entries_by_category(AuditCategory::DataAccess).len(),
            1
        );
    }

    #[test]
    fn test_audit_query_by_actor() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "alice",
            "GET",
            "/",
            "ok",
            true,
            0,
        );
        trail.log(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "bob",
            "POST",
            "/",
            "ok",
            true,
            1,
        );
        trail.log(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "alice",
            "PUT",
            "/",
            "ok",
            true,
            2,
        );

        assert_eq!(trail.entries_by_actor("alice").len(), 2);
        assert_eq!(trail.entries_by_actor("bob").len(), 1);
    }

    #[test]
    fn test_audit_failed_entries() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "u1",
            "login",
            "/",
            "ok",
            true,
            0,
        );
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Warning,
            "u2",
            "login",
            "/",
            "failed",
            false,
            1,
        );

        assert_eq!(trail.failed_entries().len(), 1);
    }

    #[test]
    fn test_audit_time_range() {
        let mut trail = AuditTrail::new();
        for i in 0..10 {
            trail.log(
                AuditCategory::ApiCall,
                AuditSeverity::Info,
                "u",
                "a",
                "/",
                "ok",
                true,
                i * 100,
            );
        }
        assert_eq!(trail.entries_in_range(200, 500).len(), 4);
    }

    #[test]
    fn test_audit_security_count() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::SecurityEvent,
            AuditSeverity::Security,
            "sys",
            "intrusion",
            "/",
            "blocked",
            true,
            0,
        );
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "u1",
            "login",
            "/",
            "ok",
            true,
            1,
        );
        trail.log(
            AuditCategory::SecurityEvent,
            AuditSeverity::Security,
            "sys",
            "brute_force",
            "/",
            "blocked",
            true,
            2,
        );

        assert_eq!(trail.security_event_count(), 2);
    }

    #[test]
    fn test_audit_metadata() {
        let mut trail = AuditTrail::new();
        let mut meta = HashMap::new();
        meta.insert("ip".to_string(), "192.168.1.1".to_string());
        meta.insert("user_agent".to_string(), "Mozilla/5.0".to_string());

        trail.log_with_metadata(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "u1",
            "GET",
            "/api",
            "ok",
            true,
            0,
            meta,
        );

        let entry = trail.get(1).unwrap();
        assert_eq!(entry.metadata.get("ip").unwrap(), "192.168.1.1");
    }

    #[test]
    fn test_audit_count_by_category() {
        let mut trail = AuditTrail::new();
        trail.log(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "u",
            "a",
            "/",
            "ok",
            true,
            0,
        );
        trail.log(
            AuditCategory::ApiCall,
            AuditSeverity::Info,
            "u",
            "a",
            "/",
            "ok",
            true,
            1,
        );
        trail.log(
            AuditCategory::Authentication,
            AuditSeverity::Info,
            "u",
            "a",
            "/",
            "ok",
            true,
            2,
        );

        assert_eq!(trail.count_by_category(AuditCategory::ApiCall), 2);
        assert_eq!(trail.count_by_category(AuditCategory::Authentication), 1);
        assert_eq!(trail.count_by_category(AuditCategory::DataAccess), 0);
    }
}
