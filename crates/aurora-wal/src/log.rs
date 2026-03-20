//! Write-Ahead Log implementation.

use std::collections::HashMap;

/// A single WAL entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalEntry {
    /// Log sequence number.
    pub lsn: u64,
    /// Operation type.
    pub op: String,
    /// Key affected.
    pub key: String,
    /// Value (if applicable).
    pub value: Option<String>,
    /// Timestamp (logical).
    pub timestamp: u64,
    /// Whether this entry has been checkpointed.
    pub checkpointed: bool,
}

/// A write-ahead log for crash recovery and durability.
///
/// Entries are appended sequentially with monotonically increasing LSNs.
/// Checkpoint support marks entries as persisted to stable storage.
#[derive(Debug)]
pub struct WriteAheadLog {
    /// All log entries in order.
    entries: Vec<WalEntry>,
    /// Next LSN to assign.
    next_lsn: u64,
    /// Last checkpointed LSN (all entries up to this LSN are safe).
    checkpoint_lsn: u64,
    /// Logical clock for timestamps.
    clock: u64,
    /// Index from key to latest LSN affecting that key.
    key_index: HashMap<String, u64>,
    /// Total bytes written (estimated).
    bytes_written: u64,
}

impl WriteAheadLog {
    /// Create a new empty WAL.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_lsn: 1,
            checkpoint_lsn: 0,
            clock: 0,
            key_index: HashMap::new(),
            bytes_written: 0,
        }
    }

    /// Append an entry to the log. Returns the assigned LSN.
    pub fn append(&mut self, op: &str, key: &str, value: Option<&str>) -> u64 {
        let lsn = self.next_lsn;
        self.next_lsn = self.next_lsn.saturating_add(1);
        self.clock = self.clock.saturating_add(1);

        let entry = WalEntry {
            lsn,
            op: op.to_string(),
            key: key.to_string(),
            value: value.map(|v| v.to_string()),
            timestamp: self.clock,
            checkpointed: false,
        };

        // Estimate bytes
        let size = entry.op.len() + entry.key.len()
            + entry.value.as_ref().map_or(0, |v| v.len()) + 24;
        self.bytes_written = self.bytes_written.saturating_add(size as u64);

        self.key_index.insert(key.to_string(), lsn);
        self.entries.push(entry);
        lsn
    }

    /// Checkpoint: mark all entries up to `lsn` as safely persisted.
    pub fn checkpoint(&mut self, lsn: u64) {
        if lsn > self.checkpoint_lsn {
            self.checkpoint_lsn = lsn;
            for entry in &mut self.entries {
                if entry.lsn <= lsn {
                    entry.checkpointed = true;
                }
            }
        }
    }

    /// Truncate all checkpointed entries (they're safely persisted).
    /// Returns the number of entries removed.
    pub fn truncate_checkpointed(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|e| !e.checkpointed);
        // Rebuild key index for remaining entries
        self.key_index.clear();
        for entry in &self.entries {
            self.key_index.insert(entry.key.clone(), entry.lsn);
        }
        before - self.entries.len()
    }

    /// Replay all entries since the last checkpoint (for crash recovery).
    pub fn replay_since_checkpoint(&self) -> Vec<&WalEntry> {
        self.entries
            .iter()
            .filter(|e| e.lsn > self.checkpoint_lsn)
            .collect()
    }

    /// Get all entries for a specific key.
    pub fn entries_for_key(&self, key: &str) -> Vec<&WalEntry> {
        self.entries.iter().filter(|e| e.key == key).collect()
    }

    /// Get the latest entry for a key.
    pub fn latest_for_key(&self, key: &str) -> Option<&WalEntry> {
        self.key_index.get(key).and_then(|lsn| {
            self.entries.iter().rev().find(|e| e.lsn == *lsn)
        })
    }

    /// Get entry by LSN.
    pub fn get(&self, lsn: u64) -> Option<&WalEntry> {
        self.entries.iter().find(|e| e.lsn == lsn)
    }

    /// Total number of entries in the log.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Last assigned LSN.
    pub fn last_lsn(&self) -> u64 {
        self.next_lsn.saturating_sub(1)
    }

    /// Last checkpointed LSN.
    pub fn checkpoint_lsn(&self) -> u64 {
        self.checkpoint_lsn
    }

    /// Number of entries not yet checkpointed.
    pub fn uncheckpointed_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.checkpointed).count()
    }

    /// Estimated total bytes written.
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    /// Get all entries as a slice.
    pub fn entries(&self) -> &[WalEntry] {
        &self.entries
    }

    /// Clear all entries and reset state.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.key_index.clear();
        self.next_lsn = 1;
        self.checkpoint_lsn = 0;
        self.clock = 0;
    }
}

