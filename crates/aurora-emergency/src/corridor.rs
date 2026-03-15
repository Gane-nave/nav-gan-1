//! Emergency corridor routing — computes priority routes for emergency vehicles
//! with traffic signal preemption, corridor clearance, and priority override.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Emergency vehicle type — determines routing priority and corridor rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergencyVehicleType {
    Ambulance,
    FireEngine,
    PoliceCar,
    HazmatUnit,
    SearchAndRescue,
    CommandVehicle,
}

/// Priority level for emergency routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EmergencyPriority {
    /// Standard priority — follows traffic rules with minor priority.
    Standard,
    /// Elevated — may use bus lanes and get signal preemption.
    Elevated,
    /// High — full corridor clearance requested.
    High,
    /// Critical — all resources diverted, max priority.
    Critical,
}

/// A corridor segment with clearance status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorSegment {
    pub segment_id: EntityId,
    pub start: GeoPosition,
    pub end: GeoPosition,
    pub road_name: Option<String>,
    pub length_m: f64,
    pub clearance_status: ClearanceStatus,
    pub signal_preemption: bool,
    pub estimated_speed_kmh: f64,
}

/// Status of corridor clearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClearanceStatus {
    /// Not yet requested.
    NotRequested,
    /// Clearance requested, pending.
    Requested,
    /// Corridor is being cleared.
    Clearing,
    /// Corridor is clear.
    Clear,
    /// Clearance failed or blocked.
    Blocked,
}

/// An emergency corridor — a priority route with clearance management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyCorridor {
    pub id: EntityId,
    pub vehicle_type: EmergencyVehicleType,
    pub priority: EmergencyPriority,
    pub segments: Vec<CorridorSegment>,
    pub origin: GeoPosition,
    pub destination: GeoPosition,
    pub total_distance_km: f64,
    pub estimated_time_min: f64,
    pub created_at: DateTime<Utc>,
    pub status: CorridorStatus,
    pub signals_preempted: u32,
    pub segments_cleared: u32,
}

/// Overall corridor status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorridorStatus {
    Planning,
    Active,
    Completed,
    Abandoned,
}

/// Corridor routing engine — plans and manages emergency corridors.
pub struct CorridorRouter {
    corridors: Vec<EmergencyCorridor>,
    /// Speed multiplier for emergency vehicles (vs normal traffic).
    emergency_speed_factor: f64,
    /// Maximum corridor length in km.
    max_corridor_km: f64,
}

impl CorridorRouter {
    pub fn new() -> Self {
        Self {
            corridors: Vec::new(),
            emergency_speed_factor: 1.5,
            max_corridor_km: 100.0,
        }
    }

    /// Plan an emergency corridor from origin to destination.
    pub fn plan_corridor(
        &mut self,
        vehicle_type: EmergencyVehicleType,
        priority: EmergencyPriority,
        origin: GeoPosition,
        destination: GeoPosition,
        waypoints: &[GeoPosition],
    ) -> EntityId {
        let corridor_id = EntityId::new();

        // Build segments from origin through waypoints to destination.
        let mut all_points = vec![origin];
        all_points.extend_from_slice(waypoints);
        all_points.push(destination);

        let segments: Vec<CorridorSegment> = all_points
            .windows(2)
            .map(|pair| {
                let distance = haversine_distance(&pair[0], &pair[1]);
                let base_speed = self.base_speed_for_priority(priority);
                CorridorSegment {
                    segment_id: EntityId::new(),
                    start: pair[0],
                    end: pair[1],
                    road_name: None,
                    length_m: distance,
                    clearance_status: ClearanceStatus::NotRequested,
                    signal_preemption: priority >= EmergencyPriority::Elevated,
                    estimated_speed_kmh: base_speed,
                }
            })
            .collect();

        let total_distance_km: f64 = segments.iter().map(|s| s.length_m).sum::<f64>() / 1000.0;
        let estimated_time_min = segments
            .iter()
            .map(|s| {
                if s.estimated_speed_kmh > 0.0 {
                    (s.length_m / 1000.0) / s.estimated_speed_kmh * 60.0
                } else {
                    0.0
                }
            })
            .sum();

        let corridor = EmergencyCorridor {
            id: corridor_id,
            vehicle_type,
            priority,
            segments,
            origin,
            destination,
            total_distance_km,
            estimated_time_min,
            created_at: Utc::now(),
            status: CorridorStatus::Planning,
            signals_preempted: 0,
            segments_cleared: 0,
        };

        debug!(
            corridor_id = %corridor_id,
            vehicle = ?vehicle_type,
            priority = ?priority,
            distance_km = total_distance_km,
            eta_min = estimated_time_min,
            "Emergency corridor planned"
        );

        self.corridors.push(corridor);
        corridor_id
    }

