//! Vehicle energy model — consumption prediction, battery range estimation,
//! and eco/fast/stable route scoring.

use aurora_core::types::{EnergyType, EntityId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Vehicle energy profile — describes the energy characteristics of a vehicle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub id: EntityId,
    pub vehicle_id: EntityId,
    pub energy_type: EnergyType,
    /// Battery capacity in kWh (for EVs/hybrids).
    pub battery_capacity_kwh: Option<f64>,
    /// Current state of charge [0, 1] (for EVs/hybrids).
    pub state_of_charge: Option<f64>,
    /// Fuel tank capacity in litres (for ICE/hybrid).
    pub fuel_capacity_l: Option<f64>,
    /// Current fuel level [0, 1].
    pub fuel_level: Option<f64>,
    /// Average consumption (kWh/100km or L/100km).
    pub avg_consumption_per_100km: f64,
    /// Vehicle mass in kg (affects consumption).
    pub vehicle_mass_kg: f64,
    /// Drag coefficient.
    pub drag_coefficient: f64,
    /// Frontal area in m².
    pub frontal_area_m2: f64,
    /// Regenerative braking efficiency [0, 1] (EVs only).
    pub regen_efficiency: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

/// Factors that affect energy consumption on a route segment.
#[derive(Debug, Clone, Copy)]
pub struct ConsumptionFactors {
    /// Distance in metres.
    pub distance_m: f64,
    /// Average speed in m/s.
    pub avg_speed_mps: f64,
    /// Elevation change in metres (positive = uphill).
    pub elevation_change_m: f64,
    /// Ambient temperature in °C.
    pub temperature_c: f64,
    /// Whether HVAC is running.
    pub hvac_active: bool,
    /// Traffic stop count (affects regen and idle loss).
    pub stop_count: u32,
    /// Road surface quality [0, 1] (1 = smooth).
    pub road_quality: f64,
}

/// Energy consumption estimate for a route or segment.
#[derive(Debug, Clone)]
pub struct ConsumptionEstimate {
    /// Estimated energy use in kWh (for EVs) or litres (for ICE).
    pub energy_used: f64,
    /// Remaining energy after this route (kWh or litres).
    pub remaining_energy: f64,
    /// Remaining range in km.
    pub remaining_range_km: f64,
    /// Whether the vehicle can complete this route without charging/refuelling.
    pub can_complete: bool,
    /// Estimated remaining SoC or fuel level [0, 1] after route.
    pub remaining_level: f64,
    /// Environmental impact score [0, 1] (lower = better).
    pub environmental_score: f64,
}

/// Route energy scoring for multi-objective optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteEnergyStrategy {
    /// Minimize energy consumption.
    Eco,
    /// Minimize time regardless of energy.
    Fastest,
    /// Balance time and energy.
    Balanced,
    /// Ensure maximum remaining range at destination.
    MaxRange,
    /// Minimize environmental impact (CO₂).
    Green,
}

/// Energy model engine — estimates consumption and scores routes.
pub struct EnergyModel {
    profiles: Vec<EnergyProfile>,
    /// CO₂ emission factor for electric vehicles (g/kWh for grid electricity).
    co2_factor_electric: f64,
    /// CO₂ emission factor for gasoline vehicles (g/L).
    co2_factor_gasoline: f64,
    /// CO₂ emission factor for diesel vehicles (g/L).
    co2_factor_diesel: f64,
}