impl Default for WriteAheadLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wal() {
        let wal = WriteAheadLog::new();
        assert!(wal.is_empty());
        assert_eq!(wal.len(), 0);
        assert_eq!(wal.last_lsn(), 0);
        assert_eq!(wal.checkpoint_lsn(), 0);
    }

    #[test]
    fn test_append() {
        let mut wal = WriteAheadLog::new();
        let lsn = wal.append("SET", "key1", Some("value1"));
        assert_eq!(lsn, 1);
        assert_eq!(wal.len(), 1);
        assert_eq!(wal.last_lsn(), 1);
    }

    #[test]
    fn test_sequential_lsns() {
        let mut wal = WriteAheadLog::new();
        let l1 = wal.append("SET", "a", Some("1"));
        let l2 = wal.append("SET", "b", Some("2"));
        let l3 = wal.append("DEL", "a", None);
        assert_eq!(l1, 1);
        assert_eq!(l2, 2);
        assert_eq!(l3, 3);
    }

    #[test]
    fn test_get_by_lsn() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "k", Some("v"));
        let entry = wal.get(1).unwrap();
        assert_eq!(entry.op, "SET");
        assert_eq!(entry.key, "k");
        assert_eq!(entry.value.as_deref(), Some("v"));
    }

    #[test]
    fn test_entries_for_key() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "x", Some("1"));
        wal.append("SET", "y", Some("2"));
        wal.append("SET", "x", Some("3"));
        let entries = wal.entries_for_key("x");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_latest_for_key() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "x", Some("old"));
        wal.append("SET", "x", Some("new"));
        let latest = wal.latest_for_key("x").unwrap();
        assert_eq!(latest.value.as_deref(), Some("new"));
    }

    #[test]
    fn test_checkpoint() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.append("SET", "b", Some("2"));
        wal.append("SET", "c", Some("3"));
        wal.checkpoint(2);
        assert_eq!(wal.checkpoint_lsn(), 2);
        assert_eq!(wal.uncheckpointed_count(), 1);
    }

    #[test]
    fn test_replay_since_checkpoint() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.append("SET", "b", Some("2"));
        wal.append("SET", "c", Some("3"));
        wal.checkpoint(1);
        let replay = wal.replay_since_checkpoint();
        assert_eq!(replay.len(), 2);
        assert_eq!(replay[0].key, "b");
        assert_eq!(replay[1].key, "c");
    }

    #[test]
    fn test_truncate_checkpointed() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.append("SET", "b", Some("2"));
        wal.append("SET", "c", Some("3"));
        wal.checkpoint(2);
        let removed = wal.truncate_checkpointed();
        assert_eq!(removed, 2);
        assert_eq!(wal.len(), 1);
        assert_eq!(wal.entries()[0].key, "c");
    }

    #[test]
    fn test_bytes_written() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "key", Some("value"));
        assert!(wal.bytes_written() > 0);
    }

    #[test]
    fn test_clear() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.append("SET", "b", Some("2"));
        wal.clear();
        assert!(wal.is_empty());
        assert_eq!(wal.last_lsn(), 0);
        assert_eq!(wal.checkpoint_lsn(), 0);
    }

    #[test]
    fn test_default() {
        let wal = WriteAheadLog::default();
        assert!(wal.is_empty());
    }

    #[test]
    fn test_no_value() {
        let mut wal = WriteAheadLog::new();
        let lsn = wal.append("DEL", "key", None);
        let entry = wal.get(lsn).unwrap();
        assert!(entry.value.is_none());
    }

    #[test]
    fn test_timestamps_increase() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.append("SET", "b", Some("2"));
        let entries = wal.entries();
        assert!(entries[1].timestamp > entries[0].timestamp);
    }

    #[test]
    fn test_checkpoint_idempotent() {
        let mut wal = WriteAheadLog::new();
        wal.append("SET", "a", Some("1"));
        wal.checkpoint(1);
        wal.checkpoint(1); // no-op
        assert_eq!(wal.checkpoint_lsn(), 1);
    }
}