    /// Activate a corridor — begin clearance operations.
    pub fn activate_corridor(&mut self, corridor_id: &EntityId) -> bool {
        let Some(corridor) = self.corridors.iter_mut().find(|c| c.id == *corridor_id) else {
            return false;
        };
        if corridor.status != CorridorStatus::Planning {
            return false;
        }

        corridor.status = CorridorStatus::Active;

        // Request clearance for all segments.
        for seg in &mut corridor.segments {
            if seg.clearance_status == ClearanceStatus::NotRequested {
                seg.clearance_status = ClearanceStatus::Requested;
            }
        }

        debug!(corridor_id = %corridor_id, "Emergency corridor activated");
        true
    }

    /// Clear a segment (mark it as cleared by traffic control).
    pub fn clear_segment(&mut self, corridor_id: &EntityId, segment_index: usize) -> bool {
        let Some(corridor) = self.corridors.iter_mut().find(|c| c.id == *corridor_id) else {
            return false;
        };
        if segment_index >= corridor.segments.len() {
            return false;
        }

        let prev_status = corridor.segments[segment_index].clearance_status;
        corridor.segments[segment_index].clearance_status = ClearanceStatus::Clear;
        if prev_status != ClearanceStatus::Clear {
            corridor.segments_cleared += 1;

            if corridor.segments[segment_index].signal_preemption {
                corridor.signals_preempted += 1;
            }
        }

        true
    }

    /// Mark a segment as blocked.
    pub fn block_segment(&mut self, corridor_id: &EntityId, segment_index: usize) -> bool {
        let Some(corridor) = self.corridors.iter_mut().find(|c| c.id == *corridor_id) else {
            return false;
        };
        if segment_index >= corridor.segments.len() {
            return false;
        }
        corridor.segments[segment_index].clearance_status = ClearanceStatus::Blocked;
        true
    }

    /// Complete a corridor.
    pub fn complete_corridor(&mut self, corridor_id: &EntityId) -> bool {
        let Some(corridor) = self.corridors.iter_mut().find(|c| c.id == *corridor_id) else {
            return false;
        };
        corridor.status = CorridorStatus::Completed;
        debug!(
            corridor_id = %corridor_id,
            cleared = corridor.segments_cleared,
            preempted = corridor.signals_preempted,
            "Emergency corridor completed"
        );
        true
    }

    /// Get a corridor by ID.
    pub fn corridor(&self, id: &EntityId) -> Option<&EmergencyCorridor> {
        self.corridors.iter().find(|c| c.id == *id)
    }

    /// Get clearance percentage for a corridor.
    pub fn clearance_pct(&self, corridor_id: &EntityId) -> Option<f64> {
        self.corridor(corridor_id).map(|c| {
            if c.segments.is_empty() {
                return 0.0;
            }
            let cleared = c
                .segments
                .iter()
                .filter(|s| s.clearance_status == ClearanceStatus::Clear)
                .count();
            cleared as f64 / c.segments.len() as f64
        })
    }

    /// Check if there are blocked segments in a corridor.
    #[allow(clippy::unnecessary_map_or)]
    pub fn has_blocked_segments(&self, corridor_id: &EntityId) -> bool {
        self.corridor(corridor_id).map_or(false, |c| {
            c.segments
                .iter()
                .any(|s| s.clearance_status == ClearanceStatus::Blocked)
        })
    }

    /// Base speed based on priority level.
    fn base_speed_for_priority(&self, priority: EmergencyPriority) -> f64 {
        let base = match priority {
            EmergencyPriority::Standard => 50.0,
            EmergencyPriority::Elevated => 70.0,
            EmergencyPriority::High => 90.0,
            EmergencyPriority::Critical => 110.0,
        };
        base * self.emergency_speed_factor
    }

    /// Set maximum corridor length.
    pub fn set_max_corridor_km(&mut self, km: f64) {
        self.max_corridor_km = km;
    }

    /// Get maximum corridor length.
    pub fn max_corridor_km(&self) -> f64 {
        self.max_corridor_km
    }

    /// Total corridor count.
    pub fn corridor_count(&self) -> usize {
        self.corridors.len()
    }

    /// Active corridors.
    pub fn active_corridors(&self) -> Vec<&EmergencyCorridor> {
        self.corridors
            .iter()
            .filter(|c| c.status == CorridorStatus::Active)
            .collect()
    }
}

