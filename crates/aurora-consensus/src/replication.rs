//! State replication — log-based replication with commit semantics.

use std::collections::HashMap;

/// A log entry for replication.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Log index (1-based, monotonically increasing).
    pub index: u64,
    /// Term in which the entry was created.
    pub term: u64,
    /// Command/data to replicate.
    pub data: Vec<u8>,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
}

/// Replication status for a follower.
#[derive(Debug, Clone)]
pub struct FollowerStatus {
    /// Follower ID.
    pub id: u64,
    /// Last replicated index.
    pub match_index: u64,
    /// Next index to send.
    pub next_index: u64,
    /// Whether the follower is caught up.
    pub caught_up: bool,
}

/// Replicated log for consensus.
pub struct ReplicatedLog {
    /// Log entries.
    entries: Vec<LogEntry>,
    /// Commit index (highest index known to be committed).
    commit_index: u64,
    /// Current term.
    current_term: u64,
    /// Follower statuses (by follower ID).
    followers: HashMap<u64, FollowerStatus>,
    /// Required quorum size (including leader).
    quorum_size: usize,
}

impl ReplicatedLog {
    /// Create a new replicated log.
    pub fn new(quorum_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            commit_index: 0,
            current_term: 1,
            followers: HashMap::new(),
            quorum_size,
        }
    }

    /// Append a new entry (as leader). Returns the log index.
    pub fn append(&mut self, data: Vec<u8>, timestamp_ms: u64) -> u64 {
        let index = self.entries.len() as u64 + 1;
        self.entries.push(LogEntry {
            index,
            term: self.current_term,
            data,
            timestamp_ms,
        });
        index
    }

    /// Record that a follower has replicated up to a given index.
    pub fn ack_replication(&mut self, follower_id: u64, match_index: u64) {
        let last = self.last_index();
        let status = self.followers.entry(follower_id).or_insert(FollowerStatus {
            id: follower_id,
            match_index: 0,
            next_index: 1,
            caught_up: false,
        });
        status.match_index = match_index;
        status.next_index = match_index + 1;
        status.caught_up = match_index == last;

        self.try_advance_commit();
    }

    /// Try to advance the commit index based on quorum.
    fn try_advance_commit(&mut self) {
        let last = self.last_index();
        for candidate_index in (self.commit_index + 1)..=last {
            // Count replicas that have this index (leader counts as 1).
            let replica_count = 1 + self
                .followers
                .values()
                .filter(|f| f.match_index >= candidate_index)
                .count();
            if replica_count >= self.quorum_size {
                self.commit_index = candidate_index;
            } else {
                break;
            }
        }
    }

    /// Get entries that need to be sent to a follower.
    pub fn entries_for_follower(&self, follower_id: u64) -> Vec<&LogEntry> {
        let next = self
            .followers
            .get(&follower_id)
            .map(|f| f.next_index)
            .unwrap_or(1);
        self.entries.iter().filter(|e| e.index >= next).collect()
    }

    /// Get a log entry by index.
    pub fn get_entry(&self, index: u64) -> Option<&LogEntry> {
        if index == 0 || index > self.entries.len() as u64 {
            return None;
        }
        Some(&self.entries[(index - 1) as usize])
    }

    /// Last log index (0 if empty).
    pub fn last_index(&self) -> u64 {
        self.entries.len() as u64
    }

    /// Current commit index.
    pub fn commit_index(&self) -> u64 {
        self.commit_index
    }

    /// Current term.
    pub fn current_term(&self) -> u64 {
        self.current_term
    }

    /// Advance to the next term.
    pub fn advance_term(&mut self) -> u64 {
        self.current_term += 1;
        self.current_term
    }

    /// Number of log entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of tracked followers.
    pub fn follower_count(&self) -> usize {
        self.followers.len()
    }

    /// Add a follower.
    pub fn add_follower(&mut self, id: u64) {
        self.followers.insert(
            id,
            FollowerStatus {
                id,
                match_index: 0,
                next_index: self.last_index() + 1,
                caught_up: false,
            },
        );
    }

    /// Check if all followers are caught up.
    pub fn all_caught_up(&self) -> bool {
        self.followers.values().all(|f| f.caught_up)
    }

    /// Get committed entries.
    pub fn committed_entries(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.index <= self.commit_index)
            .collect()
    }

    /// Get uncommitted entries.
    pub fn uncommitted_entries(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.index > self.commit_index)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_entries() {
        let mut log = ReplicatedLog::new(2);
        let idx1 = log.append(b"cmd1".to_vec(), 1000);
        let idx2 = log.append(b"cmd2".to_vec(), 2000);
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn test_commit_with_quorum() {
        let mut log = ReplicatedLog::new(2); // leader + 1 follower = quorum of 2
        log.add_follower(2);
        log.append(b"cmd1".to_vec(), 1000);
        log.append(b"cmd2".to_vec(), 2000);

        assert_eq!(log.commit_index(), 0); // Not yet committed

        log.ack_replication(2, 1);
        assert_eq!(log.commit_index(), 1); // Index 1 committed

        log.ack_replication(2, 2);
        assert_eq!(log.commit_index(), 2); // Index 2 committed
    }

    #[test]
    fn test_commit_requires_quorum() {
        let mut log = ReplicatedLog::new(3); // need 3 for quorum
        log.add_follower(2);
        log.add_follower(3);
        log.append(b"cmd1".to_vec(), 1000);

        log.ack_replication(2, 1);
        assert_eq!(log.commit_index(), 0); // 2 of 3, not enough

        log.ack_replication(3, 1);
        assert_eq!(log.commit_index(), 1); // 3 of 3, committed
    }

    #[test]
    fn test_entries_for_follower() {
        let mut log = ReplicatedLog::new(2);
        log.add_follower(2);
        log.append(b"cmd1".to_vec(), 1000);
        log.append(b"cmd2".to_vec(), 2000);

        let entries = log.entries_for_follower(2);
        assert_eq!(entries.len(), 2);

        log.ack_replication(2, 1);
        let entries = log.entries_for_follower(2);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].index, 2);
    }

    #[test]
    fn test_get_entry() {
        let mut log = ReplicatedLog::new(2);
        log.append(b"cmd1".to_vec(), 1000);
        assert!(log.get_entry(1).is_some());
        assert_eq!(log.get_entry(1).unwrap().data, b"cmd1");
        assert!(log.get_entry(0).is_none());
        assert!(log.get_entry(2).is_none());
    }

    #[test]
    fn test_term_advance() {
        let mut log = ReplicatedLog::new(2);
        assert_eq!(log.current_term(), 1);
        log.advance_term();
        assert_eq!(log.current_term(), 2);

        log.append(b"cmd".to_vec(), 1000);
        assert_eq!(log.get_entry(1).unwrap().term, 2);
    }

    #[test]
    fn test_committed_vs_uncommitted() {
        let mut log = ReplicatedLog::new(2);
        log.add_follower(2);
        log.append(b"c1".to_vec(), 1000);
        log.append(b"c2".to_vec(), 2000);

        log.ack_replication(2, 1);
        assert_eq!(log.committed_entries().len(), 1);
        assert_eq!(log.uncommitted_entries().len(), 1);
    }

    #[test]
    fn test_caught_up() {
        let mut log = ReplicatedLog::new(2);
        log.add_follower(2);
        log.append(b"c1".to_vec(), 1000);

        assert!(!log.all_caught_up());
        log.ack_replication(2, 1);
        assert!(log.all_caught_up());
    }

    #[test]
    fn test_empty_log() {
        let log = ReplicatedLog::new(2);
        assert!(log.is_empty());
        assert_eq!(log.last_index(), 0);
        assert_eq!(log.commit_index(), 0);
    }
}
