//! Evacuation planning — computes evacuation routes, manages assembly points,
//! capacity tracking, and population flow for emergency evacuations.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Evacuation urgency level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvacuationUrgency {
    /// Advisory — voluntary evacuation recommended.
    Advisory,
    /// Warning — evacuation strongly recommended.
    Warning,
    /// Mandatory — immediate evacuation required.
    Mandatory,
    /// Immediate — life-threatening, evacuate now.
    Immediate,
}

/// An evacuation zone — area to be evacuated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvacuationZone {
    pub id: EntityId,
    pub name: String,
    pub center: GeoPosition,
    pub radius_m: f64,
    pub urgency: EvacuationUrgency,
    pub estimated_population: u32,
    pub evacuated_count: u32,
    pub status: ZoneEvacStatus,
}

/// Evacuation zone status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneEvacStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

/// An assembly point — safe gathering location for evacuees.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyPoint {
    pub id: EntityId,
    pub name: String,
    pub position: GeoPosition,
    pub capacity: u32,
    pub current_occupancy: u32,
    pub has_medical: bool,
    pub has_shelter: bool,
    pub has_transport: bool,
    pub status: AssemblyPointStatus,
}

/// Assembly point status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssemblyPointStatus {
    Available,
    Active,
    Full,
    Closed,
}

/// An evacuation route connecting a zone to an assembly point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvacuationRoute {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub assembly_point_id: EntityId,
    pub waypoints: Vec<GeoPosition>,
    pub distance_km: f64,
    pub estimated_travel_min: f64,
    pub capacity_persons_per_hour: u32,
    pub is_accessible: bool,
    pub status: RouteStatus,
}

/// Route status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteStatus {
    Planned,
    Open,
    Congested,
    Blocked,
    Closed,
}

/// An evacuation plan — coordinates zones, assembly points, and routes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvacuationPlan {
    pub id: EntityId,
    pub incident_id: Option<EntityId>,
    pub name: String,
    pub zones: Vec<EvacuationZone>,
    pub assembly_points: Vec<AssemblyPoint>,
    pub routes: Vec<EvacuationRoute>,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: PlanStatus,
}

/// Plan status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Draft,
    Active,
    Completed,
    Cancelled,
}

/// Evacuation planner — creates and manages evacuation plans.
pub struct EvacuationPlanner {
    plans: Vec<EvacuationPlan>,
    /// Walking speed estimate for evacuees (km/h).
    walking_speed_kmh: f64,
}

impl EvacuationPlanner {
    pub fn new() -> Self {
        Self {
            plans: Vec::new(),
            walking_speed_kmh: 4.5,
        }
    }

    /// Create a new evacuation plan.
    pub fn create_plan(&mut self, name: &str, incident_id: Option<EntityId>) -> EntityId {
        let id = EntityId::new();
        let plan = EvacuationPlan {
            id,
            incident_id,
            name: name.to_string(),
            zones: Vec::new(),
            assembly_points: Vec::new(),
            routes: Vec::new(),
            created_at: Utc::now(),
            activated_at: None,
            completed_at: None,
            status: PlanStatus::Draft,
        };
        debug!(plan_id = %id, name, "Evacuation plan created");
        self.plans.push(plan);
        id
    }

    /// Add an evacuation zone to a plan.
    pub fn add_zone(
        &mut self,
        plan_id: &EntityId,
        name: &str,
        center: GeoPosition,
        radius_m: f64,
        urgency: EvacuationUrgency,
        estimated_population: u32,
    ) -> Option<EntityId> {
        let plan = self.plans.iter_mut().find(|p| p.id == *plan_id)?;
        let zone_id = EntityId::new();
        plan.zones.push(EvacuationZone {
            id: zone_id,
            name: name.to_string(),
            center,
            radius_m,
            urgency,
            estimated_population,
            evacuated_count: 0,
            status: ZoneEvacStatus::Planned,
        });
        debug!(plan_id = %plan_id, zone = name, pop = estimated_population, "Zone added");
        Some(zone_id)
    }

