//! Indoor navigation handoff — manages transitions between outdoor GNSS
//! navigation and indoor positioning for buildings, parking structures, etc.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// An indoor venue with floor plans and navigation points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorVenue {
    pub id: EntityId,
    pub name: String,
    pub venue_type: VenueType,
    pub entrance_position: GeoPosition,
    pub floors: Vec<Floor>,
}

/// Types of indoor venues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VenueType {
    ShoppingMall,
    Airport,
    Hospital,
    ParkingStructure,
    TrainStation,
    Office,
    Museum,
    Convention,
    University,
    Other,
}

/// A floor within an indoor venue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Floor {
    pub level: i32,
    pub name: String,
    pub nodes: Vec<IndoorNode>,
}

/// A node in the indoor navigation graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorNode {
    pub id: EntityId,
    pub name: String,
    pub node_type: IndoorNodeType,
    pub floor_level: i32,
    /// Position relative to venue origin.
    pub x_m: f64,
    pub y_m: f64,
    /// Connected node IDs with distances.
    pub connections: Vec<(EntityId, f64)>,
}

/// Types of indoor navigation nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndoorNodeType {
    Entrance,
    Exit,
    Elevator,
    Stairs,
    Escalator,
    Corridor,
    Room,
    Gate,
    Platform,
    ParkingSpot,
    Restroom,
    Shop,
    Waypoint,
}

/// The handoff state between outdoor and indoor navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandoffState {
    /// Currently in outdoor navigation mode.
    Outdoor,
    /// Transitioning from outdoor to indoor.
    TransitionIn,
    /// Currently in indoor navigation mode.
    Indoor,
    /// Transitioning from indoor to outdoor.
    TransitionOut,
}

/// Indoor navigation manager — handles venue registration, handoff detection,
/// and indoor pathfinding.
pub struct IndoorManager {
    venues: HashMap<EntityId, IndoorVenue>,
    state: HandoffState,
    active_venue: Option<EntityId>,
    active_floor: Option<i32>,
    handoff_history: Vec<HandoffEvent>,
    /// Proximity threshold for detecting entrance (metres).
    entrance_radius_m: f64,
}

/// Record of a handoff event.
#[derive(Debug, Clone)]
pub struct HandoffEvent {
    pub from: HandoffState,
    pub to: HandoffState,
    pub venue_id: Option<EntityId>,
    pub timestamp: DateTime<Utc>,
}

impl IndoorManager {
    pub fn new() -> Self {
        Self {
            venues: HashMap::new(),
            state: HandoffState::Outdoor,
            active_venue: None,
            active_floor: None,
            handoff_history: Vec::new(),
            entrance_radius_m: 30.0,
        }
    }

    /// Register an indoor venue.
    pub fn register_venue(&mut self, venue: IndoorVenue) {
        debug!(name = %venue.name, venue_type = ?venue.venue_type, "indoor venue registered");
        self.venues.insert(venue.id, venue);
    }

    /// Get a registered venue by ID.
    pub fn venue(&self, id: &EntityId) -> Option<&IndoorVenue> {
        self.venues.get(id)
    }

    /// Get the current handoff state.
    pub fn state(&self) -> HandoffState {
        self.state
    }

    /// Check if the user is near any venue entrance and trigger handoff.
    /// Returns the venue ID if a transition was triggered.
    pub fn check_proximity(&mut self, current_pos: &GeoPosition) -> Option<EntityId> {
        let matched_id = self.venues.values().find_map(|venue| {
            let dist = haversine_approx(current_pos, &venue.entrance_position);
            if dist <= self.entrance_radius_m && self.state == HandoffState::Outdoor {
                Some(venue.id)
            } else {
                None
            }
        });

        if let Some(vid) = matched_id {
            self.transition(HandoffState::TransitionIn, Some(vid));
        }

        matched_id
    }

    /// Confirm entry into a venue (e.g., after GNSS signal loss detected).
    pub fn confirm_indoor(&mut self, venue_id: EntityId, floor: i32) {
        self.active_venue = Some(venue_id);
        self.active_floor = Some(floor);
        self.transition(HandoffState::Indoor, Some(venue_id));
        info!(venue = %venue_id, floor, "indoor navigation confirmed");
    }

    /// Confirm exit from a venue (e.g., after GNSS signal reacquired).
    pub fn confirm_outdoor(&mut self) {
        let venue_id = self.active_venue;
        self.active_venue = None;
        self.active_floor = None;
        self.transition(HandoffState::TransitionOut, venue_id);
        self.transition(HandoffState::Outdoor, None);
        info!("outdoor navigation resumed");
    }

    /// Set the active floor level.
    pub fn set_floor(&mut self, level: i32) {
        self.active_floor = Some(level);
        debug!(floor = level, "floor changed");
    }

    /// Get the active venue ID.
    pub fn active_venue(&self) -> Option<EntityId> {
        self.active_venue
    }

    /// Get the active floor level.
    pub fn active_floor(&self) -> Option<i32> {
        self.active_floor
    }

    /// Find nodes on the current floor of the active venue.
    pub fn current_floor_nodes(&self) -> Vec<&IndoorNode> {
        if let (Some(venue_id), Some(floor)) = (self.active_venue, self.active_floor) {
            if let Some(venue) = self.venues.get(&venue_id) {
                if let Some(floor_data) = venue.floors.iter().find(|f| f.level == floor) {
                    return floor_data.nodes.iter().collect();
                }
            }
        }
        Vec::new()
    }

