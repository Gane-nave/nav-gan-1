//! OEM integration manager — manufacturer registration, capability
//! discovery, firmware tracking, and fleet-wide OEM operations.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// OEM types
// ---------------------------------------------------------------------------

/// A registered vehicle manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manufacturer {
    pub id: EntityId,
    pub name: String,
    pub code: String,
    pub country: String,
    pub supported_protocols: Vec<String>,
    pub capabilities: Vec<OemCapability>,
    pub firmware_versions: Vec<FirmwareVersion>,
    pub status: OemStatus,
    pub registered_at: DateTime<Utc>,
    pub vehicle_count: u64,
}

/// OEM integration capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OemCapability {
    /// Direct CAN bus access.
    CanBus,
    /// OBD-II diagnostics.
    ObdII,
    /// Real-time telemetry streaming.
    Telemetry,
    /// Remote vehicle control.
    RemoteControl,
    /// Over-the-air updates.
    OtaUpdate,
    /// ADAS sensor access.
    AdasSensors,
    /// Battery management (EV).
    BatteryManagement,
    /// Navigation data injection.
    NavInjection,
    /// Camera feed access.
    CameraFeed,
    /// V2X communication.
    V2X,
}

/// Firmware version entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersion {
    pub version: String,
    pub release_date: DateTime<Utc>,
    pub changelog: String,
    pub min_hardware_rev: String,
    pub deprecated: bool,
}

/// OEM integration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OemStatus {
    Active,
    Pending,
    Suspended,
    Deprecated,
}

// ---------------------------------------------------------------------------
// OEM manager
// ---------------------------------------------------------------------------

/// Manages OEM manufacturer registrations.
pub struct OemManager {
    manufacturers: HashMap<EntityId, Manufacturer>,
    code_index: HashMap<String, EntityId>,
}

impl OemManager {
    pub fn new() -> Self {
        Self {
            manufacturers: HashMap::new(),
            code_index: HashMap::new(),
        }
    }

    /// Register a new manufacturer.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        code: impl Into<String>,
        country: impl Into<String>,
        protocols: Vec<String>,
        capabilities: Vec<OemCapability>,
    ) -> Result<Manufacturer, OemError> {
        let code_str = code.into();
        if self.code_index.contains_key(&code_str) {
            return Err(OemError::CodeTaken(code_str));
        }

        let mfr = Manufacturer {
            id: EntityId::new(),
            name: name.into(),
            code: code_str.clone(),
            country: country.into(),
            supported_protocols: protocols,
            capabilities,
            firmware_versions: Vec::new(),
            status: OemStatus::Pending,
            registered_at: Utc::now(),
            vehicle_count: 0,
        };

        info!(mfr_id = %mfr.id, code = %code_str, "manufacturer registered");
        self.code_index.insert(code_str, mfr.id);
        let result = mfr.clone();
        self.manufacturers.insert(mfr.id, mfr);
        Ok(result)
    }

    /// Activate a pending manufacturer.
    pub fn activate(&mut self, mfr_id: &EntityId) -> Result<(), OemError> {
        let mfr = self
            .manufacturers
            .get_mut(mfr_id)
            .ok_or(OemError::NotFound(*mfr_id))?;

        if mfr.status != OemStatus::Pending {
            return Err(OemError::InvalidTransition {
                from: mfr.status,
                to: OemStatus::Active,
            });
        }

        mfr.status = OemStatus::Active;
        Ok(())
    }

    /// Suspend a manufacturer.
    pub fn suspend(&mut self, mfr_id: &EntityId) -> Result<(), OemError> {
        let mfr = self
            .manufacturers
            .get_mut(mfr_id)
            .ok_or(OemError::NotFound(*mfr_id))?;
        mfr.status = OemStatus::Suspended;
        Ok(())
    }

    /// Add a firmware version.
    pub fn add_firmware(
        &mut self,
        mfr_id: &EntityId,
        version: impl Into<String>,
        changelog: impl Into<String>,
        min_hw_rev: impl Into<String>,
    ) -> Result<(), OemError> {
        let mfr = self
            .manufacturers
            .get_mut(mfr_id)
            .ok_or(OemError::NotFound(*mfr_id))?;

        let ver_str = version.into();
        if mfr.firmware_versions.iter().any(|f| f.version == ver_str) {
            return Err(OemError::FirmwareExists(ver_str));
        }

        mfr.firmware_versions.push(FirmwareVersion {
            version: ver_str,
            release_date: Utc::now(),
            changelog: changelog.into(),
            min_hardware_rev: min_hw_rev.into(),
            deprecated: false,
        });

        Ok(())
    }

    /// Register a vehicle for a manufacturer.
    pub fn register_vehicle(&mut self, mfr_id: &EntityId) -> Result<u64, OemError> {
        let mfr = self
            .manufacturers
            .get_mut(mfr_id)
            .ok_or(OemError::NotFound(*mfr_id))?;

        if mfr.status != OemStatus::Active {
            return Err(OemError::NotActive);
        }

        mfr.vehicle_count += 1;
        Ok(mfr.vehicle_count)
    }

    /// Check if manufacturer has a capability.
    pub fn has_capability(&self, mfr_id: &EntityId, cap: OemCapability) -> Result<bool, OemError> {
        let mfr = self
            .manufacturers
            .get(mfr_id)
            .ok_or(OemError::NotFound(*mfr_id))?;
        Ok(mfr.capabilities.contains(&cap))
    }

    /// Get manufacturer by ID.
    pub fn get(&self, mfr_id: &EntityId) -> Option<&Manufacturer> {
        self.manufacturers.get(mfr_id)
    }

    /// Get manufacturer by code.
    pub fn get_by_code(&self, code: &str) -> Option<&Manufacturer> {
        self.code_index
            .get(code)
            .and_then(|id| self.manufacturers.get(id))
    }

    /// List active manufacturers.
    pub fn active(&self) -> Vec<&Manufacturer> {
        self.manufacturers
            .values()
            .filter(|m| m.status == OemStatus::Active)
            .collect()
    }

    /// Total registered manufacturers.
    pub fn total(&self) -> usize {
        self.manufacturers.len()
    }

    /// Manufacturers with a specific capability.
    pub fn with_capability(&self, cap: OemCapability) -> Vec<&Manufacturer> {
        self.manufacturers
            .values()
            .filter(|m| m.capabilities.contains(&cap) && m.status == OemStatus::Active)
            .collect()
    }
}

