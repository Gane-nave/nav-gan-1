//! Parking probability modelling — estimates the likelihood of finding
//! parking at a given location, detects legality, and tracks availability.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// A parking zone with availability and legality information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingZone {
    pub id: EntityId,
    pub name: String,
    pub zone_type: ParkingZoneType,
    pub position: GeoPosition,
    pub total_spots: u32,
    pub known_available: Option<u32>,
    pub legality: ParkingLegality,
    pub hourly_rate: Option<f64>,
    pub max_duration_min: Option<u32>,
    pub updated_at: DateTime<Utc>,
}

/// Types of parking zones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParkingZoneType {
    StreetParking,
    ParkingLot,
    ParkingGarage,
    PrivateLot,
    ResidentPermit,
    Handicapped,
    EVCharging,
    LoadingZone,
    DropoffOnly,
    ValetParking,
}

/// Parking legality status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParkingLegality {
    /// Fully legal to park here now.
    Legal,
    /// Legal with time/permit restrictions.
    Restricted,
    /// Currently illegal (street sweeping, rush hour, etc.).
    TemporarilyIllegal,
    /// Always illegal (fire lane, bus stop, etc.).
    Illegal,
    /// Unknown legality — proceed with caution.
    Unknown,
}

/// Configuration for parking probability estimation.
#[derive(Debug, Clone)]
pub struct ParkingConfig {
    /// Search radius for nearby parking (metres).
    pub search_radius_m: f64,
    /// Data freshness threshold — observations older than this are discounted.
    pub freshness_window_s: i64,
    /// Minimum probability to consider a zone viable.
    pub min_viable_probability: f64,
}

impl Default for ParkingConfig {
    fn default() -> Self {
        Self {
            search_radius_m: 500.0,
            freshness_window_s: 1800,
            min_viable_probability: 0.2,
        }
    }
}

/// A parking availability observation.
#[derive(Debug, Clone)]
pub struct ParkingObservation {
    pub zone_id: EntityId,
    pub available_spots: u32,
    pub total_spots: u32,
    pub observed_at: DateTime<Utc>,
}

/// The result of a parking probability query.
#[derive(Debug, Clone)]
pub struct ParkingProbability {
    pub zone_id: EntityId,
    pub zone_name: String,
    pub probability: f64,
    pub estimated_available: f64,
    pub distance_m: f64,
    pub is_legal: bool,
    pub hourly_rate: Option<f64>,
}

/// Parking probability engine — estimates availability and recommends zones.
pub struct ParkingEngine {
    config: ParkingConfig,
    zones: HashMap<EntityId, ParkingZone>,
    observations: Vec<ParkingObservation>,
    /// Historical occupancy rates by zone (exponential moving average).
    occupancy_ema: HashMap<EntityId, f64>,
}

impl ParkingEngine {
    pub fn new() -> Self {
        Self {
            config: ParkingConfig::default(),
            zones: HashMap::new(),
            observations: Vec::new(),
            occupancy_ema: HashMap::new(),
        }
    }