    /// Find a path between two nodes using simple BFS.
    pub fn find_path(
        &self,
        venue_id: &EntityId,
        start: &EntityId,
        end: &EntityId,
    ) -> Option<Vec<EntityId>> {
        let venue = self.venues.get(venue_id)?;

        // Build adjacency map from all floors.
        let mut adj: HashMap<EntityId, Vec<EntityId>> = HashMap::new();
        for floor in &venue.floors {
            for node in &floor.nodes {
                let entry = adj.entry(node.id).or_default();
                for (neighbor, _) in &node.connections {
                    entry.push(*neighbor);
                }
            }
        }

        // BFS.
        let mut visited: HashMap<EntityId, EntityId> = HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(*start);
        visited.insert(*start, *start);

        while let Some(current) = queue.pop_front() {
            if current == *end {
                // Reconstruct path.
                let mut path = Vec::new();
                let mut node = *end;
                while node != *start {
                    path.push(node);
                    node = *visited.get(&node)?;
                }
                path.push(*start);
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = adj.get(&current) {
                for &neighbor in neighbors {
                    if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(neighbor) {
                        e.insert(current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        None
    }

    /// Get the handoff history.
    pub fn history(&self) -> &[HandoffEvent] {
        &self.handoff_history
    }

    /// Get the number of registered venues.
    pub fn venue_count(&self) -> usize {
        self.venues.len()
    }

    fn transition(&mut self, new_state: HandoffState, venue_id: Option<EntityId>) {
        self.handoff_history.push(HandoffEvent {
            from: self.state,
            to: new_state,
            venue_id,
            timestamp: Utc::now(),
        });
        debug!(from = ?self.state, to = ?new_state, "handoff state transition");
        self.state = new_state;
    }
}

impl Default for IndoorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick approximate distance in metres (good enough for proximity checks).
fn haversine_approx(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let dlat = (b.latitude_deg - a.latitude_deg).to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let a_val = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().atan2((1.0 - a_val).sqrt());
    r * c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(lat: f64, lon: f64) -> GeoPosition {
        GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        }
    }

    fn make_venue(entrance_lat: f64, entrance_lon: f64) -> IndoorVenue {
        let node_a = IndoorNode {
            id: EntityId::new(),
            name: "Entrance".into(),
            node_type: IndoorNodeType::Entrance,
            floor_level: 0,
            x_m: 0.0,
            y_m: 0.0,
            connections: Vec::new(),
        };
        let node_b = IndoorNode {
            id: EntityId::new(),
            name: "Lobby".into(),
            node_type: IndoorNodeType::Corridor,
            floor_level: 0,
            x_m: 10.0,
            y_m: 0.0,
            connections: Vec::new(),
        };

        // Wire up connections.
        let a_id = node_a.id;
        let b_id = node_b.id;
        let mut node_a = node_a;
        let mut node_b = node_b;
        node_a.connections.push((b_id, 10.0));
        node_b.connections.push((a_id, 10.0));

        IndoorVenue {
            id: EntityId::new(),
            name: "Test Mall".into(),
            venue_type: VenueType::ShoppingMall,
            entrance_position: pos(entrance_lat, entrance_lon),
            floors: vec![Floor {
                level: 0,
                name: "Ground".into(),
                nodes: vec![node_a, node_b],
            }],
        }
    }

    #[test]
    fn register_and_query_venue() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);
        assert_eq!(mgr.venue_count(), 1);
        assert!(mgr.venue(&vid).is_some());
    }

    #[test]
    fn initial_state_is_outdoor() {
        let mgr = IndoorManager::new();
        assert_eq!(mgr.state(), HandoffState::Outdoor);
    }

    #[test]
    fn proximity_triggers_transition() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);

        // Very close to entrance.
        let result = mgr.check_proximity(&pos(32.00001, 34.00001));
        assert_eq!(result, Some(vid));
        assert_eq!(mgr.state(), HandoffState::TransitionIn);
    }

    #[test]
    fn far_position_no_transition() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        mgr.register_venue(venue);

        let result = mgr.check_proximity(&pos(33.0, 35.0));
        assert_eq!(result, None);
        assert_eq!(mgr.state(), HandoffState::Outdoor);
    }

    #[test]
    fn indoor_outdoor_lifecycle() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);

        mgr.confirm_indoor(vid, 0);
        assert_eq!(mgr.state(), HandoffState::Indoor);
        assert_eq!(mgr.active_venue(), Some(vid));
        assert_eq!(mgr.active_floor(), Some(0));

        mgr.set_floor(1);
        assert_eq!(mgr.active_floor(), Some(1));

        mgr.confirm_outdoor();
        assert_eq!(mgr.state(), HandoffState::Outdoor);
        assert!(mgr.active_venue().is_none());
    }

    #[test]
    fn current_floor_nodes() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);

        mgr.confirm_indoor(vid, 0);
        let nodes = mgr.current_floor_nodes();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn find_path_between_nodes() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        let node_a_id = venue.floors[0].nodes[0].id;
        let node_b_id = venue.floors[0].nodes[1].id;
        mgr.register_venue(venue);

        let path = mgr.find_path(&vid, &node_a_id, &node_b_id).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], node_a_id);
        assert_eq!(path[1], node_b_id);
    }

    #[test]
    fn find_path_nonexistent_returns_none() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);

        let result = mgr.find_path(&vid, &EntityId::new(), &EntityId::new());
        assert!(result.is_none());
    }

    #[test]
    fn handoff_history_tracked() {
        let mut mgr = IndoorManager::new();
        let venue = make_venue(32.0, 34.0);
        let vid = venue.id;
        mgr.register_venue(venue);

        mgr.confirm_indoor(vid, 0);
        mgr.confirm_outdoor();
        // TransitionIn (confirm_indoor) + Indoor + TransitionOut + Outdoor = 4 events
        // Actually: confirm_indoor does 1 transition (Indoor), confirm_outdoor does 2 (TransitionOut, Outdoor)
        assert!(mgr.history().len() >= 3);
    }
}
