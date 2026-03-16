//! Geofencing — define geographic regions and detect entry/exit events.

use std::collections::{HashMap, HashSet};

/// Unique geofence identifier.
pub type FenceId = u64;

/// Geofence shape.
#[derive(Debug, Clone)]
pub enum FenceShape {
    /// Circular geofence (center lat, center lon, radius in meters).
    Circle {
        center_lat: f64,
        center_lon: f64,
        radius_m: f64,
    },
    /// Rectangular geofence (min_lat, min_lon, max_lat, max_lon).
    Rectangle {
        min_lat: f64,
        min_lon: f64,
        max_lat: f64,
        max_lon: f64,
    },
}

/// A geofence definition.
#[derive(Debug, Clone)]
pub struct Geofence {
    /// Unique ID.
    pub id: FenceId,
    /// Human-readable name.
    pub name: String,
    /// Shape of the geofence.
    pub shape: FenceShape,
    /// Whether the fence is active.
    pub active: bool,
}

impl Geofence {
    /// Create a circular geofence.
    pub fn circle(id: FenceId, name: String, lat: f64, lon: f64, radius_m: f64) -> Self {
        Self {
            id,
            name,
            shape: FenceShape::Circle {
                center_lat: lat,
                center_lon: lon,
                radius_m,
            },
            active: true,
        }
    }

    /// Create a rectangular geofence.
    pub fn rectangle(
        id: FenceId,
        name: String,
        min_lat: f64,
        min_lon: f64,
        max_lat: f64,
        max_lon: f64,
    ) -> Self {
        Self {
            id,
            name,
            shape: FenceShape::Rectangle {
                min_lat,
                min_lon,
                max_lat,
                max_lon,
            },
            active: true,
        }
    }

    /// Check if a point is inside this geofence.
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        if !self.active {
            return false;
        }
        match &self.shape {
            FenceShape::Circle {
                center_lat,
                center_lon,
                radius_m,
            } => {
                let dist = haversine_meters(lat, lon, *center_lat, *center_lon);
                dist <= *radius_m
            }
            FenceShape::Rectangle {
                min_lat,
                min_lon,
                max_lat,
                max_lon,
            } => lat >= *min_lat && lat <= *max_lat && lon >= *min_lon && lon <= *max_lon,
        }
    }
}

/// Simple haversine distance in meters.
fn haversine_meters(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6_378_137.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let lat1r = lat1.to_radians();
    let lat2r = lat2.to_radians();
    let a = (dlat / 2.0).sin().powi(2) + lat1r.cos() * lat2r.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

/// Geofence transition event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceEvent {
    /// Entered a geofence.
    Enter(FenceId),
    /// Exited a geofence.
    Exit(FenceId),
}

/// Geofence manager — tracks fences and detects transitions.
pub struct GeofenceManager {
    fences: HashMap<FenceId, Geofence>,
    /// Current set of fences the tracked entity is inside.
    inside: HashSet<FenceId>,
    next_id: FenceId,
}

impl GeofenceManager {
    /// Create a new geofence manager.
    pub fn new() -> Self {
        Self {
            fences: HashMap::new(),
            inside: HashSet::new(),
            next_id: 1,
        }
    }

    /// Add a geofence. Returns the assigned ID.
    pub fn add_fence(&mut self, fence: Geofence) -> FenceId {
        let id = fence.id;
        self.fences.insert(id, fence);
        if id >= self.next_id {
            self.next_id = id + 1;
        }
        id
    }

    /// Add a circular geofence. Returns the assigned ID.
    pub fn add_circle(&mut self, name: String, lat: f64, lon: f64, radius_m: f64) -> FenceId {
        let id = self.next_id;
        self.next_id += 1;
        let fence = Geofence::circle(id, name, lat, lon, radius_m);
        self.fences.insert(id, fence);
        id
    }

    /// Add a rectangular geofence. Returns the assigned ID.
    pub fn add_rectangle(
        &mut self,
        name: String,
        min_lat: f64,
        min_lon: f64,
        max_lat: f64,
        max_lon: f64,
    ) -> FenceId {
        let id = self.next_id;
        self.next_id += 1;
        let fence = Geofence::rectangle(id, name, min_lat, min_lon, max_lat, max_lon);
        self.fences.insert(id, fence);
        id
    }

    /// Remove a geofence.
    pub fn remove_fence(&mut self, id: FenceId) -> bool {
        self.inside.remove(&id);
        self.fences.remove(&id).is_some()
    }

    /// Enable/disable a geofence.
    pub fn set_active(&mut self, id: FenceId, active: bool) {
        if let Some(fence) = self.fences.get_mut(&id) {
            fence.active = active;
            if !active {
                self.inside.remove(&id);
            }
        }
    }

