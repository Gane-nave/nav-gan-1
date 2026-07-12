//! Gate-level arrival and last-metre guidance — navigates users to the exact
//! entrance, gate, pickup point, or dropoff zone at their destination.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A point of interest at a destination (gate, entrance, pickup zone, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrivalPoint {
    pub id: EntityId,
    pub destination_id: EntityId,
    pub name: String,
    pub point_type: ArrivalPointType,
    pub position: GeoPosition,
    /// Whether this point is currently accessible.
    pub accessible: bool,
    /// Optional operating hours description.
    pub hours: Option<String>,
    /// Priority for routing (higher = preferred).
    pub priority: i32,
}

/// Types of arrival points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrivalPointType {
    MainEntrance,
    SideEntrance,
    ParkingEntrance,
    Gate,
    PickupZone,
    DropoffZone,
    LoadingDock,
    EmergencyEntrance,
    PedestrianEntrance,
    ServiceEntrance,
}

/// A micro-navigation instruction for the last portion of a route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroInstruction {
    pub step: u32,
    pub instruction: String,
    pub instruction_type: MicroInstructionType,
    pub position: GeoPosition,
    pub distance_m: f64,
    pub heading_deg: Option<f64>,
}

/// Types of micro-navigation instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MicroInstructionType {
    /// Continue straight ahead.
    Continue,
    /// Turn at an intersection or path junction.
    Turn,
    /// Enter a building or structure.
    Enter,
    /// Exit a building or structure.
    Exit,
    /// Take stairs or elevator.
    LevelChange,
    /// Arrive at the destination point.
    Arrive,
    /// Wait at a gate or barrier.
    Wait,
    /// Cross a road or walkway.
    Cross,
}

/// Configuration for the arrival engine.
#[derive(Debug, Clone)]
pub struct ArrivalConfig {
    /// Distance (metres) at which micro-navigation activates.
    pub activation_radius_m: f64,
    /// Maximum number of arrival points to consider.
    pub max_candidates: usize,
    /// Whether to prefer accessible entrances.
    pub prefer_accessible: bool,
}

impl Default for ArrivalConfig {
    fn default() -> Self {
        Self {
            activation_radius_m: 200.0,
            max_candidates: 5,
            prefer_accessible: true,
        }
    }
}

/// The arrival engine — selects the best arrival point and generates
/// micro-navigation instructions.
pub struct ArrivalEngine {
    config: ArrivalConfig,
    points: Vec<ArrivalPoint>,
    active_guidance: Option<ActiveGuidance>,
}

/// Active guidance state.
#[derive(Debug, Clone)]
pub struct ActiveGuidance {
    pub target: ArrivalPoint,
    pub instructions: Vec<MicroInstruction>,
    pub started_at: DateTime<Utc>,
    pub current_step: u32,
}

impl ArrivalEngine {
    pub fn new() -> Self {
        Self {
            config: ArrivalConfig::default(),
            points: Vec::new(),
            active_guidance: None,
        }
    }

