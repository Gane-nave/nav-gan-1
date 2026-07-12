//! Immutable audit logging.

use chrono::Utc;
use gane_core::types::{AuditLog, EntityId, TransparencyLog};
use parking_lot::Mutex;
use std::collections::VecDeque;

/// Immutable, append-only audit logger.
pub struct AuditLogger {
    logs: Mutex<VecDeque<AuditLog>>,
    transparency_logs: Mutex<VecDeque<TransparencyLog>>,
    max_entries: usize,
}

impl AuditLogger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: Mutex::new(VecDeque::with_capacity(max_entries)),
            transparency_logs: Mutex::new(VecDeque::with_capacity(max_entries)),
            max_entries,
        }
    }

    /// Log an action.
    pub fn log_action(
        &self,
        actor_id: EntityId,
        action: &str,
        entity_type: &str,
        entity_id: EntityId,
        details: serde_json::Value,
    ) {
        let entry = AuditLog {
            id: EntityId::new(),
            timestamp: Utc::now(),
            actor_id,
            action: action.to_string(),
            entity_type: entity_type.to_string(),
            entity_id,
            details,
        };

        let mut logs = self.logs.lock();
        if logs.len() >= self.max_entries {
            logs.pop_front();
        }
        logs.push_back(entry);
    }

    /// Log algorithm transparency.
    pub fn log_transparency(
        &self,
        algorithm: &str,
        version: &str,
        input_hash: &str,
        output_hash: &str,
        parameters: serde_json::Value,
    ) {
        let entry = TransparencyLog {
            id: EntityId::new(),
            timestamp: Utc::now(),
            algorithm: algorithm.to_string(),
            version: version.to_string(),
            input_hash: input_hash.to_string(),
            output_hash: output_hash.to_string(),
            parameters,
        };

        let mut logs = self.transparency_logs.lock();
        if logs.len() >= self.max_entries {
            logs.pop_front();
        }
        logs.push_back(entry);
    }

    /// Get recent audit logs.
    pub fn recent_logs(&self, count: usize) -> Vec<AuditLog> {
        let logs = self.logs.lock();
        logs.iter().rev().take(count).cloned().collect()
    }

    /// Get recent transparency logs.
    pub fn recent_transparency_logs(&self, count: usize) -> Vec<TransparencyLog> {
        let logs = self.transparency_logs.lock();
        logs.iter().rev().take(count).cloned().collect()
    }

    /// Total audit log entries.
    pub fn audit_count(&self) -> usize {
        self.logs.lock().len()
    }

    /// Total transparency log entries.
    pub fn transparency_count(&self) -> usize {
        self.transparency_logs.lock().len()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(100_000)
    }
}