impl EnergyModel {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            co2_factor_electric: 400.0,   // Average grid: 400 g CO₂/kWh
            co2_factor_gasoline: 2_310.0, // ~2310 g CO₂/litre gasoline
            co2_factor_diesel: 2_680.0,   // ~2680 g CO₂/litre diesel
        }
    }

    /// Register an energy profile.
    pub fn add_profile(&mut self, profile: EnergyProfile) {
        debug!(
            vehicle = %profile.vehicle_id,
            energy_type = ?profile.energy_type,
            "energy profile registered"
        );
        self.profiles.push(profile);
    }

    /// Get a profile by vehicle ID.
    pub fn profile_for_vehicle(&self, vehicle_id: &EntityId) -> Option<&EnergyProfile> {
        self.profiles.iter().find(|p| p.vehicle_id == *vehicle_id)
    }

    /// Estimate energy consumption for a route segment.
    pub fn estimate_consumption(
        &self,
        profile: &EnergyProfile,
        factors: &ConsumptionFactors,
    ) -> ConsumptionEstimate {
        let distance_km = factors.distance_m / 1000.0;

        // Base consumption from average rate.
        let mut consumption = profile.avg_consumption_per_100km * distance_km / 100.0;

        // Speed factor — consumption increases at high and very low speeds.
        let speed_kmh = factors.avg_speed_mps * 3.6;
        let speed_factor = if speed_kmh < 20.0 {
            1.3 // City crawl — inefficient
        } else if speed_kmh < 50.0 {
            1.0 // Optimal
        } else if speed_kmh < 80.0 {
            1.1 // Moderate highway
        } else if speed_kmh < 120.0 {
            1.3 // High speed — aero drag
        } else {
            1.6 // Very high speed
        };
        consumption *= speed_factor;

        // Elevation factor.
        if factors.elevation_change_m > 0.0 {
            // Uphill: extra energy = m * g * h / efficiency
            let gravity_energy_kwh =
                profile.vehicle_mass_kg * 9.81 * factors.elevation_change_m / (3_600_000.0 * 0.85);
            consumption += gravity_energy_kwh;
        } else if factors.elevation_change_m < 0.0 {
            // Downhill: recover some energy via regen.
            let regen = profile.regen_efficiency.unwrap_or(0.0);
            let recovery_kwh = profile.vehicle_mass_kg * 9.81 * factors.elevation_change_m.abs()
                / (3_600_000.0)
                * regen;
            consumption -= recovery_kwh;
        }

        // Temperature factor (HVAC load).
        if factors.hvac_active {
            let temp_penalty = if factors.temperature_c < 0.0 {
                0.3 // Heating in cold weather.
            } else if factors.temperature_c > 35.0 {
                0.2 // AC in hot weather.
            } else {
                0.05 // Mild climate, minimal HVAC.
            };
            consumption *= 1.0 + temp_penalty;
        }

        // Road quality factor.
        let road_factor = 1.0 + (1.0 - factors.road_quality) * 0.15;
        consumption *= road_factor;

        // Stop penalty (idle + acceleration from stops).
        let stop_penalty = factors.stop_count as f64 * 0.02; // kWh per stop
        consumption += stop_penalty;

        consumption = consumption.max(0.0);

        // Calculate remaining energy.
        let current_energy = match profile.energy_type {
            EnergyType::Electric | EnergyType::Hybrid => {
                let cap = profile.battery_capacity_kwh.unwrap_or(60.0);
                let soc = profile.state_of_charge.unwrap_or(0.5);
                cap * soc
            }
            _ => {
                let cap = profile.fuel_capacity_l.unwrap_or(50.0);
                let level = profile.fuel_level.unwrap_or(0.5);
                cap * level
            }
        };

        let remaining = (current_energy - consumption).max(0.0);
        let total_capacity = match profile.energy_type {
            EnergyType::Electric | EnergyType::Hybrid => {
                profile.battery_capacity_kwh.unwrap_or(60.0)
            }
            _ => profile.fuel_capacity_l.unwrap_or(50.0),
        };

        let remaining_level = if total_capacity > 0.0 {
            remaining / total_capacity
        } else {
            0.0
        };

        let remaining_range = if profile.avg_consumption_per_100km > 0.0 {
            remaining / profile.avg_consumption_per_100km * 100.0
        } else {
            0.0
        };

        // Environmental score (0 = clean, 1 = dirty).
        let co2_factor = match profile.energy_type {
            EnergyType::Electric => self.co2_factor_electric,
            EnergyType::Hybrid => self.co2_factor_electric, // Simplified: use electric factor
            EnergyType::Diesel => self.co2_factor_diesel,
            _ => self.co2_factor_gasoline, // Gasoline, Hydrogen, Other
        };
        let co2_grams = consumption * co2_factor;
        let environmental_score = (co2_grams / (distance_km.max(0.1) * 200.0)).clamp(0.0, 1.0);

        ConsumptionEstimate {
            energy_used: consumption,
            remaining_energy: remaining,
            remaining_range_km: remaining_range,
            can_complete: remaining > 0.0,
            remaining_level,
            environmental_score,
        }
    }

    /// Score a route for a given energy strategy.
    pub fn score_route(
        &self,
        estimate: &ConsumptionEstimate,
        duration_s: f64,
        strategy: RouteEnergyStrategy,
    ) -> f64 {
        match strategy {
            RouteEnergyStrategy::Eco => {
                // Lower consumption = higher score.
                1.0 / (1.0 + estimate.energy_used)
            }
            RouteEnergyStrategy::Fastest => {
                // Lower duration = higher score (energy not considered).
                1.0 / (1.0 + duration_s / 60.0)
            }
            RouteEnergyStrategy::Balanced => {
                // Balance time and energy.
                let time_score = 1.0 / (1.0 + duration_s / 60.0);
                let energy_score = 1.0 / (1.0 + estimate.energy_used);
                0.5 * time_score + 0.5 * energy_score
            }
            RouteEnergyStrategy::MaxRange => {
                // Higher remaining range = higher score.
                estimate.remaining_range_km / 500.0 // Normalize to ~500km max
            }
            RouteEnergyStrategy::Green => {
                // Lower environmental score = higher route score.
                1.0 - estimate.environmental_score
            }
        }
    }

    /// Estimate the remaining range for a vehicle profile.
    pub fn remaining_range_km(&self, profile: &EnergyProfile) -> f64 {
        let current_energy = match profile.energy_type {
            EnergyType::Electric | EnergyType::Hybrid => {
                let cap = profile.battery_capacity_kwh.unwrap_or(60.0);
                let soc = profile.state_of_charge.unwrap_or(0.5);
                cap * soc
            }
            _ => {
                let cap = profile.fuel_capacity_l.unwrap_or(50.0);
                let level = profile.fuel_level.unwrap_or(0.5);
                cap * level
            }
        };

        if profile.avg_consumption_per_100km > 0.0 {
            current_energy / profile.avg_consumption_per_100km * 100.0
        } else {
            0.0
        }
    }

    /// Get the number of registered profiles.
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }
}