    /// Update position and detect fence transitions.
    /// Returns a list of enter/exit events.
    pub fn update(&mut self, lat: f64, lon: f64) -> Vec<FenceEvent> {
        let mut events = Vec::new();

        for (&id, fence) in &self.fences {
            let is_inside = fence.contains(lat, lon);
            let was_inside = self.inside.contains(&id);

            if is_inside && !was_inside {
                events.push(FenceEvent::Enter(id));
            } else if !is_inside && was_inside {
                events.push(FenceEvent::Exit(id));
            }
        }

        // Update inside set
        for event in &events {
            match event {
                FenceEvent::Enter(id) => {
                    self.inside.insert(*id);
                }
                FenceEvent::Exit(id) => {
                    self.inside.remove(id);
                }
            }
        }

        events
    }

    /// Get the set of fences the entity is currently inside.
    pub fn current_fences(&self) -> Vec<FenceId> {
        self.inside.iter().copied().collect()
    }

    /// Number of registered fences.
    pub fn fence_count(&self) -> usize {
        self.fences.len()
    }

    /// Get a fence by ID.
    pub fn get_fence(&self, id: FenceId) -> Option<&Geofence> {
        self.fences.get(&id)
    }

    /// Check which fences contain a given point.
    pub fn fences_containing(&self, lat: f64, lon: f64) -> Vec<FenceId> {
        self.fences
            .iter()
            .filter(|(_, f)| f.contains(lat, lon))
            .map(|(&id, _)| id)
            .collect()
    }
}

impl Default for GeofenceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_contains() {
        let fence = Geofence::circle(1, "office".into(), 40.7128, -74.0060, 500.0);
        assert!(fence.contains(40.7128, -74.0060)); // center
        assert!(!fence.contains(41.0, -74.0)); // far away
    }

    #[test]
    fn test_rectangle_contains() {
        let fence = Geofence::rectangle(1, "zone".into(), 40.0, -75.0, 41.0, -74.0);
        assert!(fence.contains(40.5, -74.5)); // inside
        assert!(!fence.contains(42.0, -74.5)); // outside
    }

    #[test]
    fn test_inactive_fence() {
        let mut fence = Geofence::circle(1, "test".into(), 0.0, 0.0, 1000.0);
        assert!(fence.contains(0.0, 0.0));
        fence.active = false;
        assert!(!fence.contains(0.0, 0.0));
    }

    #[test]
    fn test_enter_exit_events() {
        let mut mgr = GeofenceManager::new();
        let id = mgr.add_circle("park".into(), 40.7829, -73.9654, 500.0); // Central Park

        // Enter
        let events = mgr.update(40.7829, -73.9654);
        assert_eq!(events, vec![FenceEvent::Enter(id)]);
        assert_eq!(mgr.current_fences(), vec![id]);

        // Stay inside (no events)
        let events = mgr.update(40.7830, -73.9655);
        assert!(events.is_empty());

        // Exit
        let events = mgr.update(41.0, -74.0);
        assert_eq!(events, vec![FenceEvent::Exit(id)]);
        assert!(mgr.current_fences().is_empty());
    }

    #[test]
    fn test_multiple_fences() {
        let mut mgr = GeofenceManager::new();
        let id1 = mgr.add_circle("a".into(), 0.0, 0.0, 1_000_000.0);
        let id2 = mgr.add_circle("b".into(), 0.0, 0.0, 500.0);

        let events = mgr.update(0.0, 0.0);
        assert_eq!(events.len(), 2); // enter both
        assert!(events.contains(&FenceEvent::Enter(id1)));
        assert!(events.contains(&FenceEvent::Enter(id2)));
    }

    #[test]
    fn test_remove_fence() {
        let mut mgr = GeofenceManager::new();
        let id = mgr.add_circle("temp".into(), 0.0, 0.0, 1000.0);
        mgr.update(0.0, 0.0); // enter
        assert!(mgr.remove_fence(id));
        assert_eq!(mgr.fence_count(), 0);
        assert!(mgr.current_fences().is_empty());
    }

    #[test]
    fn test_set_active() {
        let mut mgr = GeofenceManager::new();
        let id = mgr.add_circle("zone".into(), 0.0, 0.0, 1000.0);
        mgr.update(0.0, 0.0); // enter

        mgr.set_active(id, false);
        assert!(mgr.current_fences().is_empty());
    }

    #[test]
    fn test_fences_containing() {
        let mut mgr = GeofenceManager::new();
        mgr.add_rectangle("r".into(), -1.0, -1.0, 1.0, 1.0);
        mgr.add_circle("c".into(), 0.0, 0.0, 100_000.0);

        let fences = mgr.fences_containing(0.0, 0.0);
        assert_eq!(fences.len(), 2);
    }
}
