//! Sensor adapters — bridge between OEM-specific sensor formats
//! and AURORA's canonical sensor types, with calibration support.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Adapter types
// ---------------------------------------------------------------------------

/// A sensor adapter that translates OEM sensor data to canonical format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorAdapter {
    pub id: EntityId,
    pub name: String,
    pub manufacturer_id: EntityId,
    pub sensor_type: SensorType,
    pub input_format: DataFormat,
    pub calibration: Calibration,
    pub status: AdapterStatus,
    pub samples_processed: u64,
    pub errors: u64,
    pub created_at: DateTime<Utc>,
}

/// Sensor type that this adapter handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensorType {
    /// Inertial measurement unit.
    Imu,
    /// Wheel speed / odometry.
    WheelSpeed,
    /// Steering angle.
    SteeringAngle,
    /// GNSS receiver.
    Gnss,
    /// Camera (for lane detection).
    Camera,
    /// Lidar.
    Lidar,
    /// Radar.
    Radar,
    /// Ultrasonic.
    Ultrasonic,
    /// Tire pressure monitoring.
    TirePressure,
    /// Ambient temperature.
    Temperature,
}

/// Raw data format from OEM sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataFormat {
    /// CAN bus frames.
    CanBus,
    /// JSON over Ethernet.
    JsonEthernet,
    /// Protobuf over Ethernet.
    ProtobufEthernet,
    /// Raw binary stream.
    RawBinary,
    /// NMEA sentences (GNSS).
    Nmea,
    /// Proprietary format.
    Proprietary,
}

/// Calibration parameters for sensor data conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calibration {
    /// Scale factor (raw * scale = SI value).
    pub scale: f64,
    /// Offset (raw * scale + offset = SI value).
    pub offset: f64,
    /// Axis mapping: [source_x, source_y, source_z] → canonical [x, y, z].
    pub axis_map: [i8; 3],
    /// Last calibrated timestamp.
    pub calibrated_at: DateTime<Utc>,
}

impl Default for Calibration {
    fn default() -> Self {
        Self {
            scale: 1.0,
            offset: 0.0,
            axis_map: [1, 2, 3],
            calibrated_at: Utc::now(),
        }
    }
}

impl Calibration {
    /// Apply calibration to a raw value.
    pub fn apply(&self, raw: f64) -> f64 {
        raw * self.scale + self.offset
    }

    /// Apply axis mapping to a 3D vector.
    pub fn map_axes(&self, input: [f64; 3]) -> [f64; 3] {
        let mut output = [0.0; 3];
        for (i, &axis) in self.axis_map.iter().enumerate() {
            let idx = (axis.unsigned_abs() as usize).saturating_sub(1);
            let sign = if axis < 0 { -1.0 } else { 1.0 };
            if idx < 3 {
                output[i] = input[idx] * sign;
            }
        }
        output
    }
}

/// Adapter status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterStatus {
    Active,
    Calibrating,
    Error,
    Disabled,
}

/// Converted sensor reading in canonical format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalReading {
    pub adapter_id: EntityId,
    pub sensor_type: SensorType,
    pub values: Vec<f64>,
    pub quality: f64,
    pub timestamp: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Adapter registry
// ---------------------------------------------------------------------------