    /// Add an assembly point to a plan.
    pub fn add_assembly_point(
        &mut self,
        plan_id: &EntityId,
        name: &str,
        position: GeoPosition,
        capacity: u32,
        has_medical: bool,
        has_shelter: bool,
    ) -> Option<EntityId> {
        let plan = self.plans.iter_mut().find(|p| p.id == *plan_id)?;
        let ap_id = EntityId::new();
        plan.assembly_points.push(AssemblyPoint {
            id: ap_id,
            name: name.to_string(),
            position,
            capacity,
            current_occupancy: 0,
            has_medical,
            has_shelter,
            has_transport: false,
            status: AssemblyPointStatus::Available,
        });
        Some(ap_id)
    }

    /// Add an evacuation route linking a zone to an assembly point.
    pub fn add_route(
        &mut self,
        plan_id: &EntityId,
        zone_id: EntityId,
        assembly_point_id: EntityId,
        waypoints: Vec<GeoPosition>,
        is_accessible: bool,
    ) -> Option<EntityId> {
        // Compute distance before taking mutable borrow on plan.
        let distance_km = self.compute_route_distance(&waypoints);
        let travel_min = if self.walking_speed_kmh > 0.0 {
            distance_km / self.walking_speed_kmh * 60.0
        } else {
            0.0
        };

        let plan = self.plans.iter_mut().find(|p| p.id == *plan_id)?;

        // Verify zone and assembly point exist.
        if !plan.zones.iter().any(|z| z.id == zone_id) {
            return None;
        }
        if !plan
            .assembly_points
            .iter()
            .any(|a| a.id == assembly_point_id)
        {
            return None;
        }

        let route_id = EntityId::new();
        plan.routes.push(EvacuationRoute {
            id: route_id,
            zone_id,
            assembly_point_id,
            waypoints,
            distance_km,
            estimated_travel_min: travel_min,
            capacity_persons_per_hour: 500, // Default pedestrian throughput.
            is_accessible,
            status: RouteStatus::Planned,
        });
        Some(route_id)
    }

    /// Activate an evacuation plan.
    pub fn activate_plan(&mut self, plan_id: &EntityId) -> bool {
        let Some(plan) = self.plans.iter_mut().find(|p| p.id == *plan_id) else {
            return false;
        };
        if plan.status != PlanStatus::Draft {
            return false;
        }
        if plan.zones.is_empty() || plan.assembly_points.is_empty() {
            return false;
        }

        plan.status = PlanStatus::Active;
        plan.activated_at = Some(Utc::now());

        // Activate all zones.
        for zone in &mut plan.zones {
            zone.status = ZoneEvacStatus::InProgress;
        }
        // Open all routes.
        for route in &mut plan.routes {
            route.status = RouteStatus::Open;
        }
        // Activate assembly points.
        for ap in &mut plan.assembly_points {
            ap.status = AssemblyPointStatus::Active;
        }

        debug!(plan_id = %plan_id, "Evacuation plan activated");
        true
    }

    /// Update evacuee count for a zone.
    pub fn update_evacuees(
        &mut self,
        plan_id: &EntityId,
        zone_id: &EntityId,
        evacuated_count: u32,
    ) -> bool {
        let Some(plan) = self.plans.iter_mut().find(|p| p.id == *plan_id) else {
            return false;
        };
        let Some(zone) = plan.zones.iter_mut().find(|z| z.id == *zone_id) else {
            return false;
        };
        zone.evacuated_count = evacuated_count;
        if evacuated_count >= zone.estimated_population {
            zone.status = ZoneEvacStatus::Completed;
        }
        true
    }

    /// Update assembly point occupancy.
    pub fn update_occupancy(
        &mut self,
        plan_id: &EntityId,
        ap_id: &EntityId,
        occupancy: u32,
    ) -> bool {
        let Some(plan) = self.plans.iter_mut().find(|p| p.id == *plan_id) else {
            return false;
        };
        let Some(ap) = plan.assembly_points.iter_mut().find(|a| a.id == *ap_id) else {
            return false;
        };
        ap.current_occupancy = occupancy;
        if occupancy >= ap.capacity {
            ap.status = AssemblyPointStatus::Full;
        }
        true
    }

    /// Block a route.
    pub fn block_route(&mut self, plan_id: &EntityId, route_id: &EntityId) -> bool {
        let Some(plan) = self.plans.iter_mut().find(|p| p.id == *plan_id) else {
            return false;
        };
        let Some(route) = plan.routes.iter_mut().find(|r| r.id == *route_id) else {
            return false;
        };
        route.status = RouteStatus::Blocked;
        true
    }

