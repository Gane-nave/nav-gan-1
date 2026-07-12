//! Diff — compute differences between snapshots.

use std::collections::HashMap;

/// Type of change between two snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    /// Key was added.
    Added,
    /// Key was removed.
    Removed,
    /// Value was modified.
    Modified,
}

impl ChangeType {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeType::Added => "added",
            ChangeType::Removed => "removed",
            ChangeType::Modified => "modified",
        }
    }
}

/// A single change between two snapshots.
#[derive(Debug, Clone)]
pub struct Change {
    /// The key that changed.
    pub key: String,
    /// Type of change.
    pub change_type: ChangeType,
    /// Old value (if removed or modified).
    pub old_value: Option<Vec<u8>>,
    /// New value (if added or modified).
    pub new_value: Option<Vec<u8>>,
}

/// A diff between two snapshots.
#[derive(Debug, Clone)]
pub struct SnapshotDiff {
    /// Source snapshot ID.
    pub from_id: u64,
    /// Target snapshot ID.
    pub to_id: u64,
    /// List of changes.
    changes: Vec<Change>,
}

impl SnapshotDiff {
    /// Compute the diff between two key-value maps.
    pub fn compute(
        from_id: u64,
        from: &HashMap<String, Vec<u8>>,
        to_id: u64,
        to: &HashMap<String, Vec<u8>>,
    ) -> Self {
        let mut changes = Vec::new();

        // Find removed and modified keys
        for (key, old_val) in from {
            match to.get(key) {
                None => {
                    changes.push(Change {
                        key: key.clone(),
                        change_type: ChangeType::Removed,
                        old_value: Some(old_val.clone()),
                        new_value: None,
                    });
                }
                Some(new_val) => {
                    if old_val != new_val {
                        changes.push(Change {
                            key: key.clone(),
                            change_type: ChangeType::Modified,
                            old_value: Some(old_val.clone()),
                            new_value: Some(new_val.clone()),
                        });
                    }
                }
            }
        }

        // Find added keys
        for (key, new_val) in to {
            if !from.contains_key(key) {
                changes.push(Change {
                    key: key.clone(),
                    change_type: ChangeType::Added,
                    old_value: None,
                    new_value: Some(new_val.clone()),
                });
            }
        }

        // Sort by key for deterministic output
        changes.sort_by(|a, b| a.key.cmp(&b.key));

        Self {
            from_id,
            to_id,
            changes,
        }
    }

    /// Number of changes.
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Whether the diff is empty (no changes).
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Get all changes.
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    /// Count of added keys.
    pub fn added_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .count()
    }

    /// Count of removed keys.
    pub fn removed_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Removed)
            .count()
    }

    /// Count of modified keys.
    pub fn modified_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Modified)
            .count()
    }

    /// Total bytes changed (sum of new values).
    pub fn bytes_changed(&self) -> usize {
        self.changes
            .iter()
            .map(|c| {
                c.new_value.as_ref().map_or(0, |v| v.len())
                    + c.old_value.as_ref().map_or(0, |v| v.len())
            })
            .sum()
    }
}

/// Apply a diff to a base map, producing the target state.
pub fn apply_diff(
    base: &HashMap<String, Vec<u8>>,
    diff: &SnapshotDiff,
) -> HashMap<String, Vec<u8>> {
    let mut result = base.clone();
    for change in &diff.changes {
        match change.change_type {
            ChangeType::Added | ChangeType::Modified => {
                if let Some(val) = &change.new_value {
                    result.insert(change.key.clone(), val.clone());
                }
            }
            ChangeType::Removed => {
                result.remove(&change.key);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(entries: &[(&str, &[u8])]) -> HashMap<String, Vec<u8>> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_vec()))
            .collect()
    }

    #[test]
    fn test_no_changes() {
        let m = make_map(&[("a", b"1"), ("b", b"2")]);
        let diff = SnapshotDiff::compute(1, &m, 2, &m);
        assert!(diff.is_empty());
        assert_eq!(diff.change_count(), 0);
    }

    #[test]
    fn test_added_keys() {
        let from = make_map(&[("a", b"1")]);
        let to = make_map(&[("a", b"1"), ("b", b"2")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        assert_eq!(diff.added_count(), 1);
        assert_eq!(diff.removed_count(), 0);
        assert_eq!(diff.modified_count(), 0);
    }

    #[test]
    fn test_removed_keys() {
        let from = make_map(&[("a", b"1"), ("b", b"2")]);
        let to = make_map(&[("a", b"1")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        assert_eq!(diff.removed_count(), 1);
        assert_eq!(diff.changes()[0].key, "b");
    }

    #[test]
    fn test_modified_keys() {
        let from = make_map(&[("a", b"1")]);
        let to = make_map(&[("a", b"2")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        assert_eq!(diff.modified_count(), 1);
        let change = &diff.changes()[0];
        assert_eq!(change.old_value.as_deref(), Some(b"1".as_slice()));
        assert_eq!(change.new_value.as_deref(), Some(b"2".as_slice()));
    }

    #[test]
    fn test_mixed_changes() {
        let from = make_map(&[("a", b"1"), ("b", b"2"), ("c", b"3")]);
        let to = make_map(&[("a", b"1"), ("b", b"X"), ("d", b"4")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        assert_eq!(diff.added_count(), 1); // d added
        assert_eq!(diff.removed_count(), 1); // c removed
        assert_eq!(diff.modified_count(), 1); // b modified
        assert_eq!(diff.change_count(), 3);
    }

    #[test]
    fn test_apply_diff() {
        let from = make_map(&[("a", b"1"), ("b", b"2"), ("c", b"3")]);
        let to = make_map(&[("a", b"1"), ("b", b"X"), ("d", b"4")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        let result = apply_diff(&from, &diff);
        assert_eq!(result.get("a").unwrap().as_slice(), b"1");
        assert_eq!(result.get("b").unwrap().as_slice(), b"X");
        assert!(!result.contains_key("c"));
        assert_eq!(result.get("d").unwrap().as_slice(), b"4");
    }

    #[test]
    fn test_change_type_display() {
        assert_eq!(ChangeType::Added.as_str(), "added");
        assert_eq!(ChangeType::Removed.as_str(), "removed");
        assert_eq!(ChangeType::Modified.as_str(), "modified");
    }

    #[test]
    fn test_bytes_changed() {
        let from = make_map(&[("a", b"12345")]);
        let to = make_map(&[("a", b"XY")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        assert_eq!(diff.bytes_changed(), 7); // 5 old + 2 new
    }

    #[test]
    fn test_sorted_output() {
        let from = make_map(&[]);
        let to = make_map(&[("z", b"1"), ("a", b"2"), ("m", b"3")]);
        let diff = SnapshotDiff::compute(1, &from, 2, &to);

        let keys: Vec<_> = diff.changes().iter().map(|c| c.key.as_str()).collect();
        assert_eq!(keys, vec!["a", "m", "z"]);
    }
}
