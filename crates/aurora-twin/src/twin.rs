//! Digital twin — state mirroring of physical entities with real-time
//! synchronization, snapshot capture, and drift detection.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Twin types
// ---------------------------------------------------------------------------

/// Kind of entity being twinned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TwinKind {
    Vehicle,
    Intersection,
    RoadSegment,
    TrafficSignal,
    ParkingFacility,
    ChargingStation,
    Sensor,
    Fleet,
}

/// Synchronization state of a twin relative to its physical counterpart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncState {
    /// Twin matches physical state within tolerance.
    InSync,
    /// Twin is slightly behind physical state.
    Lagging,
    /// Twin has drifted beyond acceptable tolerance.
    Drifted,
    /// Physical entity is unreachable — twin is stale.
    Disconnected,
}

/// A single state property of a twin (key-value with timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinProperty {
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: DateTime<Utc>,
    pub source: String,
}

/// A point-in-time snapshot of a twin's full state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinSnapshot {
    pub id: EntityId,
    pub twin_id: EntityId,
    pub properties: HashMap<String, TwinProperty>,
    pub captured_at: DateTime<Utc>,
    pub label: Option<String>,
}

/// A digital twin entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    pub id: EntityId,
    pub physical_id: EntityId,
    pub kind: TwinKind,
    pub name: String,
    pub properties: HashMap<String, TwinProperty>,
    pub sync_state: SyncState,
    pub created_at: DateTime<Utc>,
    pub last_sync_at: Option<DateTime<Utc>>,
}

/// Drift detection result.
#[derive(Debug, Clone)]
pub struct DriftReport {
    pub twin_id: EntityId,
    pub drifted_properties: Vec<String>,
    pub max_age_s: f64,
    pub detected_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Twin registry
// ---------------------------------------------------------------------------

/// Configuration for the twin registry.
#[derive(Debug, Clone)]
pub struct TwinRegistryConfig {
    /// Maximum age (seconds) before a property is considered stale.
    pub staleness_threshold_s: f64,
    /// Maximum snapshots to retain per twin.
    pub max_snapshots_per_twin: usize,
    /// Drift detection threshold (seconds without update).
    pub drift_threshold_s: f64,
}

impl Default for TwinRegistryConfig {
    fn default() -> Self {
        Self {
            staleness_threshold_s: 120.0,
            max_snapshots_per_twin: 50,
            drift_threshold_s: 300.0,
        }
    }
}

/// The twin registry manages the lifecycle and state of all digital twins.
pub struct TwinRegistry {
    config: TwinRegistryConfig,
    twins: HashMap<EntityId, DigitalTwin>,
    /// Snapshots keyed by twin ID.
    snapshots: HashMap<EntityId, Vec<TwinSnapshot>>,
    /// Map from physical entity ID to twin ID.
    physical_to_twin: HashMap<EntityId, EntityId>,
}

impl TwinRegistry {
    pub fn new() -> Self {
        Self {
            config: TwinRegistryConfig::default(),
            twins: HashMap::new(),
            snapshots: HashMap::new(),
            physical_to_twin: HashMap::new(),
        }
    }