/// Manages sensor adapters for all manufacturers.
pub struct AdapterRegistry {
    adapters: HashMap<EntityId, SensorAdapter>,
    /// manufacturer_id → [adapter_ids]
    mfr_adapters: HashMap<EntityId, Vec<EntityId>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            mfr_adapters: HashMap::new(),
        }
    }

    /// Register a sensor adapter.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        manufacturer_id: EntityId,
        sensor_type: SensorType,
        input_format: DataFormat,
        calibration: Calibration,
    ) -> SensorAdapter {
        let adapter = SensorAdapter {
            id: EntityId::new(),
            name: name.into(),
            manufacturer_id,
            sensor_type,
            input_format,
            calibration,
            status: AdapterStatus::Active,
            samples_processed: 0,
            errors: 0,
            created_at: Utc::now(),
        };

        let result = adapter.clone();
        self.mfr_adapters
            .entry(manufacturer_id)
            .or_default()
            .push(adapter.id);
        self.adapters.insert(adapter.id, adapter);
        result
    }

    /// Process a raw reading through an adapter.
    pub fn process(
        &mut self,
        adapter_id: &EntityId,
        raw_values: Vec<f64>,
    ) -> Result<CanonicalReading, AdapterError> {
        let adapter = self
            .adapters
            .get_mut(adapter_id)
            .ok_or(AdapterError::NotFound(*adapter_id))?;

        if adapter.status != AdapterStatus::Active {
            adapter.errors += 1;
            return Err(AdapterError::NotActive);
        }

        let calibrated: Vec<f64> = raw_values
            .iter()
            .map(|&v| adapter.calibration.apply(v))
            .collect();

        adapter.samples_processed += 1;

        let quality = if adapter.errors == 0 {
            1.0
        } else {
            1.0 - (adapter.errors as f64 / (adapter.samples_processed + adapter.errors) as f64)
        };

        Ok(CanonicalReading {
            adapter_id: *adapter_id,
            sensor_type: adapter.sensor_type,
            values: calibrated,
            quality,
            timestamp: Utc::now(),
        })
    }

    /// Update calibration for an adapter.
    pub fn recalibrate(
        &mut self,
        adapter_id: &EntityId,
        calibration: Calibration,
    ) -> Result<(), AdapterError> {
        let adapter = self
            .adapters
            .get_mut(adapter_id)
            .ok_or(AdapterError::NotFound(*adapter_id))?;
        adapter.calibration = calibration;
        adapter.status = AdapterStatus::Active;
        Ok(())
    }

    /// Set adapter status.
    pub fn set_status(
        &mut self,
        adapter_id: &EntityId,
        status: AdapterStatus,
    ) -> Result<(), AdapterError> {
        let adapter = self
            .adapters
            .get_mut(adapter_id)
            .ok_or(AdapterError::NotFound(*adapter_id))?;
        adapter.status = status;
        Ok(())
    }

    /// Get adapter by ID.
    pub fn get(&self, adapter_id: &EntityId) -> Option<&SensorAdapter> {
        self.adapters.get(adapter_id)
    }

    /// Adapters for a manufacturer.
    pub fn for_manufacturer(&self, mfr_id: &EntityId) -> Vec<&SensorAdapter> {
        self.mfr_adapters
            .get(mfr_id)
            .map(|ids| ids.iter().filter_map(|id| self.adapters.get(id)).collect())
            .unwrap_or_default()
    }

    /// Total adapters.
    pub fn total(&self) -> usize {
        self.adapters.len()
    }

    /// Active adapters.
    pub fn active(&self) -> Vec<&SensorAdapter> {
        self.adapters
            .values()
            .filter(|a| a.status == AdapterStatus::Active)
            .collect()
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("adapter not found: {0}")]
    NotFound(EntityId),
    #[error("adapter is not active")]
    NotActive,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_registry() -> AdapterRegistry {
        AdapterRegistry::new()
    }

    #[test]
    fn register_and_retrieve() {
        let mut registry = test_registry();
        let mfr_id = EntityId::new();
        let adapter = registry.register(
            "Tesla IMU",
            mfr_id,
            SensorType::Imu,
            DataFormat::CanBus,
            Calibration::default(),
        );
        assert_eq!(adapter.status, AdapterStatus::Active);
        assert_eq!(adapter.samples_processed, 0);
        assert_eq!(registry.for_manufacturer(&mfr_id).len(), 1);
    }

    #[test]
    fn process_raw_values() {
        let mut registry = test_registry();
        let adapter = registry.register(
            "Speed Sensor",
            EntityId::new(),
            SensorType::WheelSpeed,
            DataFormat::CanBus,
            Calibration {
                scale: 0.01,
                offset: 0.0,
                ..Default::default()
            },
        );

        let reading = registry.process(&adapter.id, vec![1500.0]).unwrap();
        assert!((reading.values[0] - 15.0).abs() < 0.001); // 1500 * 0.01 = 15.0
        assert_eq!(reading.sensor_type, SensorType::WheelSpeed);
    }

    #[test]
    fn calibration_apply() {
        let cal = Calibration {
            scale: 2.0,
            offset: -10.0,
            ..Default::default()
        };
        assert!((cal.apply(20.0) - 30.0).abs() < 0.001); // 20 * 2 + (-10) = 30
    }

    #[test]
    fn axis_mapping() {
        let cal = Calibration {
            axis_map: [2, -1, 3], // swap x/y, negate original x
            ..Default::default()
        };
        let input = [1.0, 2.0, 3.0];
        let output = cal.map_axes(input);
        assert!((output[0] - 2.0).abs() < 0.001); // axis 2 = input[1] = 2.0
        assert!((output[1] - (-1.0)).abs() < 0.001); // axis -1 = -input[0] = -1.0
        assert!((output[2] - 3.0).abs() < 0.001); // axis 3 = input[2] = 3.0
    }

    #[test]
    fn process_inactive_fails() {
        let mut registry = test_registry();
        let adapter = registry.register(
            "Disabled",
            EntityId::new(),
            SensorType::Imu,
            DataFormat::CanBus,
            Calibration::default(),
        );
        registry
            .set_status(&adapter.id, AdapterStatus::Disabled)
            .unwrap();
        assert!(registry.process(&adapter.id, vec![1.0]).is_err());
    }

    #[test]
    fn recalibrate() {
        let mut registry = test_registry();
        let adapter = registry.register(
            "Sensor",
            EntityId::new(),
            SensorType::Temperature,
            DataFormat::CanBus,
            Calibration::default(),
        );
        registry
            .set_status(&adapter.id, AdapterStatus::Calibrating)
            .unwrap();
        registry
            .recalibrate(
                &adapter.id,
                Calibration {
                    scale: 0.1,
                    offset: -40.0,
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(
            registry.get(&adapter.id).unwrap().status,
            AdapterStatus::Active
        );
    }

    #[test]
    fn quality_degrades_with_errors() {
        let mut registry = test_registry();
        let adapter = registry.register(
            "Sensor",
            EntityId::new(),
            SensorType::Imu,
            DataFormat::CanBus,
            Calibration::default(),
        );

        // Process a few good readings.
        registry.process(&adapter.id, vec![1.0]).unwrap();
        registry.process(&adapter.id, vec![2.0]).unwrap();

        // Simulate an error by disabling and trying to process.
        registry
            .set_status(&adapter.id, AdapterStatus::Error)
            .unwrap();
        let _ = registry.process(&adapter.id, vec![3.0]); // will fail

        // Re-enable and check quality degraded.
        registry
            .set_status(&adapter.id, AdapterStatus::Active)
            .unwrap();
        let reading = registry.process(&adapter.id, vec![4.0]).unwrap();
        assert!(reading.quality < 1.0);
    }

    #[test]
    fn total_and_active() {
        let mut registry = test_registry();
        let a1 = registry.register(
            "A",
            EntityId::new(),
            SensorType::Imu,
            DataFormat::CanBus,
            Calibration::default(),
        );
        registry.register(
            "B",
            EntityId::new(),
            SensorType::Gnss,
            DataFormat::Nmea,
            Calibration::default(),
        );
        registry
            .set_status(&a1.id, AdapterStatus::Disabled)
            .unwrap();
        assert_eq!(registry.total(), 2);
        assert_eq!(registry.active().len(), 1);
    }
}