impl Default for OemManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum OemError {
    #[error("manufacturer not found: {0}")]
    NotFound(EntityId),
    #[error("manufacturer code already taken: {0}")]
    CodeTaken(String),
    #[error("invalid state transition from {from:?} to {to:?}")]
    InvalidTransition { from: OemStatus, to: OemStatus },
    #[error("manufacturer is not active")]
    NotActive,
    #[error("firmware version already exists: {0}")]
    FirmwareExists(String),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mgr() -> OemManager {
        OemManager::new()
    }

    fn register_test_mfr(mgr: &mut OemManager) -> Manufacturer {
        mgr.register(
            "Tesla",
            "TSLA",
            "US",
            vec!["CAN-FD".into()],
            vec![
                OemCapability::CanBus,
                OemCapability::Telemetry,
                OemCapability::BatteryManagement,
            ],
        )
        .unwrap()
    }

    #[test]
    fn register_and_retrieve() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        assert_eq!(mfr.status, OemStatus::Pending);
        assert_eq!(mfr.capabilities.len(), 3);
        assert!(mgr.get_by_code("TSLA").is_some());
    }

    #[test]
    fn duplicate_code_rejected() {
        let mut mgr = test_mgr();
        register_test_mfr(&mut mgr);
        let result = mgr.register("Tesla 2", "TSLA", "US", vec![], vec![]);
        assert!(matches!(result.unwrap_err(), OemError::CodeTaken(_)));
    }

    #[test]
    fn activate() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.activate(&mfr.id).unwrap();
        assert_eq!(mgr.get(&mfr.id).unwrap().status, OemStatus::Active);
    }

    #[test]
    fn activate_non_pending_fails() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.activate(&mfr.id).unwrap();
        let result = mgr.activate(&mfr.id);
        assert!(result.is_err());
    }

    #[test]
    fn register_vehicle() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.activate(&mfr.id).unwrap();
        assert_eq!(mgr.register_vehicle(&mfr.id).unwrap(), 1);
        assert_eq!(mgr.register_vehicle(&mfr.id).unwrap(), 2);
    }

    #[test]
    fn register_vehicle_inactive_fails() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        assert!(mgr.register_vehicle(&mfr.id).is_err());
    }

    #[test]
    fn firmware() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.add_firmware(&mfr.id, "1.0.0", "Initial", "HW-1")
            .unwrap();
        mgr.add_firmware(&mfr.id, "1.1.0", "Update", "HW-1")
            .unwrap();
        assert_eq!(mgr.get(&mfr.id).unwrap().firmware_versions.len(), 2);
    }

    #[test]
    fn duplicate_firmware_rejected() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.add_firmware(&mfr.id, "1.0.0", "Initial", "HW-1")
            .unwrap();
        let result = mgr.add_firmware(&mfr.id, "1.0.0", "Dup", "HW-1");
        assert!(matches!(result.unwrap_err(), OemError::FirmwareExists(_)));
    }

    #[test]
    fn capability_check() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        assert!(mgr.has_capability(&mfr.id, OemCapability::CanBus).unwrap());
        assert!(!mgr.has_capability(&mfr.id, OemCapability::V2X).unwrap());
    }

    #[test]
    fn with_capability_filter() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.activate(&mfr.id).unwrap();

        mgr.register(
            "BMW",
            "BMW",
            "DE",
            vec![],
            vec![OemCapability::CanBus, OemCapability::V2X],
        )
        .unwrap();
        // BMW is pending so won't appear.

        let can_bus_mfrs = mgr.with_capability(OemCapability::CanBus);
        assert_eq!(can_bus_mfrs.len(), 1);
        assert_eq!(can_bus_mfrs[0].code, "TSLA");
    }

    #[test]
    fn suspend() {
        let mut mgr = test_mgr();
        let mfr = register_test_mfr(&mut mgr);
        mgr.activate(&mfr.id).unwrap();
        mgr.suspend(&mfr.id).unwrap();
        assert_eq!(mgr.get(&mfr.id).unwrap().status, OemStatus::Suspended);
        assert_eq!(mgr.active().len(), 0);
    }
}
