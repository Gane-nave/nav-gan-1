//! City-wide state aggregation — zone management, congestion zones,
//! emission zones, district-level metrics, and city dashboard data.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Zone types
// ---------------------------------------------------------------------------

/// Kind of managed zone within a city.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneKind {
    /// Low-emission zone with vehicle class restrictions.
    LowEmission,
    /// Congestion-charge zone.
    CongestionCharge,
    /// Pedestrian-only zone.
    PedestrianOnly,
    /// School zone with reduced speed.
    SchoolZone,
    /// Residential quiet zone.
    ResidentialQuiet,
    /// Construction zone with temporary restrictions.
    Construction,
    /// Special event zone (stadium, parade, etc.).
    SpecialEvent,
}

/// A managed zone within the city.
#[derive(Debug, Clone)]
pub struct ManagedZone {
    pub id: EntityId,
    pub name: String,
    pub kind: ZoneKind,
    /// Bounding polygon vertices (lat, lon).
    pub boundary: Vec<(f64, f64)>,
    /// Whether the zone is currently active.
    pub active: bool,
    /// Optional speed limit override (km/h).
    pub speed_limit_kmh: Option<f64>,
    /// Optional charge amount for congestion zones.
    pub charge_amount: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Aggregated metrics for a district or zone.
#[derive(Debug, Clone)]
pub struct DistrictMetrics {
    pub district_id: EntityId,
    pub name: String,
    /// Average congestion level [0, 1].
    pub avg_congestion: f64,
    /// Average air quality index (0–500, lower is better).
    pub avg_aqi: f64,
    /// Active incident count.
    pub active_incidents: u32,
    /// Average road condition score [0, 1].
    pub road_health: f64,
    /// Pedestrian density (estimated per km²).
    pub pedestrian_density: f64,
    pub computed_at: DateTime<Utc>,
}

/// City-level summary dashboard data.
#[derive(Debug, Clone)]
pub struct CityDashboard {
    pub city_name: String,
    pub total_zones: usize,
    pub active_zones: usize,
    pub total_districts: usize,
    pub avg_congestion: f64,
    pub avg_aqi: f64,
    pub total_active_incidents: u32,
    pub computed_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// City state manager
// ---------------------------------------------------------------------------

/// Manages city-wide state: zones, district metrics, and dashboard
/// aggregation for the AURORA NAV smart-city integration.
pub struct CityStateManager {
    city_name: String,
    /// Managed zones keyed by ID.
    zones: HashMap<EntityId, ManagedZone>,
    /// District metrics keyed by district ID.
    districts: HashMap<EntityId, DistrictMetrics>,
    /// AQI readings per district.
    aqi_readings: HashMap<EntityId, Vec<f64>>,
}

impl CityStateManager {
    pub fn new(city_name: impl Into<String>) -> Self {
        Self {
            city_name: city_name.into(),
            zones: HashMap::new(),
            districts: HashMap::new(),
            aqi_readings: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Zone management
    // -----------------------------------------------------------------------

    /// Register a managed zone.
    pub fn add_zone(&mut self, zone: ManagedZone) {
        info!(id = %zone.id, name = %zone.name, kind = ?zone.kind, "zone registered");
        self.zones.insert(zone.id, zone);
    }

    /// Activate a zone.
    pub fn activate_zone(&mut self, zone_id: &EntityId) -> bool {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            zone.active = true;
            zone.updated_at = Utc::now();
            debug!(id = %zone_id, "zone activated");
            return true;
        }
        false
    }

    /// Deactivate a zone.
    pub fn deactivate_zone(&mut self, zone_id: &EntityId) -> bool {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            zone.active = false;
            zone.updated_at = Utc::now();
            debug!(id = %zone_id, "zone deactivated");
            return true;
        }
        false
    }

