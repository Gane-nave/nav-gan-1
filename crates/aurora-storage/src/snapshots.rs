//! Backup/restore snapshots — point-in-time state capture and restoration.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Snapshot status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotStatus {
    Creating,
    Complete,
    Failed,
    Restoring,
    Restored,
}

/// Metadata for a point-in-time snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub status: SnapshotStatus,
    pub size_bytes: u64,
    pub entry_count: usize,
    pub tags: HashMap<String, String>,
    data: HashMap<String, serde_json::Value>,
}

/// Snapshot manager — create, list, restore point-in-time backups.
pub struct SnapshotManager {
    snapshots: RwLock<Vec<Snapshot>>,
    max_snapshots: usize,
}

impl SnapshotManager {
    /// Create a new manager with a retention limit.
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: RwLock::new(Vec::new()),
            max_snapshots,
        }
    }

    /// Create a snapshot from a set of key-value pairs.
    pub fn create(
        &self,
        name: &str,
        data: HashMap<String, serde_json::Value>,
        tags: HashMap<String, String>,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let serialized = serde_json::to_string(&data).unwrap_or_default();
        let size_bytes = serialized.len() as u64;
        let entry_count = data.len();

        let snapshot = Snapshot {
            id,
            name: name.to_string(),
            created_at: Utc::now(),
            status: SnapshotStatus::Complete,
            size_bytes,
            entry_count,
            tags,
            data,
        };

        let mut snapshots = self.snapshots.write();
        snapshots.push(snapshot);

        // Enforce retention: remove oldest if over limit
        if snapshots.len() > self.max_snapshots {
            let remove_count = snapshots.len() - self.max_snapshots;
            snapshots.drain(..remove_count);
        }

        id
    }

    /// Get snapshot metadata by ID.
    pub fn get(&self, id: Uuid) -> Option<Snapshot> {
        self.snapshots.read().iter().find(|s| s.id == id).cloned()
    }

    /// Restore data from a snapshot. Returns the stored key-value pairs.
    pub fn restore(&self, id: Uuid) -> Option<HashMap<String, serde_json::Value>> {
        let mut snapshots = self.snapshots.write();
        let snap = snapshots.iter_mut().find(|s| s.id == id)?;
        if snap.status != SnapshotStatus::Complete {
            return None;
        }
        snap.status = SnapshotStatus::Restored;
        Some(snap.data.clone())
    }

    /// List all snapshots (most recent first).
    pub fn list(&self) -> Vec<Snapshot> {
        let mut snaps: Vec<Snapshot> = self.snapshots.read().clone();
        snaps.sort_by_key(|s| std::cmp::Reverse(s.created_at));
        snaps
    }

    /// Delete a snapshot by ID.
    pub fn delete(&self, id: Uuid) -> bool {
        let mut snapshots = self.snapshots.write();
        let before = snapshots.len();
        snapshots.retain(|s| s.id != id);
        snapshots.len() < before
    }

    /// Find snapshots by tag.
    pub fn find_by_tag(&self, key: &str, value: &str) -> Vec<Snapshot> {
        self.snapshots
            .read()
            .iter()
            .filter(|s| s.tags.get(key).is_some_and(|v| v == value))
            .cloned()
            .collect()
    }

    /// Total number of snapshots.
    pub fn count(&self) -> usize {
        self.snapshots.read().len()
    }

    /// Total storage used by all snapshots.
    pub fn total_size_bytes(&self) -> u64 {
        self.snapshots.read().iter().map(|s| s.size_bytes).sum()
    }

    /// Get the latest snapshot.
    pub fn latest(&self) -> Option<Snapshot> {
        self.snapshots
            .read()
            .iter()
            .max_by_key(|s| s.created_at)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> HashMap<String, serde_json::Value> {
        let mut m = HashMap::new();
        m.insert(
            "route".to_string(),
            serde_json::json!({"from": "A", "to": "B"}),
        );
        m.insert("speed".to_string(), serde_json::json!(65.5));
        m
    }

    fn sample_tags() -> HashMap<String, String> {
        let mut t = HashMap::new();
        t.insert("type".to_string(), "manual".to_string());
        t
    }

    #[test]
    fn test_create_and_get() {
        let mgr = SnapshotManager::new(10);
        let id = mgr.create("backup-1", sample_data(), sample_tags());
        let snap = mgr.get(id).unwrap();
        assert_eq!(snap.name, "backup-1");
        assert_eq!(snap.entry_count, 2);
        assert_eq!(snap.status, SnapshotStatus::Complete);
    }

    #[test]
    fn test_restore() {
        let mgr = SnapshotManager::new(10);
        let id = mgr.create("snap", sample_data(), HashMap::new());
        let data = mgr.restore(id).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data["speed"], serde_json::json!(65.5));
        // After restore, status changes
        let snap = mgr.get(id).unwrap();
        assert_eq!(snap.status, SnapshotStatus::Restored);
        // Cannot restore again (status is Restored, not Complete)
        assert!(mgr.restore(id).is_none());
    }

    #[test]
    fn test_retention_limit() {
        let mgr = SnapshotManager::new(3);
        for i in 0..5 {
            mgr.create(&format!("snap-{i}"), sample_data(), HashMap::new());
        }
        assert_eq!(mgr.count(), 3);
    }

    #[test]
    fn test_delete() {
        let mgr = SnapshotManager::new(10);
        let id = mgr.create("snap", sample_data(), HashMap::new());
        assert!(mgr.delete(id));
        assert!(mgr.get(id).is_none());
        assert!(!mgr.delete(id)); // already deleted
    }

    #[test]
    fn test_find_by_tag() {
        let mgr = SnapshotManager::new(10);
        let mut tags1 = HashMap::new();
        tags1.insert("env".to_string(), "prod".to_string());
        let mut tags2 = HashMap::new();
        tags2.insert("env".to_string(), "staging".to_string());
        mgr.create("prod-snap", sample_data(), tags1);
        mgr.create("stage-snap", sample_data(), tags2);
        let found = mgr.find_by_tag("env", "prod");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "prod-snap");
    }

    #[test]
    fn test_list_ordering() {
        let mgr = SnapshotManager::new(10);
        mgr.create("first", sample_data(), HashMap::new());
        mgr.create("second", sample_data(), HashMap::new());
        let list = mgr.list();
        assert_eq!(list.len(), 2);
        assert!(list[0].created_at >= list[1].created_at);
    }

    #[test]
    fn test_total_size() {
        let mgr = SnapshotManager::new(10);
        mgr.create("a", sample_data(), HashMap::new());
        mgr.create("b", sample_data(), HashMap::new());
        assert!(mgr.total_size_bytes() > 0);
    }

    #[test]
    fn test_latest() {
        let mgr = SnapshotManager::new(10);
        assert!(mgr.latest().is_none());
        mgr.create("first", sample_data(), HashMap::new());
        mgr.create("second", sample_data(), HashMap::new());
        assert_eq!(mgr.latest().unwrap().name, "second");
    }

    #[test]
    fn test_missing_snapshot() {
        let mgr = SnapshotManager::new(10);
        assert!(mgr.get(Uuid::new_v4()).is_none());
        assert!(mgr.restore(Uuid::new_v4()).is_none());
    }
}