    pub fn with_config(config: ArrivalConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Register an arrival point for a destination.
    pub fn add_point(&mut self, point: ArrivalPoint) {
        debug!(
            name = %point.name,
            point_type = ?point.point_type,
            "arrival point registered"
        );
        self.points.push(point);
    }

    /// Get all arrival points for a destination.
    pub fn points_for_destination(&self, destination_id: &EntityId) -> Vec<&ArrivalPoint> {
        self.points
            .iter()
            .filter(|p| p.destination_id == *destination_id)
            .collect()
    }

    /// Select the best arrival point for a destination given current position
    /// and transport mode preferences.
    pub fn select_best(
        &self,
        destination_id: &EntityId,
        current_pos: &GeoPosition,
        point_type_filter: Option<ArrivalPointType>,
    ) -> Option<&ArrivalPoint> {
        let mut candidates: Vec<&ArrivalPoint> = self
            .points
            .iter()
            .filter(|p| {
                p.destination_id == *destination_id
                    && p.accessible
                    && point_type_filter.map_or(true, |t| p.point_type == t)
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Sort by: priority (descending), then distance (ascending).
        candidates.sort_by(|a, b| {
            let pri_cmp = b.priority.cmp(&a.priority);
            if pri_cmp != std::cmp::Ordering::Equal {
                return pri_cmp;
            }
            let dist_a = haversine_distance(current_pos, &a.position);
            let dist_b = haversine_distance(current_pos, &b.position);
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        candidates
            .into_iter()
            .take(self.config.max_candidates)
            .next()
    }

    /// Check if the user is within the activation radius of any arrival point.
    pub fn is_in_arrival_zone(&self, current_pos: &GeoPosition, destination_id: &EntityId) -> bool {
        self.points.iter().any(|p| {
            p.destination_id == *destination_id
                && haversine_distance(current_pos, &p.position) <= self.config.activation_radius_m
        })
    }

    /// Start micro-navigation guidance toward an arrival point.
    pub fn start_guidance(&mut self, target: &ArrivalPoint, current_pos: &GeoPosition) {
        let distance = haversine_distance(current_pos, &target.position);

        let instructions = vec![
            MicroInstruction {
                step: 1,
                instruction: format!("Head toward {}", target.name),
                instruction_type: MicroInstructionType::Continue,
                position: *current_pos,
                distance_m: distance,
                heading_deg: Some(bearing(current_pos, &target.position)),
            },
            MicroInstruction {
                step: 2,
                instruction: format!("Arrive at {}", target.name),
                instruction_type: MicroInstructionType::Arrive,
                position: target.position,
                distance_m: 0.0,
                heading_deg: None,
            },
        ];

        debug!(
            target = %target.name,
            distance_m = distance,
            "micro-navigation guidance started"
        );

        self.active_guidance = Some(ActiveGuidance {
            target: target.clone(),
            instructions,
            started_at: Utc::now(),
            current_step: 1,
        });
    }

    /// Advance to the next micro-navigation step.
    pub fn advance_step(&mut self) -> Option<&MicroInstruction> {
        if let Some(guidance) = &mut self.active_guidance {
            if (guidance.current_step as usize) < guidance.instructions.len() {
                guidance.current_step += 1;
            }
            guidance
                .instructions
                .get(guidance.current_step as usize - 1)
        } else {
            None
        }
    }

    /// Get the current guidance state.
    pub fn active_guidance(&self) -> Option<&ActiveGuidance> {
        self.active_guidance.as_ref()
    }

    /// Stop active guidance.
    pub fn stop_guidance(&mut self) {
        self.active_guidance = None;
    }

    /// Get the distance to the active target.
    pub fn distance_to_target(&self, current_pos: &GeoPosition) -> Option<f64> {
        self.active_guidance
            .as_ref()
            .map(|g| haversine_distance(current_pos, &g.target.position))
    }

    /// Get the total number of registered arrival points.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl Default for ArrivalEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance between two geo positions in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0; // Earth radius in metres
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let dlat = (b.latitude_deg - a.latitude_deg).to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();

    let a_val = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().atan2((1.0 - a_val).sqrt());
    r * c
}

/// Bearing from point a to point b in degrees [0, 360).
fn bearing(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let brng = y.atan2(x).to_degrees();
    (brng + 360.0) % 360.0
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

    fn make_point(
        dest_id: EntityId,
        name: &str,
        pt: ArrivalPointType,
        lat: f64,
        lon: f64,
        priority: i32,
    ) -> ArrivalPoint {
        ArrivalPoint {
            id: EntityId::new(),
            destination_id: dest_id,
            name: name.into(),
            point_type: pt,
            position: pos(lat, lon),
            accessible: true,
            hours: None,
            priority,
        }
    }

    #[test]
    fn add_and_query_points() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        engine.add_point(make_point(
            dest,
            "Main Gate",
            ArrivalPointType::MainEntrance,
            32.0,
            34.0,
            10,
        ));
        engine.add_point(make_point(
            dest,
            "Side Gate",
            ArrivalPointType::SideEntrance,
            32.001,
            34.001,
            5,
        ));
        assert_eq!(engine.points_for_destination(&dest).len(), 2);
        assert_eq!(engine.points_for_destination(&EntityId::new()).len(), 0);
    }

    #[test]
    fn select_best_by_priority() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        engine.add_point(make_point(
            dest,
            "Low",
            ArrivalPointType::SideEntrance,
            32.001,
            34.001,
            1,
        ));
        engine.add_point(make_point(
            dest,
            "High",
            ArrivalPointType::MainEntrance,
            32.002,
            34.002,
            10,
        ));

        let best = engine.select_best(&dest, &pos(32.0, 34.0), None).unwrap();
        assert_eq!(best.name, "High");
    }

    #[test]
    fn select_best_with_type_filter() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        engine.add_point(make_point(
            dest,
            "Main",
            ArrivalPointType::MainEntrance,
            32.0,
            34.0,
            10,
        ));
        engine.add_point(make_point(
            dest,
            "Pickup",
            ArrivalPointType::PickupZone,
            32.001,
            34.0,
            5,
        ));

        let best = engine
            .select_best(&dest, &pos(32.0, 34.0), Some(ArrivalPointType::PickupZone))
            .unwrap();
        assert_eq!(best.name, "Pickup");
    }

    #[test]
    fn inaccessible_point_filtered() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        let mut point = make_point(
            dest,
            "Closed",
            ArrivalPointType::MainEntrance,
            32.0,
            34.0,
            10,
        );
        point.accessible = false;
        engine.add_point(point);

        assert!(engine.select_best(&dest, &pos(32.0, 34.0), None).is_none());
    }

    #[test]
    fn arrival_zone_detection() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        engine.add_point(make_point(
            dest,
            "Gate",
            ArrivalPointType::Gate,
            32.0,
            34.0,
            5,
        ));

        // Very close — within 200m.
        assert!(engine.is_in_arrival_zone(&pos(32.0001, 34.0001), &dest));
        // Far away.
        assert!(!engine.is_in_arrival_zone(&pos(33.0, 35.0), &dest));
    }

