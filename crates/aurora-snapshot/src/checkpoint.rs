//! Checkpoint — named restore points with metadata.

use std::collections::HashMap;

/// Checkpoint status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointStatus {
    /// Checkpoint is being created.
    Creating,
    /// Checkpoint is complete and valid.
    Valid,
    /// Checkpoint is corrupted or incomplete.
    Invalid,
    /// Checkpoint has been restored.
    Restored,
}

impl CheckpointStatus {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckpointStatus::Creating => "creating",
            CheckpointStatus::Valid => "valid",
            CheckpointStatus::Invalid => "invalid",
            CheckpointStatus::Restored => "restored",
        }
    }

    /// Whether the checkpoint can be restored.
    pub fn is_restorable(&self) -> bool {
        *self == CheckpointStatus::Valid
    }
}

/// A named checkpoint with metadata and data reference.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    /// Unique checkpoint ID.
    pub id: u64,
    /// Human-readable name.
    pub name: String,
    /// Timestamp when created (epoch ms).
    pub created_ms: u64,
    /// Status of this checkpoint.
    pub status: CheckpointStatus,
    /// Associated snapshot ID.
    pub snapshot_id: u64,
    /// Metadata tags.
    metadata: HashMap<String, String>,
}

impl Checkpoint {
    /// Create a new checkpoint.
    pub fn new(id: u64, name: &str, created_ms: u64, snapshot_id: u64) -> Self {
        Self {
            id,
            name: name.to_string(),
            created_ms,
            status: CheckpointStatus::Creating,
            snapshot_id,
            metadata: HashMap::new(),
        }
    }

    /// Mark the checkpoint as valid.
    pub fn mark_valid(&mut self) {
        self.status = CheckpointStatus::Valid;
    }

    /// Mark the checkpoint as invalid.
    pub fn mark_invalid(&mut self) {
        self.status = CheckpointStatus::Invalid;
    }

    /// Mark as restored.
    pub fn mark_restored(&mut self) {
        self.status = CheckpointStatus::Restored;
    }

    /// Set a metadata tag.
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Get a metadata tag.
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    /// All metadata keys.
    pub fn metadata_keys(&self) -> Vec<&str> {
        self.metadata.keys().map(|k| k.as_str()).collect()
    }
}

/// Registry of checkpoints.
pub struct CheckpointRegistry {
    checkpoints: Vec<Checkpoint>,
    max_checkpoints: usize,
    next_id: u64,
}

impl CheckpointRegistry {
    /// Create a new registry.
    pub fn new(max_checkpoints: usize) -> Self {
        Self {
            checkpoints: Vec::new(),
            max_checkpoints,
            next_id: 1,
        }
    }

    /// Create a new checkpoint.
    pub fn create(&mut self, name: &str, created_ms: u64, snapshot_id: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let checkpoint = Checkpoint::new(id, name, created_ms, snapshot_id);
        self.checkpoints.push(checkpoint);

        // Enforce retention — remove oldest valid checkpoints
        while self.checkpoints.len() > self.max_checkpoints {
            self.checkpoints.remove(0);
        }

        id
    }