    /// Get evacuation progress percentage for a plan.
    pub fn progress_pct(&self, plan_id: &EntityId) -> Option<f64> {
        let plan = self.plans.iter().find(|p| p.id == *plan_id)?;
        if plan.zones.is_empty() {
            return Some(0.0);
        }
        let total_pop: u32 = plan.zones.iter().map(|z| z.estimated_population).sum();
        let total_evac: u32 = plan.zones.iter().map(|z| z.evacuated_count).sum();
        if total_pop == 0 {
            return Some(0.0);
        }
        Some(total_evac as f64 / total_pop as f64 * 100.0)
    }

    /// Total remaining capacity across assembly points.
    pub fn remaining_capacity(&self, plan_id: &EntityId) -> Option<u32> {
        let plan = self.plans.iter().find(|p| p.id == *plan_id)?;
        Some(
            plan.assembly_points
                .iter()
                .filter(|a| a.status != AssemblyPointStatus::Closed)
                .map(|a| a.capacity.saturating_sub(a.current_occupancy))
                .sum(),
        )
    }

    /// Complete a plan.
    pub fn complete_plan(&mut self, plan_id: &EntityId) -> bool {
        let Some(plan) = self.plans.iter_mut().find(|p| p.id == *plan_id) else {
            return false;
        };
        plan.status = PlanStatus::Completed;
        plan.completed_at = Some(Utc::now());
        true
    }

    /// Get a plan by ID.
    pub fn plan(&self, id: &EntityId) -> Option<&EvacuationPlan> {
        self.plans.iter().find(|p| p.id == *id)
    }

    /// Plan count.
    pub fn plan_count(&self) -> usize {
        self.plans.len()
    }

    /// Compute route distance from waypoints.
    fn compute_route_distance(&self, waypoints: &[GeoPosition]) -> f64 {
        if waypoints.len() < 2 {
            return 0.0;
        }
        let mut total = 0.0;
        for pair in waypoints.windows(2) {
            total += haversine_distance(&pair[0], &pair[1]) / 1000.0;
        }
        total
    }
}

impl Default for EvacuationPlanner {
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
    fn create_and_activate_plan() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Building Fire Evac", None);

        let zone_id = planner
            .add_zone(
                &plan_id,
                "Block A",
                pos(32.0, 34.0),
                200.0,
                EvacuationUrgency::Mandatory,
                500,
            )
            .unwrap();
        let ap_id = planner
            .add_assembly_point(&plan_id, "Park", pos(32.005, 34.005), 1000, true, true)
            .unwrap();
        planner.add_route(
            &plan_id,
            zone_id,
            ap_id,
            vec![pos(32.0, 34.0), pos(32.002, 34.002), pos(32.005, 34.005)],
            true,
        );

