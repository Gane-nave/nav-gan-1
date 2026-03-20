//! Event sourcing — append-only event store with snapshots.

use std::collections::HashMap;

/// An event in the event store.
#[derive(Debug, Clone)]
pub struct Event {
    /// Global sequence number.
    pub sequence: u64,
    /// Aggregate ID this event belongs to.
    pub aggregate_id: String,
    /// Event type name.
    pub event_type: String,
    /// Serialized event data.
    pub data: Vec<u8>,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
    /// Version of the aggregate after this event.
    pub version: u64,
}

/// A snapshot of aggregate state.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Aggregate ID.
    pub aggregate_id: String,
    /// Serialized state.
    pub state: Vec<u8>,
    /// Version at snapshot time.
    pub version: u64,
    /// Timestamp.
    pub timestamp_ms: u64,
}

/// Append-only event store.
pub struct EventStore {
    /// All events in order.
    events: Vec<Event>,
    /// Events indexed by aggregate ID.
    by_aggregate: HashMap<String, Vec<usize>>,
    /// Snapshots by aggregate ID.
    snapshots: HashMap<String, Snapshot>,
    /// Global sequence counter.
    next_sequence: u64,
}

impl EventStore {
    /// Create a new event store.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            by_aggregate: HashMap::new(),
            snapshots: HashMap::new(),
            next_sequence: 1,
        }
    }

    /// Append an event. Returns the global sequence number.
    pub fn append(
        &mut self,
        aggregate_id: &str,
        event_type: &str,
        data: Vec<u8>,
        timestamp_ms: u64,
    ) -> u64 {
        let seq = self.next_sequence;
        self.next_sequence += 1;

        let agg_events = self
            .by_aggregate
            .entry(aggregate_id.to_string())
            .or_default();
        let version = agg_events.len() as u64 + 1;

        let event = Event {
            sequence: seq,
            aggregate_id: aggregate_id.to_string(),
            event_type: event_type.to_string(),
            data,
            timestamp_ms,
            version,
        };

        let idx = self.events.len();
        self.events.push(event);
        agg_events.push(idx);

        seq
    }

    /// Get all events for an aggregate (after optional version).
    pub fn get_events(&self, aggregate_id: &str, after_version: u64) -> Vec<&Event> {
        self.by_aggregate
            .get(aggregate_id)
            .map(|indices| {
                indices
                    .iter()
                    .map(|&i| &self.events[i])
                    .filter(|e| e.version > after_version)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get the current version of an aggregate.
    pub fn aggregate_version(&self, aggregate_id: &str) -> u64 {
        self.by_aggregate
            .get(aggregate_id)
            .map(|indices| indices.len() as u64)
            .unwrap_or(0)
    }

    /// Save a snapshot.
    pub fn save_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots
            .insert(snapshot.aggregate_id.clone(), snapshot);
    }

    /// Get the latest snapshot for an aggregate.
    pub fn get_snapshot(&self, aggregate_id: &str) -> Option<&Snapshot> {
        self.snapshots.get(aggregate_id)
    }

    /// Total number of events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Number of distinct aggregates.
    pub fn aggregate_count(&self) -> usize {
        self.by_aggregate.len()
    }

    /// Number of snapshots.
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Get events by global sequence range.
    pub fn get_events_by_range(&self, from_seq: u64, to_seq: u64) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| (from_seq..=to_seq).contains(&e.sequence))
            .collect()
    }

    /// Get all events (global order).
    pub fn all_events(&self) -> &[Event] {
        &self.events
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_get_events() {
        let mut store = EventStore::new();
        store.append("vehicle-1", "PositionUpdated", b"pos1".to_vec(), 1000);
        store.append("vehicle-1", "SpeedChanged", b"spd1".to_vec(), 2000);
        store.append("vehicle-2", "PositionUpdated", b"pos2".to_vec(), 3000);

        let events = store.get_events("vehicle-1", 0);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "PositionUpdated");
        assert_eq!(events[1].event_type, "SpeedChanged");
    }

    #[test]
    fn test_versioning() {
        let mut store = EventStore::new();
        store.append("agg-1", "Created", b"c".to_vec(), 1000);
        store.append("agg-1", "Updated", b"u".to_vec(), 2000);
        assert_eq!(store.aggregate_version("agg-1"), 2);

        let events = store.get_events("agg-1", 1); // after version 1
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "Updated");
    }

    #[test]
    fn test_snapshots() {
        let mut store = EventStore::new();
        store.append("agg-1", "e1", b"d1".to_vec(), 1000);
        store.append("agg-1", "e2", b"d2".to_vec(), 2000);

        store.save_snapshot(Snapshot {
            aggregate_id: "agg-1".to_string(),
            state: b"snapshot_state".to_vec(),
            version: 2,
            timestamp_ms: 2000,
        });

        let snap = store.get_snapshot("agg-1").unwrap();
        assert_eq!(snap.version, 2);
        assert_eq!(snap.state, b"snapshot_state");
    }

    #[test]
    fn test_global_sequence() {
        let mut store = EventStore::new();
        let s1 = store.append("a", "e", b"".to_vec(), 1000);
        let s2 = store.append("b", "e", b"".to_vec(), 2000);
        let s3 = store.append("a", "e", b"".to_vec(), 3000);
        assert_eq!(s1, 1);
        assert_eq!(s2, 2);
        assert_eq!(s3, 3);
    }

    #[test]
    fn test_events_by_range() {
        let mut store = EventStore::new();
        store.append("a", "e1", b"".to_vec(), 1000);
        store.append("a", "e2", b"".to_vec(), 2000);
        store.append("a", "e3", b"".to_vec(), 3000);

        let range = store.get_events_by_range(2, 3);
        assert_eq!(range.len(), 2);
        assert_eq!(range[0].sequence, 2);
        assert_eq!(range[1].sequence, 3);
    }

    #[test]
    fn test_counters() {
        let mut store = EventStore::new();
        store.append("a", "e", b"".to_vec(), 1000);
        store.append("b", "e", b"".to_vec(), 2000);
        assert_eq!(store.event_count(), 2);
        assert_eq!(store.aggregate_count(), 2);
    }

    #[test]
    fn test_empty_aggregate() {
        let store = EventStore::new();
        assert_eq!(store.aggregate_version("nonexistent"), 0);
        assert!(store.get_events("nonexistent", 0).is_empty());
        assert!(store.get_snapshot("nonexistent").is_none());
    }
}