    /// Get a checkpoint by ID.
    pub fn get(&self, id: u64) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| c.id == id)
    }

    /// Get a mutable checkpoint by ID.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Checkpoint> {
        self.checkpoints.iter_mut().find(|c| c.id == id)
    }

    /// Find a checkpoint by name.
    pub fn find_by_name(&self, name: &str) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| c.name == name)
    }

    /// Get the latest valid checkpoint.
    pub fn latest_valid(&self) -> Option<&Checkpoint> {
        self.checkpoints
            .iter()
            .rev()
            .find(|c| c.status == CheckpointStatus::Valid)
    }

    /// Number of checkpoints.
    pub fn count(&self) -> usize {
        self.checkpoints.len()
    }

    /// List all checkpoint names.
    pub fn names(&self) -> Vec<&str> {
        self.checkpoints.iter().map(|c| c.name.as_str()).collect()
    }

    /// Remove a checkpoint by ID.
    pub fn remove(&mut self, id: u64) -> bool {
        let len_before = self.checkpoints.len();
        self.checkpoints.retain(|c| c.id != id);
        self.checkpoints.len() < len_before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_creation() {
        let cp = Checkpoint::new(1, "pre-route", 5000, 10);
        assert_eq!(cp.id, 1);
        assert_eq!(cp.name, "pre-route");
        assert_eq!(cp.status, CheckpointStatus::Creating);
        assert_eq!(cp.snapshot_id, 10);
    }

    #[test]
    fn test_checkpoint_lifecycle() {
        let mut cp = Checkpoint::new(1, "test", 5000, 10);
        assert!(!cp.status.is_restorable());

        cp.mark_valid();
        assert!(cp.status.is_restorable());

        cp.mark_restored();
        assert_eq!(cp.status, CheckpointStatus::Restored);
        assert!(!cp.status.is_restorable());
    }

    #[test]
    fn test_checkpoint_metadata() {
        let mut cp = Checkpoint::new(1, "test", 5000, 10);
        cp.set_metadata("reason", "user-triggered");
        cp.set_metadata("version", "1.0");

        assert_eq!(cp.get_metadata("reason"), Some("user-triggered"));
        assert_eq!(cp.get_metadata("version"), Some("1.0"));
        assert_eq!(cp.get_metadata("missing"), None);
        assert_eq!(cp.metadata_keys().len(), 2);
    }

    #[test]
    fn test_checkpoint_status_display() {
        assert_eq!(CheckpointStatus::Creating.as_str(), "creating");
        assert_eq!(CheckpointStatus::Valid.as_str(), "valid");
        assert_eq!(CheckpointStatus::Invalid.as_str(), "invalid");
        assert_eq!(CheckpointStatus::Restored.as_str(), "restored");
    }

    #[test]
    fn test_registry_create_and_get() {
        let mut reg = CheckpointRegistry::new(10);
        let id = reg.create("cp1", 1000, 1);
        assert!(reg.get(id).is_some());
        assert_eq!(reg.get(id).unwrap().name, "cp1");
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn test_registry_find_by_name() {
        let mut reg = CheckpointRegistry::new(10);
        reg.create("alpha", 1000, 1);
        reg.create("beta", 2000, 2);

        assert!(reg.find_by_name("alpha").is_some());
        assert!(reg.find_by_name("gamma").is_none());
    }

    #[test]
    fn test_registry_latest_valid() {
        let mut reg = CheckpointRegistry::new(10);
        let id1 = reg.create("cp1", 1000, 1);
        reg.get_mut(id1).unwrap().mark_valid();
        let id2 = reg.create("cp2", 2000, 2);
        reg.get_mut(id2).unwrap().mark_invalid();

        let latest = reg.latest_valid().unwrap();
        assert_eq!(latest.name, "cp1"); // cp2 is invalid
    }

    #[test]
    fn test_registry_retention() {
        let mut reg = CheckpointRegistry::new(3);
        reg.create("a", 1000, 1);
        reg.create("b", 2000, 2);
        reg.create("c", 3000, 3);
        reg.create("d", 4000, 4); // evicts "a"

        assert_eq!(reg.count(), 3);
        assert!(reg.find_by_name("a").is_none());
        assert!(reg.find_by_name("d").is_some());
    }

    #[test]
    fn test_registry_remove() {
        let mut reg = CheckpointRegistry::new(10);
        let id = reg.create("cp1", 1000, 1);
        assert!(reg.remove(id));
        assert_eq!(reg.count(), 0);
        assert!(!reg.remove(id)); // already removed
    }

    #[test]
    fn test_registry_names() {
        let mut reg = CheckpointRegistry::new(10);
        reg.create("alpha", 1000, 1);
        reg.create("beta", 2000, 2);

        let names = reg.names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }
}
