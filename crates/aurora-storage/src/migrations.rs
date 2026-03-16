//! Schema migrations — versioned, ordered, idempotent migrations for storage evolution.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status of a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    Pending,
    Applied,
    Failed,
    RolledBack,
}

/// A single migration record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub version: u64,
    pub name: String,
    pub description: String,
    pub status: MigrationStatus,
    pub applied_at: Option<DateTime<Utc>>,
    pub checksum: u64,
}

/// Result of applying a migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationResult {
    Applied,
    AlreadyApplied,
    Failed(String),
    DependencyMissing(u64),
}

/// Migration manager — tracks and applies versioned schema changes.
pub struct MigrationManager {
    migrations: RwLock<Vec<Migration>>,
    applied_versions: RwLock<HashMap<u64, DateTime<Utc>>>,
}

impl MigrationManager {
    /// Create a new migration manager.
    pub fn new() -> Self {
        Self {
            migrations: RwLock::new(Vec::new()),
            applied_versions: RwLock::new(HashMap::new()),
        }
    }

    /// Register a migration (must be registered in version order).
    pub fn register(&self, version: u64, name: &str, description: &str, checksum: u64) {
        let mut migrations = self.migrations.write();
        if migrations.iter().any(|m| m.version == version) {
            return; // idempotent
        }
        migrations.push(Migration {
            version,
            name: name.to_string(),
            description: description.to_string(),
            status: MigrationStatus::Pending,
            applied_at: None,
            checksum,
        });
        migrations.sort_by_key(|m| m.version);
    }

    /// Apply a migration by version. Requires all prior versions to be applied.
    pub fn apply(&self, version: u64) -> MigrationResult {
        // Acquire migrations lock first to match rollback() lock ordering
        let mut migrations = self.migrations.write();

        let idx = match migrations.iter().position(|m| m.version == version) {
            Some(i) => i,
            None => return MigrationResult::Failed(format!("version {version} not registered")),
        };

        if migrations[idx].status == MigrationStatus::Applied {
            return MigrationResult::AlreadyApplied;
        }

        // Check dependencies using migration status (no separate lock needed)
        for m in migrations.iter().take(idx) {
            if m.status != MigrationStatus::Applied {
                return MigrationResult::DependencyMissing(m.version);
            }
        }

        let now = Utc::now();
        migrations[idx].status = MigrationStatus::Applied;
        migrations[idx].applied_at = Some(now);
        self.applied_versions.write().insert(version, now);

        MigrationResult::Applied
    }

    /// Roll back a migration by version. Only the latest applied can be rolled back.
    pub fn rollback(&self, version: u64) -> MigrationResult {
        let mut migrations = self.migrations.write();

        let idx = match migrations.iter().position(|m| m.version == version) {
            Some(i) => i,
            None => return MigrationResult::Failed(format!("version {version} not registered")),
        };

        if migrations[idx].status != MigrationStatus::Applied {
            return MigrationResult::Failed("not applied".to_string());
        }

        // Ensure no later migrations depend on this one
        for m in migrations.iter().skip(idx + 1) {
            if m.status == MigrationStatus::Applied {
                return MigrationResult::Failed(format!(
                    "cannot rollback: version {} depends on it",
                    m.version
                ));
            }
        }

        migrations[idx].status = MigrationStatus::RolledBack;
        migrations[idx].applied_at = None;
        self.applied_versions.write().remove(&version);

        MigrationResult::Applied
    }

    /// Get the current schema version (highest applied migration).
    pub fn current_version(&self) -> Option<u64> {
        self.migrations
            .read()
            .iter()
            .rev()
            .find(|m| m.status == MigrationStatus::Applied)
            .map(|m| m.version)
    }

    /// Get all pending migrations.
    pub fn pending(&self) -> Vec<Migration> {
        self.migrations
            .read()
            .iter()
            .filter(|m| m.status == MigrationStatus::Pending)
            .cloned()
            .collect()
    }

    /// Get all migrations.
    pub fn all(&self) -> Vec<Migration> {
        self.migrations.read().clone()
    }

    /// Apply all pending migrations in order.
    pub fn apply_all(&self) -> Vec<(u64, MigrationResult)> {
        let pending_versions: Vec<u64> = self
            .migrations
            .read()
            .iter()
            .filter(|m| m.status == MigrationStatus::Pending)
            .map(|m| m.version)
            .collect();

        pending_versions
            .into_iter()
            .map(|v| (v, self.apply(v)))
            .collect()
    }

    /// Verify checksum of a migration against expected value.
    pub fn verify_checksum(&self, version: u64, expected: u64) -> bool {
        self.migrations
            .read()
            .iter()
            .find(|m| m.version == version)
            .map_or(false, |m| m.checksum == expected)
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_apply() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "Create base tables", 0xABCD);
        mgr.register(2, "add_index", "Add spatial index", 0xDEF0);
        assert_eq!(mgr.pending().len(), 2);
        assert_eq!(mgr.apply(1), MigrationResult::Applied);
        assert_eq!(mgr.current_version(), Some(1));
        assert_eq!(mgr.pending().len(), 1);
    }

    #[test]
    fn test_dependency_check() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0);
        mgr.register(2, "second", "second", 0);
        assert_eq!(mgr.apply(2), MigrationResult::DependencyMissing(1));
    }

    #[test]
    fn test_idempotent_register() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0);
        mgr.register(1, "init_dup", "dup", 0);
        assert_eq!(mgr.all().len(), 1);
        assert_eq!(mgr.all()[0].name, "init");
    }

    #[test]
    fn test_already_applied() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0);
        mgr.apply(1);
        assert_eq!(mgr.apply(1), MigrationResult::AlreadyApplied);
    }

    #[test]
    fn test_rollback() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0);
        mgr.register(2, "idx", "idx", 0);
        mgr.apply(1);
        mgr.apply(2);
        assert_eq!(mgr.rollback(2), MigrationResult::Applied);
        assert_eq!(mgr.current_version(), Some(1));
    }

    #[test]
    fn test_rollback_dependency_block() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0);
        mgr.register(2, "idx", "idx", 0);
        mgr.apply(1);
        mgr.apply(2);
        let result = mgr.rollback(1);
        assert!(matches!(result, MigrationResult::Failed(_)));
    }

    #[test]
    fn test_apply_all() {
        let mgr = MigrationManager::new();
        mgr.register(1, "a", "a", 0);
        mgr.register(2, "b", "b", 0);
        mgr.register(3, "c", "c", 0);
        let results = mgr.apply_all();
        assert_eq!(results.len(), 3);
        for (_, r) in &results {
            assert_eq!(*r, MigrationResult::Applied);
        }
        assert_eq!(mgr.current_version(), Some(3));
    }

    #[test]
    fn test_verify_checksum() {
        let mgr = MigrationManager::new();
        mgr.register(1, "init", "init", 0xDEAD);
        assert!(mgr.verify_checksum(1, 0xDEAD));
        assert!(!mgr.verify_checksum(1, 0xBEEF));
        assert!(!mgr.verify_checksum(99, 0xDEAD));
    }
}