    pub fn with_config(config: TwinRegistryConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -----------------------------------------------------------------------
    // Registration
    // -----------------------------------------------------------------------

    /// Create a digital twin for a physical entity.
    pub fn create_twin(
        &mut self,
        physical_id: EntityId,
        kind: TwinKind,
        name: impl Into<String>,
    ) -> EntityId {
        let twin_id = EntityId::new();
        let twin = DigitalTwin {
            id: twin_id,
            physical_id,
            kind,
            name: name.into(),
            properties: HashMap::new(),
            sync_state: SyncState::Disconnected,
            created_at: Utc::now(),
            last_sync_at: None,
        };
        info!(twin = %twin_id, physical = %physical_id, kind = ?kind, "digital twin created");
        self.twins.insert(twin_id, twin);
        self.physical_to_twin.insert(physical_id, twin_id);
        twin_id
    }

    /// Remove a digital twin.
    pub fn remove_twin(&mut self, twin_id: &EntityId) -> bool {
        if let Some(twin) = self.twins.remove(twin_id) {
            self.physical_to_twin.remove(&twin.physical_id);
            self.snapshots.remove(twin_id);
            info!(twin = %twin_id, "digital twin removed");
            return true;
        }
        false
    }

    /// Get a twin by its ID.
    pub fn get_twin(&self, twin_id: &EntityId) -> Option<&DigitalTwin> {
        self.twins.get(twin_id)
    }

    /// Get a twin by its physical entity ID.
    pub fn get_twin_for_physical(&self, physical_id: &EntityId) -> Option<&DigitalTwin> {
        let twin_id = self.physical_to_twin.get(physical_id)?;
        self.twins.get(twin_id)
    }

    /// Number of registered twins.
    pub fn twin_count(&self) -> usize {
        self.twins.len()
    }

    /// Get all twins of a specific kind.
    pub fn twins_by_kind(&self, kind: TwinKind) -> Vec<&DigitalTwin> {
        self.twins.values().filter(|t| t.kind == kind).collect()
    }

    // -----------------------------------------------------------------------
    // State synchronization
    // -----------------------------------------------------------------------

    /// Update a property on a twin. This simulates receiving state from the
    /// physical entity.
    pub fn update_property(
        &mut self,
        twin_id: &EntityId,
        key: impl Into<String>,
        value: serde_json::Value,
        source: impl Into<String>,
    ) -> bool {
        let Some(twin) = self.twins.get_mut(twin_id) else {
            warn!(twin = %twin_id, "update_property for unknown twin");
            return false;
        };

        let now = Utc::now();
        let key = key.into();
        twin.properties.insert(
            key.clone(),
            TwinProperty {
                key,
                value,
                updated_at: now,
                source: source.into(),
            },
        );
        twin.last_sync_at = Some(now);
        twin.sync_state = SyncState::InSync;
        true
    }

    /// Batch-update multiple properties at once.
    pub fn sync_properties(
        &mut self,
        twin_id: &EntityId,
        updates: Vec<(String, serde_json::Value)>,
        source: impl Into<String>,
    ) -> usize {
        let source = source.into();
        let now = Utc::now();
        let Some(twin) = self.twins.get_mut(twin_id) else {
            return 0;
        };

        let mut count = 0;
        for (key, value) in updates {
            twin.properties.insert(
                key.clone(),
                TwinProperty {
                    key,
                    value,
                    updated_at: now,
                    source: source.clone(),
                },
            );
            count += 1;
        }

        if count > 0 {
            twin.last_sync_at = Some(now);
            twin.sync_state = SyncState::InSync;
            debug!(twin = %twin_id, count, "properties synced");
        }
        count
    }

    /// Get a property value from a twin.
    pub fn get_property(&self, twin_id: &EntityId, key: &str) -> Option<&TwinProperty> {
        self.twins.get(twin_id)?.properties.get(key)
    }

    // -----------------------------------------------------------------------
    // Drift detection
    // -----------------------------------------------------------------------

    /// Check all twins for drift (stale properties).
    pub fn detect_drift(&mut self) -> Vec<DriftReport> {
        let now = Utc::now();
        let threshold = self.config.drift_threshold_s;
        let staleness = self.config.staleness_threshold_s;
        let mut reports = Vec::new();

        let twin_ids: Vec<EntityId> = self.twins.keys().copied().collect();

        for twin_id in twin_ids {
            let twin = self.twins.get(&twin_id).unwrap();
            let mut drifted_props = Vec::new();
            let mut max_age = 0.0_f64;

            for (key, prop) in &twin.properties {
                let age = (now - prop.updated_at).num_seconds() as f64;
                if age > staleness {
                    drifted_props.push(key.clone());
                    max_age = max_age.max(age);
                }
            }

            // Check overall twin sync.
            let overall_age = twin
                .last_sync_at
                .map(|t| (now - t).num_seconds() as f64)
                .unwrap_or(f64::MAX);

            if overall_age > threshold || !drifted_props.is_empty() {
                let twin_mut = self.twins.get_mut(&twin_id).unwrap();
                if overall_age > threshold {
                    twin_mut.sync_state = SyncState::Drifted;
                } else if !drifted_props.is_empty() {
                    twin_mut.sync_state = SyncState::Lagging;
                }

                reports.push(DriftReport {
                    twin_id,
                    drifted_properties: drifted_props,
                    max_age_s: max_age.max(overall_age),
                    detected_at: now,
                });
            }
        }

        reports
    }

    /// Get all twins in a specific sync state.
    pub fn twins_in_state(&self, state: SyncState) -> Vec<&DigitalTwin> {
        self.twins
            .values()
            .filter(|t| t.sync_state == state)
            .collect()
    }

    // -----------------------------------------------------------------------
    // Snapshots
    // -----------------------------------------------------------------------

    /// Capture a snapshot of a twin's current state.
    pub fn capture_snapshot(
        &mut self,
        twin_id: &EntityId,
        label: Option<String>,
    ) -> Option<EntityId> {
        let twin = self.twins.get(twin_id)?;

        let snapshot = TwinSnapshot {
            id: EntityId::new(),
            twin_id: *twin_id,
            properties: twin.properties.clone(),
            captured_at: Utc::now(),
            label,
        };
        let snap_id = snapshot.id;

        let snaps = self.snapshots.entry(*twin_id).or_default();
        if snaps.len() >= self.config.max_snapshots_per_twin {
            snaps.remove(0);
        }
        snaps.push(snapshot);

        debug!(twin = %twin_id, snapshot = %snap_id, "snapshot captured");
        Some(snap_id)
    }

    /// Get all snapshots for a twin.
    pub fn get_snapshots(&self, twin_id: &EntityId) -> &[TwinSnapshot] {
        self.snapshots
            .get(twin_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Compare two snapshots and return the keys that differ.
    pub fn diff_snapshots(&self, snap_a: &EntityId, snap_b: &EntityId) -> Option<Vec<String>> {
        let (a, b) = self.find_snapshots(snap_a, snap_b)?;

        let mut diffs = Vec::new();

        // Keys in A but not in B, or with different values.
        for (key, prop_a) in &a.properties {
            match b.properties.get(key) {
                Some(prop_b) if prop_a.value != prop_b.value => {
                    diffs.push(key.clone());
                }
                None => {
                    diffs.push(key.clone());
                }
                _ => {}
            }
        }

        // Keys in B but not in A.
        for key in b.properties.keys() {
            if !a.properties.contains_key(key) {
                diffs.push(key.clone());
            }
        }

        Some(diffs)
    }

    fn find_snapshots(
        &self,
        snap_a: &EntityId,
        snap_b: &EntityId,
    ) -> Option<(&TwinSnapshot, &TwinSnapshot)> {
        let mut found_a = None;
        let mut found_b = None;

        for snaps in self.snapshots.values() {
            for snap in snaps {
                if snap.id == *snap_a {
                    found_a = Some(snap);
                }
                if snap.id == *snap_b {
                    found_b = Some(snap);
                }
            }
        }

        Some((found_a?, found_b?))
    }

    /// Snapshot count for a twin.
    pub fn snapshot_count(&self, twin_id: &EntityId) -> usize {
        self.snapshots.get(twin_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for TwinRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_twin() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Vehicle, "Car-001");

        assert_eq!(reg.twin_count(), 1);
        let twin = reg.get_twin(&twin_id).unwrap();
        assert_eq!(twin.kind, TwinKind::Vehicle);
        assert_eq!(twin.sync_state, SyncState::Disconnected);
    }

    #[test]
    fn get_twin_for_physical() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Intersection, "Int-42");

        let found = reg.get_twin_for_physical(&phys).unwrap();
        assert_eq!(found.id, twin_id);
    }

    #[test]
    fn remove_twin() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Sensor, "Temp-1");

