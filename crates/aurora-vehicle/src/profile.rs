//! Vehicle profiles — per-vehicle configuration, sensor mapping,
//! performance characteristics, and compatibility management.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Profile types
// ---------------------------------------------------------------------------

/// A vehicle profile describing its configuration and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleProfile {
    pub id: EntityId,
    pub vin: String,
    pub manufacturer_id: EntityId,
    pub model: String,
    pub year: u16,
    pub powertrain: Powertrain,
    pub dimensions: VehicleDimensions,
    pub sensor_map: Vec<SensorMapping>,
    pub performance: PerformanceSpec,
    pub features: Vec<VehicleFeature>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Powertrain type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Powertrain {
    Ice,
    Hybrid,
    PluginHybrid,
    BatteryElectric,
    HydrogenFuelCell,
}

/// Vehicle dimensions for routing and parking constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleDimensions {
    pub length_mm: u32,
    pub width_mm: u32,
    pub height_mm: u32,
    pub wheelbase_mm: u32,
    pub turning_radius_m: f64,
    pub curb_weight_kg: u32,
    pub gross_weight_kg: u32,
}

/// Mapping of a physical sensor to the canonical sensor system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorMapping {
    pub sensor_name: String,
    pub adapter_id: EntityId,
    pub position: SensorPosition,
    pub active: bool,
}

/// Sensor position on the vehicle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorPosition {
    /// Forward offset from rear axle centre (mm).
    pub x_mm: i32,
    /// Lateral offset from centre line (mm, positive = left).
    pub y_mm: i32,
    /// Height above ground (mm).
    pub z_mm: i32,
}

/// Vehicle performance specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSpec {
    pub max_speed_kmh: f64,
    pub acceleration_0_100_s: f64,
    pub braking_100_0_m: f64,
    /// Battery capacity in kWh (EV/PHEV only).
    pub battery_capacity_kwh: Option<f64>,
    /// Fuel tank capacity in litres (ICE/Hybrid only).
    pub fuel_capacity_l: Option<f64>,
    /// Estimated range in km.
    pub estimated_range_km: f64,
}

/// Vehicle feature flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VehicleFeature {
    AdaptiveCruiseControl,
    LaneKeepAssist,
    AutomaticEmergencyBraking,
    BlindSpotDetection,
    ParkingAssist,
    TrafficSignRecognition,
    HeadUpDisplay,
    NightVision,
    SurroundViewCamera,
    V2XCommunication,
    OverTheAirUpdates,
    RemoteStart,
}

// ---------------------------------------------------------------------------
// Profile manager
// ---------------------------------------------------------------------------

