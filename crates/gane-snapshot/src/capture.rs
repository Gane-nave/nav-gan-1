//! Capture — point-in-time state snapshots.

use std::collections::HashMap;

/// A key-value entry in a snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotEntry {
    pub key: String,
    pub value: Vec<u8>,
}

/// A point-in-time snapshot of system state.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Unique snapshot ID.
    pub id: u64,
    /// Timestamp when the snapshot was taken (epoch ms).
    pub timestamp_ms: u64,
    /// Snapshot entries.
    entries: HashMap<String, Vec<u8>>,
    /// Size in bytes.
    size_bytes: usize,
}

impl Snapshot {
    /// Create a new empty snapshot.
    pub fn new(id: u64, timestamp_ms: u64) -> Self {
        Self {
            id,
            timestamp_ms,
            entries: HashMap::new(),
            size_bytes: 0,
        }
    }

    /// Add or update an entry.
    pub fn put(&mut self, key: &str, value: Vec<u8>) {
        let value_len = value.len();
        if let Some(old) = self.entries.insert(key.to_string(), value) {
            self.size_bytes = self.size_bytes.saturating_sub(old.len());
        }
        self.size_bytes += value_len;
    }

    /// Get an entry by key.
    pub fn get(&self, key: &str) -> Option<&[u8]> {
        self.entries.get(key).map(|v| v.as_slice())
    }

    /// Remove an entry.
    pub fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(v) = self.entries.remove(key) {
            self.size_bytes = self.size_bytes.saturating_sub(v.len());
            Some(v)
        } else {
            None
        }
    }

    /// Number of entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Total size in bytes.
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }

    /// All keys in the snapshot.
    pub fn keys(&self) -> Vec<&str> {
        self.entries.keys().map(|k| k.as_str()).collect()
    }

    /// Whether the snapshot contains a key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_slice()))
    }
}

/// Manages a series of snapshots with retention policy.
pub struct SnapshotStore {
    snapshots: Vec<Snapshot>,
    max_snapshots: usize,
    next_id: u64,
}

impl SnapshotStore {
    /// Create a new snapshot store.
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots,
            next_id: 1,
        }
    }

    /// Take a new snapshot (returns a mutable reference to fill).
    pub fn create(&mut self, timestamp_ms: u64) -> &mut Snapshot {
        let id = self.next_id;
        self.next_id += 1;

        let snapshot = Snapshot::new(id, timestamp_ms);
        self.snapshots.push(snapshot);

        // Enforce retention
        while self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }

        self.snapshots.last_mut().unwrap()
    }

    /// Get the latest snapshot.
    pub fn latest(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }

    /// Get a snapshot by ID.
    pub fn get(&self, id: u64) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    /// Number of stored snapshots.
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    /// Total size of all snapshots in bytes.
    pub fn total_size_bytes(&self) -> usize {
        self.snapshots.iter().map(|s| s.size_bytes()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_put_get() {
        let mut snap = Snapshot::new(1, 1000);
        snap.put("lat", vec![1, 2, 3, 4]);
        snap.put("lon", vec![5, 6, 7, 8]);

        assert_eq!(snap.get("lat"), Some([1, 2, 3, 4].as_slice()));
        assert_eq!(snap.get("lon"), Some([5, 6, 7, 8].as_slice()));
        assert_eq!(snap.entry_count(), 2);
        assert_eq!(snap.size_bytes(), 8);
    }

    #[test]
    fn test_snapshot_overwrite() {
        let mut snap = Snapshot::new(1, 1000);
        snap.put("key", vec![1, 2, 3]);
        assert_eq!(snap.size_bytes(), 3);

        snap.put("key", vec![4, 5]);
        assert_eq!(snap.get("key"), Some([4, 5].as_slice()));
        assert_eq!(snap.size_bytes(), 2);
        assert_eq!(snap.entry_count(), 1);
    }

    #[test]
    fn test_snapshot_remove() {
        let mut snap = Snapshot::new(1, 1000);
        snap.put("key", vec![1, 2, 3]);
        let removed = snap.remove("key");
        assert_eq!(removed, Some(vec![1, 2, 3]));
        assert_eq!(snap.entry_count(), 0);
        assert_eq!(snap.size_bytes(), 0);
    }

    #[test]
    fn test_snapshot_contains_key() {
        let mut snap = Snapshot::new(1, 1000);
        snap.put("exists", vec![1]);
        assert!(snap.contains_key("exists"));
        assert!(!snap.contains_key("missing"));
    }

    #[test]
    fn test_store_create_and_latest() {
        let mut store = SnapshotStore::new(5);
        let snap = store.create(1000);
        snap.put("key", vec![1]);

        assert_eq!(store.count(), 1);
        assert_eq!(store.latest().unwrap().id, 1);
    }

    #[test]
    fn test_store_retention() {
        let mut store = SnapshotStore::new(3);
        store.create(1000);
        store.create(2000);
        store.create(3000);
        store.create(4000); // should evict oldest

        assert_eq!(store.count(), 3);
        assert!(store.get(1).is_none()); // evicted
        assert!(store.get(4).is_some()); // latest
    }

    #[test]
    fn test_store_total_size() {
        let mut store = SnapshotStore::new(10);
        let s1 = store.create(1000);
        s1.put("a", vec![1, 2, 3]);
        let s2 = store.create(2000);
        s2.put("b", vec![4, 5]);

        assert_eq!(store.total_size_bytes(), 5);
    }

    #[test]
    fn test_snapshot_iter() {
        let mut snap = Snapshot::new(1, 1000);
        snap.put("a", vec![1]);
        snap.put("b", vec![2]);

        let entries: Vec<_> = snap.iter().collect();
        assert_eq!(entries.len(), 2);
    }
}
