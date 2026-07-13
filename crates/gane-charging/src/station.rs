//! Charging station integration — station registry, availability tracking,
//! congestion prediction, and route-to-charger planning.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// A charging station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingStation {
    pub id: EntityId,
    pub name: String,
    pub operator: String,
    pub position: GeoPosition,
    pub connectors: Vec<Connector>,
    pub amenities: Vec<Amenity>,
    pub is_open_24h: bool,
    pub updated_at: DateTime<Utc>,
}

/// A charging connector at a station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: EntityId,
    pub connector_type: ConnectorType,
    pub max_power_kw: f64,
    pub status: ConnectorStatus,
    pub price_per_kwh: Option<f64>,
}

/// Connector types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorType {
    Type1,
    Type2,
    CCS1,
    CCS2,
    CHAdeMO,
    Tesla,
    GBT,
}

/// Connector availability status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorStatus {
    Available,
    InUse,
    OutOfService,
    Reserved,
    Unknown,
}

/// Station amenities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Amenity {
    Restroom,
    Food,
    Coffee,
    WiFi,
    Shopping,
    Playground,
    Shelter,
    Parking,
}

/// Charging congestion level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongestionLevel {
    /// Plenty of chargers available.
    Low,
    /// Some chargers in use, moderate wait possible.
    Moderate,
    /// Most chargers busy, expect a wait.
    High,
    /// All chargers in use, significant wait.
    Full,
}

/// A charging recommendation for a route.
#[derive(Debug, Clone)]
pub struct ChargingRecommendation {
    pub station_id: EntityId,
    pub station_name: String,
    pub distance_m: f64,
    pub detour_m: f64,
    pub max_power_kw: f64,
    pub estimated_charge_time_min: f64,
    pub estimated_cost: Option<f64>,
    pub congestion: CongestionLevel,
    pub compatible_connectors: usize,
    pub has_amenities: bool,
}

/// Station manager — registry, availability tracking, and recommendations.
pub struct StationManager {
    stations: HashMap<EntityId, ChargingStation>,
    /// Congestion predictions per station.
    congestion_cache: HashMap<EntityId, CongestionLevel>,
    search_radius_m: f64,
}