        assert!(planner.activate_plan(&plan_id));
        let plan = planner.plan(&plan_id).unwrap();
        assert_eq!(plan.status, PlanStatus::Active);
        assert!(plan.activated_at.is_some());
    }

    #[test]
    fn cannot_activate_empty_plan() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Empty Plan", None);
        assert!(!planner.activate_plan(&plan_id));
    }

    #[test]
    fn evacuation_progress_tracking() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Evacuation", None);
        let zone_id = planner
            .add_zone(
                &plan_id,
                "Zone 1",
                pos(32.0, 34.0),
                100.0,
                EvacuationUrgency::Warning,
                100,
            )
            .unwrap();
        planner.add_assembly_point(&plan_id, "Point A", pos(32.01, 34.01), 200, false, false);

        assert_eq!(planner.progress_pct(&plan_id), Some(0.0));

        planner.update_evacuees(&plan_id, &zone_id, 50);
        assert!((planner.progress_pct(&plan_id).unwrap() - 50.0).abs() < 0.1);

        planner.update_evacuees(&plan_id, &zone_id, 100);
        assert!((planner.progress_pct(&plan_id).unwrap() - 100.0).abs() < 0.1);
    }

    #[test]
    fn zone_auto_completes_when_fully_evacuated() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Test", None);
        let zone_id = planner
            .add_zone(
                &plan_id,
                "Z1",
                pos(32.0, 34.0),
                50.0,
                EvacuationUrgency::Immediate,
                200,
            )
            .unwrap();

        planner.update_evacuees(&plan_id, &zone_id, 200);
        let plan = planner.plan(&plan_id).unwrap();
        assert_eq!(plan.zones[0].status, ZoneEvacStatus::Completed);
    }

    #[test]
    fn assembly_point_capacity_tracking() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Capacity Test", None);
        let ap_id = planner
            .add_assembly_point(&plan_id, "Shelter", pos(32.01, 34.01), 100, true, true)
            .unwrap();

        assert_eq!(planner.remaining_capacity(&plan_id), Some(100));

        planner.update_occupancy(&plan_id, &ap_id, 60);
        assert_eq!(planner.remaining_capacity(&plan_id), Some(40));

        planner.update_occupancy(&plan_id, &ap_id, 100);
        let plan = planner.plan(&plan_id).unwrap();
        assert_eq!(plan.assembly_points[0].status, AssemblyPointStatus::Full);
        assert_eq!(planner.remaining_capacity(&plan_id), Some(0));
    }

    #[test]
    fn block_evacuation_route() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Route Test", None);
        let zone_id = planner
            .add_zone(
                &plan_id,
                "Z",
                pos(32.0, 34.0),
                100.0,
                EvacuationUrgency::Mandatory,
                50,
            )
            .unwrap();
        let ap_id = planner
            .add_assembly_point(&plan_id, "AP", pos(32.01, 34.01), 100, false, false)
            .unwrap();
        let route_id = planner
            .add_route(
                &plan_id,
                zone_id,
                ap_id,
                vec![pos(32.0, 34.0), pos(32.01, 34.01)],
                true,
            )
            .unwrap();

        planner.activate_plan(&plan_id);
        assert!(planner.block_route(&plan_id, &route_id));
        let plan = planner.plan(&plan_id).unwrap();
        assert_eq!(plan.routes[0].status, RouteStatus::Blocked);
    }

    #[test]
    fn route_requires_valid_zone_and_ap() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Invalid Route", None);
        let zone_id = planner
            .add_zone(
                &plan_id,
                "Z",
                pos(32.0, 34.0),
                50.0,
                EvacuationUrgency::Advisory,
                10,
            )
            .unwrap();

        // Invalid assembly point.
        let result = planner.add_route(
            &plan_id,
            zone_id,
            EntityId::new(),
            vec![pos(32.0, 34.0), pos(32.01, 34.01)],
            true,
        );
        assert!(result.is_none());
    }

    #[test]
    fn multiple_zones_progress() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Multi-Zone", None);
        let z1 = planner
            .add_zone(
                &plan_id,
                "Z1",
                pos(32.0, 34.0),
                100.0,
                EvacuationUrgency::Mandatory,
                100,
            )
            .unwrap();
        let z2 = planner
            .add_zone(
                &plan_id,
                "Z2",
                pos(32.01, 34.01),
                100.0,
                EvacuationUrgency::Warning,
                200,
            )
            .unwrap();

        planner.update_evacuees(&plan_id, &z1, 50); // 50/100 from Z1.
        planner.update_evacuees(&plan_id, &z2, 100); // 100/200 from Z2.
                                                     // Total: 150/300 = 50%.
        assert!((planner.progress_pct(&plan_id).unwrap() - 50.0).abs() < 0.1);
    }

    #[test]
    fn urgency_ordering() {
        assert!(EvacuationUrgency::Immediate > EvacuationUrgency::Mandatory);
        assert!(EvacuationUrgency::Mandatory > EvacuationUrgency::Warning);
        assert!(EvacuationUrgency::Warning > EvacuationUrgency::Advisory);
    }

    #[test]
    fn complete_plan() {
        let mut planner = EvacuationPlanner::new();
        let plan_id = planner.create_plan("Complete Test", None);
        planner.add_zone(
            &plan_id,
            "Z",
            pos(32.0, 34.0),
            50.0,
            EvacuationUrgency::Advisory,
            10,
        );
        planner.add_assembly_point(&plan_id, "AP", pos(32.01, 34.01), 20, false, false);
        planner.activate_plan(&plan_id);

        assert!(planner.complete_plan(&plan_id));
        assert_eq!(
            planner.plan(&plan_id).unwrap().status,
            PlanStatus::Completed
        );
    }
}