        assert!(reg.remove_twin(&twin_id));
        assert_eq!(reg.twin_count(), 0);
        assert!(reg.get_twin_for_physical(&phys).is_none());
    }

    #[test]
    fn update_property_syncs() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Vehicle, "Car-002");

        assert!(reg.update_property(&twin_id, "speed_kmh", serde_json::json!(65.0), "gnss"));

        let twin = reg.get_twin(&twin_id).unwrap();
        assert_eq!(twin.sync_state, SyncState::InSync);
        assert!(twin.last_sync_at.is_some());

        let prop = reg.get_property(&twin_id, "speed_kmh").unwrap();
        assert_eq!(prop.value, serde_json::json!(65.0));
    }

    #[test]
    fn batch_sync_properties() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::RoadSegment, "Seg-101");

        let updates = vec![
            ("flow".into(), serde_json::json!(450)),
            ("speed".into(), serde_json::json!(72.5)),
            ("density".into(), serde_json::json!(12)),
        ];
        let count = reg.sync_properties(&twin_id, updates, "traffic_mgmt");
        assert_eq!(count, 3);
    }

    #[test]
    fn twins_by_kind() {
        let mut reg = TwinRegistry::new();
        reg.create_twin(EntityId::new(), TwinKind::Vehicle, "V1");
        reg.create_twin(EntityId::new(), TwinKind::Vehicle, "V2");
        reg.create_twin(EntityId::new(), TwinKind::TrafficSignal, "TS1");

        assert_eq!(reg.twins_by_kind(TwinKind::Vehicle).len(), 2);
        assert_eq!(reg.twins_by_kind(TwinKind::TrafficSignal).len(), 1);
    }

    #[test]
    fn drift_detection_marks_drifted() {
        let config = TwinRegistryConfig {
            drift_threshold_s: 0.0, // Immediate drift
            staleness_threshold_s: 0.0,
            ..Default::default()
        };
        let mut reg = TwinRegistry::with_config(config);

        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Sensor, "S1");

        // Twin was just created with no sync → should be detected as drifted.
        let reports = reg.detect_drift();
        assert!(!reports.is_empty());

        let twin = reg.get_twin(&twin_id).unwrap();
        assert_eq!(twin.sync_state, SyncState::Drifted);
    }

    #[test]
    fn no_drift_when_recently_synced() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Vehicle, "V1");

        reg.update_property(&twin_id, "speed", serde_json::json!(50), "gps");

        let reports = reg.detect_drift();
        // With default 300s threshold and just-synced twin, no drift expected.
        assert!(reports.is_empty());
    }

    #[test]
    fn capture_and_query_snapshot() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Intersection, "Int-1");

        reg.update_property(&twin_id, "queue_length", serde_json::json!(15), "cam");

        let snap_id = reg
            .capture_snapshot(&twin_id, Some("baseline".into()))
            .unwrap();
        assert_eq!(reg.snapshot_count(&twin_id), 1);

        let snaps = reg.get_snapshots(&twin_id);
        assert_eq!(snaps[0].id, snap_id);
        assert_eq!(snaps[0].label, Some("baseline".into()));
    }

    #[test]
    fn snapshot_ring_buffer() {
        let config = TwinRegistryConfig {
            max_snapshots_per_twin: 3,
            ..Default::default()
        };
        let mut reg = TwinRegistry::with_config(config);
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::Vehicle, "V1");

        for i in 0..5 {
            reg.update_property(&twin_id, "idx", serde_json::json!(i), "test");
            reg.capture_snapshot(&twin_id, None);
        }

        assert_eq!(reg.snapshot_count(&twin_id), 3);
    }

    #[test]
    fn diff_snapshots_detects_changes() {
        let mut reg = TwinRegistry::new();
        let phys = EntityId::new();
        let twin_id = reg.create_twin(phys, TwinKind::RoadSegment, "Seg-1");

        reg.update_property(&twin_id, "flow", serde_json::json!(100), "sensor");
        let snap_a = reg.capture_snapshot(&twin_id, None).unwrap();

        reg.update_property(&twin_id, "flow", serde_json::json!(200), "sensor");
        reg.update_property(&twin_id, "density", serde_json::json!(30), "sensor");
        let snap_b = reg.capture_snapshot(&twin_id, None).unwrap();

        let diffs = reg.diff_snapshots(&snap_a, &snap_b).unwrap();
        assert!(diffs.contains(&"flow".to_string()));
        assert!(diffs.contains(&"density".to_string()));
    }

    #[test]
    fn twins_in_state() {
        let mut reg = TwinRegistry::new();
        let t1 = reg.create_twin(EntityId::new(), TwinKind::Vehicle, "V1");
        reg.create_twin(EntityId::new(), TwinKind::Vehicle, "V2");

        reg.update_property(&t1, "speed", serde_json::json!(50), "gps");

        let in_sync = reg.twins_in_state(SyncState::InSync);
        let disconnected = reg.twins_in_state(SyncState::Disconnected);
        assert_eq!(in_sync.len(), 1);
        assert_eq!(disconnected.len(), 1);
    }
}