impl Default for CorridorRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let h = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
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

    #[test]
    fn plan_and_activate_corridor() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::High,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[],
        );
        let corridor = router.corridor(&id).unwrap();
        assert_eq!(corridor.status, CorridorStatus::Planning);
        assert_eq!(corridor.segments.len(), 1);
        assert!(corridor.total_distance_km > 0.0);

        assert!(router.activate_corridor(&id));
        assert_eq!(router.corridor(&id).unwrap().status, CorridorStatus::Active);
    }

    #[test]
    fn corridor_with_waypoints() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::FireEngine,
            EmergencyPriority::Critical,
            pos(32.0, 34.0),
            pos(32.2, 34.2),
            &[pos(32.05, 34.05), pos(32.1, 34.1), pos(32.15, 34.15)],
        );
        let corridor = router.corridor(&id).unwrap();
        assert_eq!(corridor.segments.len(), 4);
    }

    #[test]
    fn clear_and_complete_corridor() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::Elevated,
            pos(32.0, 34.0),
            pos(32.05, 34.05),
            &[pos(32.025, 34.025)],
        );
        router.activate_corridor(&id);

        assert!(router.clear_segment(&id, 0));
        assert!(router.clear_segment(&id, 1));
        assert_eq!(router.clearance_pct(&id), Some(1.0));

        assert!(router.complete_corridor(&id));
        assert_eq!(
            router.corridor(&id).unwrap().status,
            CorridorStatus::Completed
        );
    }

    #[test]
    fn blocked_segment_detected() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::PoliceCar,
            EmergencyPriority::High,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[pos(32.05, 34.05)],
        );
        router.activate_corridor(&id);

        router.clear_segment(&id, 0);
        router.block_segment(&id, 1);

        assert!(router.has_blocked_segments(&id));
        assert!((router.clearance_pct(&id).unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn signal_preemption_counted() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::Elevated,
            pos(32.0, 34.0),
            pos(32.05, 34.05),
            &[],
        );
        router.activate_corridor(&id);
        router.clear_segment(&id, 0);

        let corridor = router.corridor(&id).unwrap();
        assert!(corridor.signals_preempted > 0);
        assert!(corridor.segments[0].signal_preemption);
    }

    #[test]
    fn standard_priority_no_preemption() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::CommandVehicle,
            EmergencyPriority::Standard,
            pos(32.0, 34.0),
            pos(32.05, 34.05),
            &[],
        );
        let corridor = router.corridor(&id).unwrap();
        assert!(!corridor.segments[0].signal_preemption);
    }

    #[test]
    fn critical_priority_fastest_eta() {
        let mut router = CorridorRouter::new();
        let standard_id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::Standard,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[],
        );
        let critical_id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::Critical,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[],
        );

        let standard = router.corridor(&standard_id).unwrap();
        let critical = router.corridor(&critical_id).unwrap();
        assert!(
            critical.estimated_time_min < standard.estimated_time_min,
            "Critical should have shorter ETA"
        );
    }

    #[test]
    fn active_corridors_filter() {
        let mut router = CorridorRouter::new();
        let id1 = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::High,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[],
        );
        router.plan_corridor(
            EmergencyVehicleType::FireEngine,
            EmergencyPriority::Critical,
            pos(32.0, 34.0),
            pos(32.1, 34.1),
            &[],
        );

        router.activate_corridor(&id1);
        assert_eq!(router.active_corridors().len(), 1);
    }

    #[test]
    fn cannot_activate_completed_corridor() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::High,
            pos(32.0, 34.0),
            pos(32.05, 34.05),
            &[],
        );
        router.activate_corridor(&id);
        router.complete_corridor(&id);
        assert!(!router.activate_corridor(&id));
    }

    #[test]
    fn clear_segment_idempotent_no_double_count() {
        let mut router = CorridorRouter::new();
        let id = router.plan_corridor(
            EmergencyVehicleType::Ambulance,
            EmergencyPriority::Elevated,
            pos(32.0, 34.0),
            pos(32.05, 34.05),
            &[],
        );
        router.activate_corridor(&id);

        // Clear same segment twice.
        assert!(router.clear_segment(&id, 0));
        assert!(router.clear_segment(&id, 0));

        let corridor = router.corridor(&id).unwrap();
        assert_eq!(
            corridor.segments_cleared, 1,
            "double clear must not double-count"
        );
        assert_eq!(
            corridor.signals_preempted, 1,
            "double clear must not double-count preemption"
        );
    }
}