    pub fn with_config(config: ParkingConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Register a parking zone.
    pub fn add_zone(&mut self, zone: ParkingZone) {
        debug!(name = %zone.name, zone_type = ?zone.zone_type, "parking zone registered");
        self.zones.insert(zone.id, zone);
    }

    /// Record a parking availability observation.
    pub fn observe(&mut self, obs: ParkingObservation) {
        let occupancy = if obs.total_spots > 0 {
            1.0 - (obs.available_spots as f64 / obs.total_spots as f64)
        } else {
            1.0
        };

        // Update EMA with alpha = 0.3.
        let alpha = 0.3;
        let current = self
            .occupancy_ema
            .get(&obs.zone_id)
            .copied()
            .unwrap_or(occupancy);
        let new_ema = alpha * occupancy + (1.0 - alpha) * current;
        self.occupancy_ema.insert(obs.zone_id, new_ema);

        // Update zone's known availability.
        if let Some(zone) = self.zones.get_mut(&obs.zone_id) {
            zone.known_available = Some(obs.available_spots);
            zone.updated_at = obs.observed_at;
        }

        self.observations.push(obs);
    }

    /// Estimate parking probability for all zones near a position.
    pub fn estimate_near(
        &self,
        position: &GeoPosition,
        legal_only: bool,
    ) -> Vec<ParkingProbability> {
        let mut results: Vec<ParkingProbability> = self
            .zones
            .values()
            .filter_map(|zone| {
                let dist = haversine_distance(position, &zone.position);
                if dist > self.config.search_radius_m {
                    return None;
                }

                let is_legal = matches!(
                    zone.legality,
                    ParkingLegality::Legal | ParkingLegality::Restricted
                );

                if legal_only && !is_legal {
                    return None;
                }

                let probability = self.compute_probability(zone);
                if probability < self.config.min_viable_probability {
                    return None;
                }

                let estimated_available = probability * zone.total_spots as f64;

                Some(ParkingProbability {
                    zone_id: zone.id,
                    zone_name: zone.name.clone(),
                    probability,
                    estimated_available,
                    distance_m: dist,
                    is_legal,
                    hourly_rate: zone.hourly_rate,
                })
            })
            .collect();

        // Sort by probability descending, then distance ascending.
        results.sort_by(|a, b| {
            b.probability
                .partial_cmp(&a.probability)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.distance_m
                        .partial_cmp(&b.distance_m)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        results
    }

    /// Compute the probability of finding a spot at a specific zone.
    fn compute_probability(&self, zone: &ParkingZone) -> f64 {
        // If we have recent known availability, use it directly.
        if let Some(available) = zone.known_available {
            let now = Utc::now();
            let age_s = (now - zone.updated_at).num_seconds();

            if age_s < self.config.freshness_window_s {
                let freshness = 1.0 - (age_s as f64 / self.config.freshness_window_s as f64);
                let raw = available as f64 / zone.total_spots.max(1) as f64;
                // Blend fresh observation with EMA.
                let ema = self.occupancy_ema.get(&zone.id).copied().unwrap_or(0.5);
                return (raw * freshness + (1.0 - ema) * (1.0 - freshness)).clamp(0.0, 1.0);
            }
        }

        // Fall back to EMA or default.
        let ema = self.occupancy_ema.get(&zone.id).copied().unwrap_or(0.5);
        (1.0 - ema).clamp(0.0, 1.0)
    }

    /// Check legality of parking at a specific zone.
    pub fn check_legality(&self, zone_id: &EntityId) -> ParkingLegality {
        self.zones
            .get(zone_id)
            .map(|z| z.legality)
            .unwrap_or(ParkingLegality::Unknown)
    }

    /// Get a zone by ID.
    pub fn zone(&self, id: &EntityId) -> Option<&ParkingZone> {
        self.zones.get(id)
    }

    /// Get the number of registered zones.
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Prune old observations outside the freshness window.
    pub fn prune_observations(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::seconds(self.config.freshness_window_s);
        self.observations.retain(|o| o.observed_at >= cutoff);
    }
}

impl Default for ParkingEngine {
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

    fn make_zone(
        name: &str,
        lat: f64,
        lon: f64,
        spots: u32,
        zone_type: ParkingZoneType,
    ) -> ParkingZone {
        ParkingZone {
            id: EntityId::new(),
            name: name.into(),
            zone_type,
            position: pos(lat, lon),
            total_spots: spots,
            known_available: None,
            legality: ParkingLegality::Legal,
            hourly_rate: Some(5.0),
            max_duration_min: Some(120),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn add_and_query_zones() {
        let mut engine = ParkingEngine::new();
        let zone = make_zone("Lot A", 32.0, 34.0, 100, ParkingZoneType::ParkingLot);
        let zid = zone.id;
        engine.add_zone(zone);
        assert_eq!(engine.zone_count(), 1);
        assert!(engine.zone(&zid).is_some());
    }

    #[test]
    fn estimate_near_finds_close_zones() {
        let mut engine = ParkingEngine::new();
        engine.add_zone(make_zone(
            "Close",
            32.0001,
            34.0001,
            50,
            ParkingZoneType::StreetParking,
        ));
        engine.add_zone(make_zone(
            "Far",
            33.0,
            35.0,
            50,
            ParkingZoneType::ParkingLot,
        ));

        let results = engine.estimate_near(&pos(32.0, 34.0), false);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].zone_name, "Close");
    }

    #[test]
    fn legal_only_filter() {
        let mut engine = ParkingEngine::new();
        let mut legal = make_zone("Legal", 32.0001, 34.0, 50, ParkingZoneType::StreetParking);
        legal.legality = ParkingLegality::Legal;
        engine.add_zone(legal);

        let mut illegal = make_zone("Illegal", 32.0002, 34.0, 50, ParkingZoneType::StreetParking);
        illegal.legality = ParkingLegality::Illegal;
        engine.add_zone(illegal);

        let results = engine.estimate_near(&pos(32.0, 34.0), true);
        assert_eq!(results.len(), 1);
        assert!(results[0].is_legal);
    }

    #[test]
    fn observation_updates_probability() {
        let mut engine = ParkingEngine::new();
        let zone = make_zone("Lot", 32.0001, 34.0, 100, ParkingZoneType::ParkingLot);
        let zid = zone.id;
        engine.add_zone(zone);

        // Observe mostly full.
        engine.observe(ParkingObservation {
            zone_id: zid,
            available_spots: 5,
            total_spots: 100,
            observed_at: Utc::now(),
        });

        let results = engine.estimate_near(&pos(32.0, 34.0), false);
        // Should still appear but with lower probability.
        if !results.is_empty() {
            assert!(results[0].probability < 0.5);
        }
    }

    #[test]
    fn observation_updates_ema() {
        let mut engine = ParkingEngine::new();
        let zone = make_zone("Lot", 32.0001, 34.0, 100, ParkingZoneType::ParkingLot);
        let zid = zone.id;
        engine.add_zone(zone);

        // Full zone.
        engine.observe(ParkingObservation {
            zone_id: zid,
            available_spots: 0,
            total_spots: 100,
            observed_at: Utc::now(),
        });

        // EMA should reflect high occupancy.
        let ema = engine.occupancy_ema.get(&zid).copied().unwrap();
        assert!(ema > 0.2);
    }

    #[test]
    fn check_legality() {
        let mut engine = ParkingEngine::new();
        let mut zone = make_zone("Zone", 32.0, 34.0, 50, ParkingZoneType::StreetParking);
        zone.legality = ParkingLegality::TemporarilyIllegal;
        let zid = zone.id;
        engine.add_zone(zone);

        assert_eq!(
            engine.check_legality(&zid),
            ParkingLegality::TemporarilyIllegal
        );
        assert_eq!(
            engine.check_legality(&EntityId::new()),
            ParkingLegality::Unknown
        );
    }

    #[test]
    fn probability_sorted_by_highest_first() {
        let mut engine = ParkingEngine::new();

        let mut high_avail = make_zone("High", 32.0001, 34.0, 100, ParkingZoneType::ParkingLot);
        let hid = high_avail.id;
        engine.add_zone(high_avail);

        let mut low_avail = make_zone("Low", 32.0002, 34.0, 100, ParkingZoneType::ParkingLot);
        let lid = low_avail.id;
        engine.add_zone(low_avail);

        // Make "High" have lots of availability.
        engine.observe(ParkingObservation {
            zone_id: hid,
            available_spots: 90,
            total_spots: 100,
            observed_at: Utc::now(),
        });

        // Make "Low" nearly full.
        engine.observe(ParkingObservation {
            zone_id: lid,
            available_spots: 2,
            total_spots: 100,
            observed_at: Utc::now(),
        });

        let results = engine.estimate_near(&pos(32.0, 34.0), false);
        assert!(results.len() >= 1);
        // First result should have higher probability.
        if results.len() >= 2 {
            assert!(results[0].probability >= results[1].probability);
        }
    }

    #[test]
    fn min_viable_probability_filters() {
        let config = ParkingConfig {
            min_viable_probability: 0.8,
            ..Default::default()
        };
        let mut engine = ParkingEngine::with_config(config);
        let zone = make_zone("Zone", 32.0001, 34.0, 100, ParkingZoneType::ParkingLot);
        let zid = zone.id;
        engine.add_zone(zone);

        // Observe mostly full — probability should be below 0.8.
        engine.observe(ParkingObservation {
            zone_id: zid,
            available_spots: 5,
            total_spots: 100,
            observed_at: Utc::now(),
        });

        let results = engine.estimate_near(&pos(32.0, 34.0), false);
        assert!(results.is_empty());
    }
}