    /// Check whether a point falls inside a zone (ray-casting algorithm).
    pub fn point_in_zone(&self, zone_id: &EntityId, lat: f64, lon: f64) -> bool {
        let Some(zone) = self.zones.get(zone_id) else {
            return false;
        };
        if !zone.active {
            return false;
        }
        point_in_polygon(lat, lon, &zone.boundary)
    }

    /// Get all active zones that contain a point.
    pub fn zones_at_point(&self, lat: f64, lon: f64) -> Vec<&ManagedZone> {
        self.zones
            .values()
            .filter(|z| z.active && point_in_polygon(lat, lon, &z.boundary))
            .collect()
    }

    /// Get zones by kind.
    pub fn zones_by_kind(&self, kind: ZoneKind) -> Vec<&ManagedZone> {
        self.zones.values().filter(|z| z.kind == kind).collect()
    }

    /// Count of all registered zones.
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Count of active zones.
    pub fn active_zone_count(&self) -> usize {
        self.zones.values().filter(|z| z.active).count()
    }

    /// Get a zone by ID.
    pub fn get_zone(&self, id: &EntityId) -> Option<&ManagedZone> {
        self.zones.get(id)
    }

    /// Get the effective speed limit at a point (lowest of all active zones).
    pub fn effective_speed_limit(&self, lat: f64, lon: f64) -> Option<f64> {
        let zones = self.zones_at_point(lat, lon);
        zones
            .iter()
            .filter_map(|z| z.speed_limit_kmh)
            .reduce(f64::min)
    }

    /// Get total congestion charge at a point.
    pub fn total_charge(&self, lat: f64, lon: f64) -> f64 {
        let zones = self.zones_at_point(lat, lon);
        zones.iter().filter_map(|z| z.charge_amount).sum()
    }

    // -----------------------------------------------------------------------
    // District metrics
    // -----------------------------------------------------------------------

    /// Update metrics for a district.
    pub fn update_district_metrics(&mut self, metrics: DistrictMetrics) {
        debug!(district = %metrics.district_id, name = %metrics.name, "district updated");
        self.districts.insert(metrics.district_id, metrics);
    }

    /// Record an AQI reading for a district.
    pub fn record_aqi(&mut self, district_id: EntityId, aqi: f64) {
        self.aqi_readings.entry(district_id).or_default().push(aqi);
    }

    /// Get average AQI for a district from recorded readings.
    pub fn average_aqi(&self, district_id: &EntityId) -> Option<f64> {
        let readings = self.aqi_readings.get(district_id)?;
        if readings.is_empty() {
            return None;
        }
        Some(readings.iter().sum::<f64>() / readings.len() as f64)
    }

    /// Get the most congested district.
    pub fn most_congested_district(&self) -> Option<&DistrictMetrics> {
        self.districts
            .values()
            .max_by(|a, b| a.avg_congestion.partial_cmp(&b.avg_congestion).unwrap())
    }

    /// Get district metrics by ID.
    pub fn get_district(&self, id: &EntityId) -> Option<&DistrictMetrics> {
        self.districts.get(id)
    }

    /// Number of districts.
    pub fn district_count(&self) -> usize {
        self.districts.len()
    }

    // -----------------------------------------------------------------------
    // City dashboard
    // -----------------------------------------------------------------------