    #[test]
    fn guidance_lifecycle() {
        let mut engine = ArrivalEngine::new();
        let dest = EntityId::new();
        let point = make_point(dest, "Gate A", ArrivalPointType::Gate, 32.001, 34.001, 5);
        engine.add_point(point.clone());

        let current = pos(32.0, 34.0);
        engine.start_guidance(&point, &current);
        assert!(engine.active_guidance().is_some());
        assert_eq!(engine.active_guidance().unwrap().current_step, 1);

        // Advance.
        let next = engine.advance_step();
        assert!(next.is_some());
        assert_eq!(next.unwrap().instruction_type, MicroInstructionType::Arrive);

        // Stop.
        engine.stop_guidance();
        assert!(engine.active_guidance().is_none());
    }

    #[test]
    fn distance_to_target() {
        let mut engine = ArrivalEngine::new();
        let point = make_point(
            EntityId::new(),
            "Gate",
            ArrivalPointType::Gate,
            32.01,
            34.0,
            5,
        );
        engine.start_guidance(&point, &pos(32.0, 34.0));

        let dist = engine.distance_to_target(&pos(32.0, 34.0)).unwrap();
        assert!(dist > 1000.0); // ~1.1 km
        assert!(dist < 1200.0);
    }

    #[test]
    fn haversine_same_point_is_zero() {
        let p = pos(32.0, 34.0);
        assert!(haversine_distance(&p, &p).abs() < 0.01);
    }

    #[test]
    fn bearing_north() {
        let a = pos(32.0, 34.0);
        let b = pos(33.0, 34.0); // Due north.
        let brng = bearing(&a, &b);
        assert!(!(1.0..=359.0).contains(&brng)); // ~0 degrees.
    }
}