/// Manages vehicle profiles.
pub struct ProfileManager {
    profiles: HashMap<EntityId, VehicleProfile>,
    vin_index: HashMap<String, EntityId>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            vin_index: HashMap::new(),
        }
    }

    /// Create a vehicle profile.
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        &mut self,
        vin: impl Into<String>,
        manufacturer_id: EntityId,
        model: impl Into<String>,
        year: u16,
        powertrain: Powertrain,
        dimensions: VehicleDimensions,
        performance: PerformanceSpec,
    ) -> Result<VehicleProfile, ProfileError> {
        let vin_str = vin.into();
        if self.vin_index.contains_key(&vin_str) {
            return Err(ProfileError::VinExists(vin_str));
        }

        let now = Utc::now();
        let profile = VehicleProfile {
            id: EntityId::new(),
            vin: vin_str.clone(),
            manufacturer_id,
            model: model.into(),
            year,
            powertrain,
            dimensions,
            sensor_map: Vec::new(),
            performance,
            features: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        self.vin_index.insert(vin_str, profile.id);
        let result = profile.clone();
        self.profiles.insert(profile.id, profile);
        Ok(result)
    }

    /// Add a sensor mapping to a profile.
    pub fn add_sensor(
        &mut self,
        profile_id: &EntityId,
        sensor_name: impl Into<String>,
        adapter_id: EntityId,
        position: SensorPosition,
    ) -> Result<(), ProfileError> {
        let profile = self
            .profiles
            .get_mut(profile_id)
            .ok_or(ProfileError::NotFound(*profile_id))?;

        profile.sensor_map.push(SensorMapping {
            sensor_name: sensor_name.into(),
            adapter_id,
            position,
            active: true,
        });
        profile.updated_at = Utc::now();
        Ok(())
    }

    /// Add vehicle features.
    pub fn add_features(
        &mut self,
        profile_id: &EntityId,
        features: Vec<VehicleFeature>,
    ) -> Result<(), ProfileError> {
        let profile = self
            .profiles
            .get_mut(profile_id)
            .ok_or(ProfileError::NotFound(*profile_id))?;

        for feat in features {
            if !profile.features.contains(&feat) {
                profile.features.push(feat);
            }
        }
        profile.updated_at = Utc::now();
        Ok(())
    }

    /// Check if a vehicle has a feature.
    pub fn has_feature(
        &self,
        profile_id: &EntityId,
        feature: VehicleFeature,
    ) -> Result<bool, ProfileError> {
        let profile = self
            .profiles
            .get(profile_id)
            .ok_or(ProfileError::NotFound(*profile_id))?;
        Ok(profile.features.contains(&feature))
    }

    /// Get profile by ID.
    pub fn get(&self, profile_id: &EntityId) -> Option<&VehicleProfile> {
        self.profiles.get(profile_id)
    }

    /// Get profile by VIN.
    pub fn get_by_vin(&self, vin: &str) -> Option<&VehicleProfile> {
        self.vin_index.get(vin).and_then(|id| self.profiles.get(id))
    }

    /// List profiles for a manufacturer.
    pub fn for_manufacturer(&self, mfr_id: &EntityId) -> Vec<&VehicleProfile> {
        self.profiles
            .values()
            .filter(|p| p.manufacturer_id == *mfr_id)
            .collect()
    }

    /// Total profiles.
    pub fn total(&self) -> usize {
        self.profiles.len()
    }

    /// Profiles with a specific powertrain.
    pub fn by_powertrain(&self, pt: Powertrain) -> Vec<&VehicleProfile> {
        self.profiles
            .values()
            .filter(|p| p.powertrain == pt)
            .collect()
    }

    /// Check if vehicle fits dimensional constraints.
    pub fn fits_constraints(
        &self,
        profile_id: &EntityId,
        max_height_mm: u32,
        max_width_mm: u32,
        max_weight_kg: u32,
    ) -> Result<bool, ProfileError> {
        let profile = self
            .profiles
            .get(profile_id)
            .ok_or(ProfileError::NotFound(*profile_id))?;

        Ok(profile.dimensions.height_mm <= max_height_mm
            && profile.dimensions.width_mm <= max_width_mm
            && profile.dimensions.gross_weight_kg <= max_weight_kg)
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("profile not found: {0}")]
    NotFound(EntityId),
    #[error("VIN already exists: {0}")]
    VinExists(String),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dims() -> VehicleDimensions {
        VehicleDimensions {
            length_mm: 4694,
            width_mm: 1849,
            height_mm: 1443,
            wheelbase_mm: 2875,
            turning_radius_m: 5.8,
            curb_weight_kg: 1847,
            gross_weight_kg: 2300,
        }
    }

    fn test_perf() -> PerformanceSpec {
        PerformanceSpec {
            max_speed_kmh: 261.0,
            acceleration_0_100_s: 3.1,
            braking_100_0_m: 34.0,
            battery_capacity_kwh: Some(82.0),
            fuel_capacity_l: None,
            estimated_range_km: 510.0,
        }
    }

    fn test_mgr_with_profile() -> (ProfileManager, VehicleProfile) {
        let mut mgr = ProfileManager::new();
        let profile = mgr
            .create(
                "WBA12345678901234",
                EntityId::new(),
                "Model 3",
                2025,
                Powertrain::BatteryElectric,
                test_dims(),
                test_perf(),
            )
            .unwrap();
        (mgr, profile)
    }

    #[test]
    fn create_and_retrieve() {
        let (mgr, profile) = test_mgr_with_profile();
        assert_eq!(profile.model, "Model 3");
        assert_eq!(profile.powertrain, Powertrain::BatteryElectric);
        assert!(mgr.get_by_vin("WBA12345678901234").is_some());
    }

    #[test]
    fn duplicate_vin_rejected() {
        let (mut mgr, _) = test_mgr_with_profile();
        let result = mgr.create(
            "WBA12345678901234",
            EntityId::new(),
            "Duplicate",
            2025,
            Powertrain::Ice,
            test_dims(),
            test_perf(),
        );
        assert!(matches!(result.unwrap_err(), ProfileError::VinExists(_)));
    }

    #[test]
    fn add_sensor() {
        let (mut mgr, profile) = test_mgr_with_profile();
        mgr.add_sensor(
            &profile.id,
            "Front Camera",
            EntityId::new(),
            SensorPosition {
                x_mm: 2800,
                y_mm: 0,
                z_mm: 1200,
            },
        )
        .unwrap();
        assert_eq!(mgr.get(&profile.id).unwrap().sensor_map.len(), 1);
    }

    #[test]
    fn features() {
        let (mut mgr, profile) = test_mgr_with_profile();
        mgr.add_features(
            &profile.id,
            vec![
                VehicleFeature::LaneKeepAssist,
                VehicleFeature::AdaptiveCruiseControl,
            ],
        )
        .unwrap();
        assert!(mgr
            .has_feature(&profile.id, VehicleFeature::LaneKeepAssist)
            .unwrap());
        assert!(!mgr
            .has_feature(&profile.id, VehicleFeature::NightVision)
            .unwrap());
    }

    #[test]
    fn feature_dedup() {
        let (mut mgr, profile) = test_mgr_with_profile();
        mgr.add_features(&profile.id, vec![VehicleFeature::ParkingAssist])
            .unwrap();
        mgr.add_features(&profile.id, vec![VehicleFeature::ParkingAssist])
            .unwrap();
        assert_eq!(mgr.get(&profile.id).unwrap().features.len(), 1);
    }

    #[test]
    fn dimensional_constraints() {
        let (mgr, profile) = test_mgr_with_profile();
        assert!(mgr.fits_constraints(&profile.id, 2000, 2000, 3000).unwrap());
        assert!(!mgr.fits_constraints(&profile.id, 1400, 2000, 3000).unwrap()); // too tall
        assert!(!mgr.fits_constraints(&profile.id, 2000, 1800, 3000).unwrap()); // too wide
        assert!(!mgr.fits_constraints(&profile.id, 2000, 2000, 2000).unwrap()); // too heavy
    }

    #[test]
    fn by_powertrain() {
        let (mut mgr, _) = test_mgr_with_profile();
        mgr.create(
            "VIN2",
            EntityId::new(),
            "Civic",
            2025,
            Powertrain::Ice,
            test_dims(),
            test_perf(),
        )
        .unwrap();
        assert_eq!(mgr.by_powertrain(Powertrain::BatteryElectric).len(), 1);
        assert_eq!(mgr.by_powertrain(Powertrain::Ice).len(), 1);
        assert_eq!(mgr.by_powertrain(Powertrain::Hybrid).len(), 0);
    }

    #[test]
    fn for_manufacturer() {
        let mut mgr = ProfileManager::new();
        let mfr1 = EntityId::new();
        let mfr2 = EntityId::new();
        mgr.create(
            "VIN1",
            mfr1,
            "A",
            2025,
            Powertrain::Ice,
            test_dims(),
            test_perf(),
        )
        .unwrap();
        mgr.create(
            "VIN2",
            mfr1,
            "B",
            2025,
            Powertrain::Ice,
            test_dims(),
            test_perf(),
        )
        .unwrap();
        mgr.create(
            "VIN3",
            mfr2,
            "C",
            2025,
            Powertrain::Ice,
            test_dims(),
            test_perf(),
        )
        .unwrap();
        assert_eq!(mgr.for_manufacturer(&mfr1).len(), 2);
        assert_eq!(mgr.for_manufacturer(&mfr2).len(), 1);
    }

    #[test]
    fn total() {
        let (mgr, _) = test_mgr_with_profile();
        assert_eq!(mgr.total(), 1);
    }
}