impl StationManager {
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
            congestion_cache: HashMap::new(),
            search_radius_m: 50_000.0, // 50 km default search
        }
    }

    /// Register a charging station.
    pub fn add_station(&mut self, station: ChargingStation) {
        debug!(name = %station.name, connectors = station.connectors.len(), "charging station registered");
        self.stations.insert(station.id, station);
    }

    /// Get a station by ID.
    pub fn station(&self, id: &EntityId) -> Option<&ChargingStation> {
        self.stations.get(id)
    }

    /// Update a connector's status.
    pub fn update_connector_status(
        &mut self,
        station_id: &EntityId,
        connector_id: &EntityId,
        status: ConnectorStatus,
    ) -> bool {
        if let Some(station) = self.stations.get_mut(station_id) {
            if let Some(conn) = station
                .connectors
                .iter_mut()
                .find(|c| c.id == *connector_id)
            {
                conn.status = status;
                station.updated_at = Utc::now();
                // Invalidate cached congestion so next query recalculates.
                self.congestion_cache.remove(station_id);
                return true;
            }
        }
        false
    }

    /// Find stations near a position with available connectors.
    pub fn find_available_near(
        &self,
        position: &GeoPosition,
        connector_type: Option<ConnectorType>,
    ) -> Vec<&ChargingStation> {
        let mut results: Vec<(&ChargingStation, f64)> = self
            .stations
            .values()
            .filter_map(|s| {
                let dist = haversine_distance(position, &s.position);
                if dist > self.search_radius_m {
                    return None;
                }

                let has_available = s.connectors.iter().any(|c| {
                    c.status == ConnectorStatus::Available
                        && connector_type.map_or(true, |t| c.connector_type == t)
                });

                if has_available {
                    Some((s, dist))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        results.into_iter().map(|(s, _)| s).collect()
    }

    /// Generate charging recommendations for a route.
    pub fn recommend(
        &self,
        current_pos: &GeoPosition,
        destination: &GeoPosition,
        current_range_km: f64,
        connector_type: Option<ConnectorType>,
        energy_needed_kwh: f64,
    ) -> Vec<ChargingRecommendation> {
        let route_distance = haversine_distance(current_pos, destination) / 1000.0;

        // If we can make it without charging, return empty.
        if current_range_km > route_distance * 1.2 {
            return Vec::new();
        }

        let available = self.find_available_near(current_pos, connector_type);

        available
            .iter()
            .map(|station| {
                let dist_to_station = haversine_distance(current_pos, &station.position);
                let station_to_dest = haversine_distance(&station.position, destination);
                let direct_dist = haversine_distance(current_pos, destination);
                let detour = (dist_to_station + station_to_dest - direct_dist).max(0.0);

                let max_power = station
                    .connectors
                    .iter()
                    .filter(|c| {
                        c.status == ConnectorStatus::Available
                            && connector_type.map_or(true, |t| c.connector_type == t)
                    })
                    .map(|c| c.max_power_kw)
                    .fold(0.0f64, f64::max);

                let charge_time_min = if max_power > 0.0 {
                    energy_needed_kwh / max_power * 60.0
                } else {
                    f64::INFINITY
                };

                let min_price = station
                    .connectors
                    .iter()
                    .filter_map(|c| c.price_per_kwh)
                    .fold(f64::INFINITY, f64::min);

                let estimated_cost = if min_price.is_finite() {
                    Some(min_price * energy_needed_kwh)
                } else {
                    None
                };

                let compatible = station
                    .connectors
                    .iter()
                    .filter(|c| {
                        c.status == ConnectorStatus::Available
                            && connector_type.map_or(true, |t| c.connector_type == t)
                    })
                    .count();

                let congestion = self.estimate_congestion(station);

                ChargingRecommendation {
                    station_id: station.id,
                    station_name: station.name.clone(),
                    distance_m: dist_to_station,
                    detour_m: detour,
                    max_power_kw: max_power,
                    estimated_charge_time_min: charge_time_min,
                    estimated_cost,
                    congestion,
                    compatible_connectors: compatible,
                    has_amenities: !station.amenities.is_empty(),
                }
            })
            .collect()
    }

    /// Estimate congestion at a station.
    fn estimate_congestion(&self, station: &ChargingStation) -> CongestionLevel {
        if let Some(&cached) = self.congestion_cache.get(&station.id) {
            return cached;
        }

        let total = station.connectors.len();
        if total == 0 {
            return CongestionLevel::Full;
        }

        let available = station
            .connectors
            .iter()
            .filter(|c| c.status == ConnectorStatus::Available)
            .count();

        let ratio = available as f64 / total as f64;

        if ratio > 0.7 {
            CongestionLevel::Low
        } else if ratio > 0.3 {
            CongestionLevel::Moderate
        } else if ratio > 0.0 {
            CongestionLevel::High
        } else {
            CongestionLevel::Full
        }
    }

    /// Update the congestion prediction for a station.
    pub fn set_congestion(&mut self, station_id: EntityId, level: CongestionLevel) {
        self.congestion_cache.insert(station_id, level);
    }

    /// Get the total station count.
    pub fn station_count(&self) -> usize {
        self.stations.len()
    }

    /// Set the search radius.
    pub fn set_search_radius(&mut self, radius_m: f64) {
        self.search_radius_m = radius_m;
    }
}

impl Default for StationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
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

    fn make_station(name: &str, lat: f64, lon: f64, connector_count: usize) -> ChargingStation {
        let connectors: Vec<Connector> = (0..connector_count)
            .map(|_| Connector {
                id: EntityId::new(),
                connector_type: ConnectorType::CCS2,
                max_power_kw: 150.0,
                status: ConnectorStatus::Available,
                price_per_kwh: Some(0.35),
            })
            .collect();

        ChargingStation {
            id: EntityId::new(),
            name: name.into(),
            operator: "TestOp".into(),
            position: pos(lat, lon),
            connectors,
            amenities: vec![Amenity::Restroom, Amenity::Coffee],
            is_open_24h: true,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn add_and_query_station() {
        let mut mgr = StationManager::new();
        let station = make_station("Station A", 32.0, 34.0, 4);
        let sid = station.id;
        mgr.add_station(station);
        assert_eq!(mgr.station_count(), 1);
        assert!(mgr.station(&sid).is_some());
    }

    #[test]
    fn find_available_near() {
        let mut mgr = StationManager::new();
        mgr.add_station(make_station("Near", 32.001, 34.001, 2));
        mgr.add_station(make_station("Far", 40.0, 40.0, 2));

        let results = mgr.find_available_near(&pos(32.0, 34.0), None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Near");
    }

    #[test]
    fn find_by_connector_type() {
        let mut mgr = StationManager::new();
        let mut station = make_station("Station", 32.001, 34.0, 2);
        station.connectors[0].connector_type = ConnectorType::CHAdeMO;
        station.connectors[1].connector_type = ConnectorType::CCS2;
        mgr.add_station(station);

        let ccs = mgr.find_available_near(&pos(32.0, 34.0), Some(ConnectorType::CCS2));
        assert_eq!(ccs.len(), 1);

        let type1 = mgr.find_available_near(&pos(32.0, 34.0), Some(ConnectorType::Type1));
        assert!(type1.is_empty());
    }

    #[test]
    fn update_connector_status() {
        let mut mgr = StationManager::new();
        let station = make_station("Station", 32.001, 34.0, 2);
        let sid = station.id;
        let cid = station.connectors[0].id;
        mgr.add_station(station);

        assert!(mgr.update_connector_status(&sid, &cid, ConnectorStatus::InUse));
        let s = mgr.station(&sid).unwrap();
        assert_eq!(s.connectors[0].status, ConnectorStatus::InUse);
    }

    #[test]
    fn congestion_estimation() {
        let mut mgr = StationManager::new();
        let mut station = make_station("Busy", 32.001, 34.0, 4);
        // Make 3 of 4 in use.
        station.connectors[0].status = ConnectorStatus::InUse;
        station.connectors[1].status = ConnectorStatus::InUse;
        station.connectors[2].status = ConnectorStatus::InUse;
        let sid = station.id;
        mgr.add_station(station);

        let s = mgr.station(&sid).unwrap();
        let congestion = mgr.estimate_congestion(s);
        assert_eq!(congestion, CongestionLevel::High);
    }

    #[test]
    fn recommend_when_range_sufficient() {
        let mut mgr = StationManager::new();
        mgr.add_station(make_station("Station", 32.001, 34.0, 2));

        // Plenty of range — no recommendation.
        let recs = mgr.recommend(
            &pos(32.0, 34.0),
            &pos(32.01, 34.01),
            500.0, // 500 km range
            None,
            0.0,
        );
        assert!(recs.is_empty());
    }

    #[test]
    fn recommend_when_range_insufficient() {
        let mut mgr = StationManager::new();
        mgr.add_station(make_station("Station", 32.005, 34.005, 2));

        // Low range — should recommend.
        let recs = mgr.recommend(
            &pos(32.0, 34.0),
            &pos(32.1, 34.1),
            5.0, // Only 5 km range
            None,
            30.0, // Need 30 kWh
        );
        assert!(!recs.is_empty());
        assert!(recs[0].estimated_charge_time_min > 0.0);
    }

    #[test]
    fn recommendation_has_cost() {
        let mut mgr = StationManager::new();
        mgr.add_station(make_station("Station", 32.005, 34.005, 2));

        let recs = mgr.recommend(&pos(32.0, 34.0), &pos(32.1, 34.1), 5.0, None, 30.0);
        if !recs.is_empty() {
            assert!(recs[0].estimated_cost.is_some());
            assert!(recs[0].estimated_cost.unwrap() > 0.0);
        }
    }

    #[test]
    fn set_congestion_cache() {
        let mut mgr = StationManager::new();
        let station = make_station("Station", 32.0, 34.0, 4);
        let sid = station.id;
        mgr.add_station(station);

        mgr.set_congestion(sid, CongestionLevel::Full);
        let s = mgr.station(&sid).unwrap();
        // The cached congestion should be used.
        let congestion = mgr.estimate_congestion(s);
        assert_eq!(congestion, CongestionLevel::Full);
    }

    #[test]
    fn adversarial_cache_invalidated_on_connector_update() {
        let mut mgr = StationManager::new();
        let station = make_station("CacheTest", 32.0, 34.0, 2);
        let sid = station.id;
        let conn_ids: Vec<EntityId> = station.connectors.iter().map(|c| c.id).collect();
        mgr.add_station(station);

        // Step 1: Set congestion cache to Low
        mgr.set_congestion(sid, CongestionLevel::Low);
        let s = mgr.station(&sid).unwrap();
        assert_eq!(mgr.estimate_congestion(s), CongestionLevel::Low);

        // Step 2: Mark all connectors InUse — this should invalidate the cache
        for cid in &conn_ids {
            assert!(mgr.update_connector_status(&sid, cid, ConnectorStatus::InUse));
        }

        // Step 3: Cache should be invalidated, recalculate → Full
        let s = mgr.station(&sid).unwrap();
        let congestion = mgr.estimate_congestion(s);
        assert_eq!(
            congestion,
            CongestionLevel::Full,
            "BUG: Expected Full after all connectors InUse, got {:?} (stale cache!)",
            congestion
        );
    }
}