    /// Compute the city-level dashboard summary.
    pub fn compute_dashboard(&self) -> CityDashboard {
        let total_zones = self.zones.len();
        let active_zones = self.zones.values().filter(|z| z.active).count();
        let total_districts = self.districts.len();

        let (avg_congestion, avg_aqi, total_incidents) = if self.districts.is_empty() {
            (0.0, 0.0, 0)
        } else {
            let n = self.districts.len() as f64;
            let congestion: f64 = self
                .districts
                .values()
                .map(|d| d.avg_congestion)
                .sum::<f64>()
                / n;
            let aqi: f64 = self.districts.values().map(|d| d.avg_aqi).sum::<f64>() / n;
            let incidents: u32 = self.districts.values().map(|d| d.active_incidents).sum();
            (congestion, aqi, incidents)
        };

        CityDashboard {
            city_name: self.city_name.clone(),
            total_zones,
            active_zones,
            total_districts,
            avg_congestion,
            avg_aqi,
            total_active_incidents: total_incidents,
            computed_at: Utc::now(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Ray-casting point-in-polygon test.
fn point_in_polygon(lat: f64, lon: f64, polygon: &[(f64, f64)]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;

    for i in 0..n {
        let (yi, xi) = polygon[i];
        let (yj, xj) = polygon[j];

        if ((yi > lat) != (yj > lat)) && (lon < (xj - xi) * (lat - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square_zone(id: EntityId, kind: ZoneKind, active: bool) -> ManagedZone {
        // Square zone from (0,0) to (1,1).
        ManagedZone {
            id,
            name: "Test Zone".into(),
            kind,
            boundary: vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
            active,
            speed_limit_kmh: Some(30.0),
            charge_amount: Some(5.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_district(id: EntityId, name: &str, congestion: f64, aqi: f64) -> DistrictMetrics {
        DistrictMetrics {
            district_id: id,
            name: name.into(),
            avg_congestion: congestion,
            avg_aqi: aqi,
            active_incidents: 2,
            road_health: 0.8,
            pedestrian_density: 100.0,
            computed_at: Utc::now(),
        }
    }

    #[test]
    fn add_and_query_zone() {
        let mut mgr = CityStateManager::new("TestCity");
        let zid = EntityId::new();
        mgr.add_zone(square_zone(zid, ZoneKind::LowEmission, true));

        assert_eq!(mgr.zone_count(), 1);
        assert_eq!(mgr.active_zone_count(), 1);
        assert!(mgr.get_zone(&zid).is_some());
    }

    #[test]
    fn activate_deactivate_zone() {
        let mut mgr = CityStateManager::new("TestCity");
        let zid = EntityId::new();
        mgr.add_zone(square_zone(zid, ZoneKind::CongestionCharge, false));

        assert_eq!(mgr.active_zone_count(), 0);
        assert!(mgr.activate_zone(&zid));
        assert_eq!(mgr.active_zone_count(), 1);
        assert!(mgr.deactivate_zone(&zid));
        assert_eq!(mgr.active_zone_count(), 0);
    }

    #[test]
    fn point_inside_active_zone() {
        let mut mgr = CityStateManager::new("TestCity");
        let zid = EntityId::new();
        mgr.add_zone(square_zone(zid, ZoneKind::SchoolZone, true));

        assert!(mgr.point_in_zone(&zid, 0.5, 0.5));
        assert!(!mgr.point_in_zone(&zid, 2.0, 2.0));
    }

    #[test]
    fn point_in_inactive_zone_returns_false() {
        let mut mgr = CityStateManager::new("TestCity");
        let zid = EntityId::new();
        mgr.add_zone(square_zone(zid, ZoneKind::SchoolZone, false));

        assert!(!mgr.point_in_zone(&zid, 0.5, 0.5));
    }

    #[test]
    fn zones_at_point_multiple() {
        let mut mgr = CityStateManager::new("TestCity");
        let z1 = EntityId::new();
        let z2 = EntityId::new();

        mgr.add_zone(square_zone(z1, ZoneKind::LowEmission, true));
        mgr.add_zone(square_zone(z2, ZoneKind::CongestionCharge, true));

        let zones = mgr.zones_at_point(0.5, 0.5);
        assert_eq!(zones.len(), 2);
    }

    #[test]
    fn zones_by_kind() {
        let mut mgr = CityStateManager::new("TestCity");
        mgr.add_zone(square_zone(EntityId::new(), ZoneKind::LowEmission, true));
        mgr.add_zone(square_zone(EntityId::new(), ZoneKind::SchoolZone, true));

        assert_eq!(mgr.zones_by_kind(ZoneKind::LowEmission).len(), 1);
        assert_eq!(mgr.zones_by_kind(ZoneKind::SchoolZone).len(), 1);
        assert_eq!(mgr.zones_by_kind(ZoneKind::Construction).len(), 0);
    }

    #[test]
    fn effective_speed_limit_takes_minimum() {
        let mut mgr = CityStateManager::new("TestCity");

        let mut z1 = square_zone(EntityId::new(), ZoneKind::SchoolZone, true);
        z1.speed_limit_kmh = Some(30.0);
        let mut z2 = square_zone(EntityId::new(), ZoneKind::Construction, true);
        z2.speed_limit_kmh = Some(20.0);

        mgr.add_zone(z1);
        mgr.add_zone(z2);

        assert_eq!(mgr.effective_speed_limit(0.5, 0.5), Some(20.0));
    }

    #[test]
    fn total_charge_sums_zones() {
        let mut mgr = CityStateManager::new("TestCity");

        let mut z1 = square_zone(EntityId::new(), ZoneKind::CongestionCharge, true);
        z1.charge_amount = Some(5.0);
        let mut z2 = square_zone(EntityId::new(), ZoneKind::LowEmission, true);
        z2.charge_amount = Some(3.0);

        mgr.add_zone(z1);
        mgr.add_zone(z2);

        let total = mgr.total_charge(0.5, 0.5);
        assert!((total - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn district_metrics_crud() {
        let mut mgr = CityStateManager::new("TestCity");
        let did = EntityId::new();

        mgr.update_district_metrics(make_district(did, "Downtown", 0.7, 80.0));
        assert_eq!(mgr.district_count(), 1);
        assert!(mgr.get_district(&did).is_some());
    }

    #[test]
    fn most_congested_district() {
        let mut mgr = CityStateManager::new("TestCity");
        let d1 = EntityId::new();
        let d2 = EntityId::new();

        mgr.update_district_metrics(make_district(d1, "Suburbs", 0.2, 40.0));
        mgr.update_district_metrics(make_district(d2, "Downtown", 0.8, 90.0));

        let worst = mgr.most_congested_district().unwrap();
        assert_eq!(worst.district_id, d2);
    }

    #[test]
    fn aqi_recording_and_average() {
        let mut mgr = CityStateManager::new("TestCity");
        let did = EntityId::new();

        mgr.record_aqi(did, 50.0);
        mgr.record_aqi(did, 70.0);
        mgr.record_aqi(did, 80.0);

        let avg = mgr.average_aqi(&did).unwrap();
        assert!((avg - 200.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn city_dashboard() {
        let mut mgr = CityStateManager::new("TelAviv");
        mgr.add_zone(square_zone(EntityId::new(), ZoneKind::LowEmission, true));
        mgr.add_zone(square_zone(EntityId::new(), ZoneKind::SchoolZone, false));

        let d1 = EntityId::new();
        let d2 = EntityId::new();
        mgr.update_district_metrics(make_district(d1, "North", 0.3, 40.0));
        mgr.update_district_metrics(make_district(d2, "South", 0.7, 100.0));

        let dash = mgr.compute_dashboard();
        assert_eq!(dash.city_name, "TelAviv");
        assert_eq!(dash.total_zones, 2);
        assert_eq!(dash.active_zones, 1);
        assert_eq!(dash.total_districts, 2);
        assert!((dash.avg_congestion - 0.5).abs() < f64::EPSILON);
        assert!((dash.avg_aqi - 70.0).abs() < f64::EPSILON);
        assert_eq!(dash.total_active_incidents, 4);
    }

    #[test]
    fn point_outside_polygon() {
        assert!(!point_in_polygon(
            5.0,
            5.0,
            &[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]
        ));
    }

    #[test]
    fn degenerate_polygon_returns_false() {
        assert!(!point_in_polygon(0.5, 0.5, &[(0.0, 0.0), (1.0, 1.0)]));
    }
}