impl Default for EnergyModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev_profile() -> EnergyProfile {
        EnergyProfile {
            id: EntityId::new(),
            vehicle_id: EntityId::new(),
            energy_type: EnergyType::Electric,
            battery_capacity_kwh: Some(75.0),
            state_of_charge: Some(0.8),
            fuel_capacity_l: None,
            fuel_level: None,
            avg_consumption_per_100km: 18.0, // 18 kWh/100km
            vehicle_mass_kg: 2000.0,
            drag_coefficient: 0.24,
            frontal_area_m2: 2.3,
            regen_efficiency: Some(0.7),
            updated_at: Utc::now(),
        }
    }

    fn ice_profile() -> EnergyProfile {
        EnergyProfile {
            id: EntityId::new(),
            vehicle_id: EntityId::new(),
            energy_type: EnergyType::Gasoline,
            battery_capacity_kwh: None,
            state_of_charge: None,
            fuel_capacity_l: Some(50.0),
            fuel_level: Some(0.6),
            avg_consumption_per_100km: 7.0, // 7 L/100km
            vehicle_mass_kg: 1500.0,
            drag_coefficient: 0.30,
            frontal_area_m2: 2.2,
            regen_efficiency: None,
            updated_at: Utc::now(),
        }
    }

    fn flat_highway_factors() -> ConsumptionFactors {
        ConsumptionFactors {
            distance_m: 50_000.0, // 50 km
            avg_speed_mps: 27.8,  // 100 km/h
            elevation_change_m: 0.0,
            temperature_c: 20.0,
            hvac_active: false,
            stop_count: 0,
            road_quality: 0.9,
        }
    }

    #[test]
    fn ev_consumption_estimate() {
        let model = EnergyModel::new();
        let profile = ev_profile();
        let factors = flat_highway_factors();

        let estimate = model.estimate_consumption(&profile, &factors);
        assert!(estimate.energy_used > 0.0);
        assert!(estimate.can_complete);
        assert!(estimate.remaining_range_km > 0.0);
    }

    #[test]
    fn uphill_costs_more_energy() {
        let model = EnergyModel::new();
        let profile = ev_profile();

        let flat = ConsumptionFactors {
            elevation_change_m: 0.0,
            ..flat_highway_factors()
        };
        let uphill = ConsumptionFactors {
            elevation_change_m: 500.0,
            ..flat_highway_factors()
        };

        let flat_est = model.estimate_consumption(&profile, &flat);
        let uphill_est = model.estimate_consumption(&profile, &uphill);
        assert!(uphill_est.energy_used > flat_est.energy_used);
    }

    #[test]
    fn downhill_recovers_energy_with_regen() {
        let model = EnergyModel::new();
        let profile = ev_profile();

        let flat = ConsumptionFactors {
            elevation_change_m: 0.0,
            ..flat_highway_factors()
        };
        let downhill = ConsumptionFactors {
            elevation_change_m: -300.0,
            ..flat_highway_factors()
        };

        let flat_est = model.estimate_consumption(&profile, &flat);
        let downhill_est = model.estimate_consumption(&profile, &downhill);
        assert!(downhill_est.energy_used < flat_est.energy_used);
    }

    #[test]
    fn cold_weather_hvac_penalty() {
        let model = EnergyModel::new();
        let profile = ev_profile();

        let mild = ConsumptionFactors {
            temperature_c: 20.0,
            hvac_active: false,
            ..flat_highway_factors()
        };
        let cold_hvac = ConsumptionFactors {
            temperature_c: -10.0,
            hvac_active: true,
            ..flat_highway_factors()
        };

        let mild_est = model.estimate_consumption(&profile, &mild);
        let cold_est = model.estimate_consumption(&profile, &cold_hvac);
        assert!(cold_est.energy_used > mild_est.energy_used);
    }

    #[test]
    fn ice_vehicle_consumption() {
        let model = EnergyModel::new();
        let profile = ice_profile();
        let factors = flat_highway_factors();

        let estimate = model.estimate_consumption(&profile, &factors);
        assert!(estimate.energy_used > 0.0);
        assert!(estimate.can_complete);
    }

    #[test]
    fn remaining_range_calculation() {
        let model = EnergyModel::new();
        let profile = ev_profile();
        // 75 kWh * 0.8 SoC = 60 kWh / 18 per 100km * 100 = 333 km
        let range = model.remaining_range_km(&profile);
        assert!(range > 300.0);
        assert!(range < 400.0);
    }

    #[test]
    fn cannot_complete_long_route() {
        let model = EnergyModel::new();
        let mut profile = ev_profile();
        profile.state_of_charge = Some(0.1); // Very low battery

        let factors = ConsumptionFactors {
            distance_m: 200_000.0, // 200 km
            ..flat_highway_factors()
        };

        let estimate = model.estimate_consumption(&profile, &factors);
        assert!(!estimate.can_complete);
    }

    #[test]
    fn route_scoring_strategies() {
        let model = EnergyModel::new();
        let profile = ev_profile();
        let factors = flat_highway_factors();
        let estimate = model.estimate_consumption(&profile, &factors);

        let eco_score = model.score_route(&estimate, 1800.0, RouteEnergyStrategy::Eco);
        let fast_score = model.score_route(&estimate, 1800.0, RouteEnergyStrategy::Fastest);
        let balanced = model.score_route(&estimate, 1800.0, RouteEnergyStrategy::Balanced);
        let green = model.score_route(&estimate, 1800.0, RouteEnergyStrategy::Green);

        assert!(eco_score > 0.0);
        assert!(fast_score > 0.0);
        assert!(balanced > 0.0);
        assert!(green > 0.0);
    }

    #[test]
    fn profile_registration() {
        let mut model = EnergyModel::new();
        let profile = ev_profile();
        let vid = profile.vehicle_id;
        model.add_profile(profile);
        assert_eq!(model.profile_count(), 1);
        assert!(model.profile_for_vehicle(&vid).is_some());
    }

    #[test]
    fn stops_increase_consumption() {
        let model = EnergyModel::new();
        let profile = ev_profile();

        let no_stops = ConsumptionFactors {
            stop_count: 0,
            ..flat_highway_factors()
        };
        let many_stops = ConsumptionFactors {
            stop_count: 20,
            ..flat_highway_factors()
        };

        let no_stop_est = model.estimate_consumption(&profile, &no_stops);
        let stop_est = model.estimate_consumption(&profile, &many_stops);
        assert!(stop_est.energy_used > no_stop_est.energy_used);
    }
}
