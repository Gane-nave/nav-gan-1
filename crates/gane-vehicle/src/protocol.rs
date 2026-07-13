//! Vehicle communication protocols — CAN bus message parsing,
//! OBD-II PID handling, and protocol negotiation.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Protocol types
// ---------------------------------------------------------------------------

/// A CAN bus message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanMessage {
    pub id: u32,
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub bus: CanBusType,
    pub extended: bool,
}

/// CAN bus type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanBusType {
    /// Standard CAN (500 kbps).
    Can,
    /// CAN-FD (up to 8 Mbps).
    CanFd,
    /// Automotive Ethernet (100 Mbps).
    Ethernet,
}

/// An OBD-II Parameter ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObdPid {
    EngineRpm,
    VehicleSpeed,
    CoolantTemp,
    FuelLevel,
    ThrottlePosition,
    IntakeAirTemp,
    MafAirFlow,
    EngineLoad,
    FuelPressure,
    BatteryVoltage,
}

impl ObdPid {
    /// OBD-II PID byte code.
    pub fn code(&self) -> u8 {
        match self {
            Self::EngineRpm => 0x0C,
            Self::VehicleSpeed => 0x0D,
            Self::CoolantTemp => 0x05,
            Self::FuelLevel => 0x2F,
            Self::ThrottlePosition => 0x11,
            Self::IntakeAirTemp => 0x0F,
            Self::MafAirFlow => 0x10,
            Self::EngineLoad => 0x04,
            Self::FuelPressure => 0x0A,
            Self::BatteryVoltage => 0x42,
        }
    }

    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::EngineRpm => "Engine RPM",
            Self::VehicleSpeed => "Vehicle Speed",
            Self::CoolantTemp => "Coolant Temperature",
            Self::FuelLevel => "Fuel Level",
            Self::ThrottlePosition => "Throttle Position",
            Self::IntakeAirTemp => "Intake Air Temperature",
            Self::MafAirFlow => "MAF Air Flow",
            Self::EngineLoad => "Engine Load",
            Self::FuelPressure => "Fuel Pressure",
            Self::BatteryVoltage => "Battery Voltage",
        }
    }

    /// Unit of measurement.
    pub fn unit(&self) -> &'static str {
        match self {
            Self::EngineRpm => "rpm",
            Self::VehicleSpeed => "km/h",
            Self::CoolantTemp | Self::IntakeAirTemp => "°C",
            Self::FuelLevel | Self::ThrottlePosition | Self::EngineLoad => "%",
            Self::MafAirFlow => "g/s",
            Self::FuelPressure => "kPa",
            Self::BatteryVoltage => "V",
        }
    }
}

/// Decoded OBD-II reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObdReading {
    pub pid: ObdPid,
    pub value: f64,
    pub raw_bytes: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

/// Protocol negotiation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSession {
    pub id: EntityId,
    pub protocol: CanBusType,
    pub supported_pids: Vec<ObdPid>,
    pub vehicle_vin: Option<String>,
    pub established_at: DateTime<Utc>,
    pub active: bool,
}

// ---------------------------------------------------------------------------
// Protocol handler
// ---------------------------------------------------------------------------

/// Handles vehicle protocol communication.
pub struct ProtocolHandler {
    sessions: HashMap<EntityId, ProtocolSession>,
    message_log: Vec<CanMessage>,
    max_log_size: usize,
}

impl ProtocolHandler {
    pub fn new(max_log_size: usize) -> Self {
        Self {
            sessions: HashMap::new(),
            message_log: Vec::new(),
            max_log_size,
        }
    }

    /// Negotiate a protocol session with a vehicle.
    pub fn negotiate(
        &mut self,
        protocol: CanBusType,
        supported_pids: Vec<ObdPid>,
        vin: Option<String>,
    ) -> ProtocolSession {
        let session = ProtocolSession {
            id: EntityId::new(),
            protocol,
            supported_pids,
            vehicle_vin: vin,
            established_at: Utc::now(),
            active: true,
        };

        let result = session.clone();
        self.sessions.insert(session.id, session);
        result
    }

    /// Close a protocol session.
    pub fn close_session(&mut self, session_id: &EntityId) -> Result<(), ProtocolError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or(ProtocolError::SessionNotFound(*session_id))?;
        session.active = false;
        Ok(())
    }

    /// Log a CAN message.
    pub fn log_message(&mut self, message: CanMessage) {
        if self.message_log.len() >= self.max_log_size {
            self.message_log.remove(0);
        }
        self.message_log.push(message);
    }

    /// Decode an OBD-II response.
    pub fn decode_obd(pid: ObdPid, data: &[u8]) -> Result<f64, ProtocolError> {
        if data.is_empty() {
            return Err(ProtocolError::InvalidData("empty response".into()));
        }

        let value = match pid {
            ObdPid::EngineRpm => {
                if data.len() < 2 {
                    return Err(ProtocolError::InvalidData("RPM needs 2 bytes".into()));
                }
                ((data[0] as f64) * 256.0 + data[1] as f64) / 4.0
            }
            ObdPid::VehicleSpeed => data[0] as f64,
            ObdPid::CoolantTemp | ObdPid::IntakeAirTemp => data[0] as f64 - 40.0,
            ObdPid::FuelLevel | ObdPid::ThrottlePosition | ObdPid::EngineLoad => {
                data[0] as f64 * 100.0 / 255.0
            }
            ObdPid::MafAirFlow => {
                if data.len() < 2 {
                    return Err(ProtocolError::InvalidData("MAF needs 2 bytes".into()));
                }
                ((data[0] as f64) * 256.0 + data[1] as f64) / 100.0
            }
            ObdPid::FuelPressure => data[0] as f64 * 3.0,
            ObdPid::BatteryVoltage => {
                if data.len() < 2 {
                    return Err(ProtocolError::InvalidData(
                        "Battery voltage needs 2 bytes".into(),
                    ));
                }
                ((data[0] as f64) * 256.0 + data[1] as f64) / 1000.0
            }
        };

        Ok(value)
    }

    /// Check if a session supports a PID.
    pub fn supports_pid(&self, session_id: &EntityId, pid: ObdPid) -> Result<bool, ProtocolError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or(ProtocolError::SessionNotFound(*session_id))?;
        Ok(session.supported_pids.contains(&pid))
    }

    /// Get session by ID.
    pub fn get_session(&self, session_id: &EntityId) -> Option<&ProtocolSession> {
        self.sessions.get(session_id)
    }

    /// Active sessions.
    pub fn active_sessions(&self) -> Vec<&ProtocolSession> {
        self.sessions.values().filter(|s| s.active).collect()
    }

    /// Message log length.
    pub fn log_size(&self) -> usize {
        self.message_log.len()
    }

    /// Recent messages.
    pub fn recent_messages(&self, count: usize) -> &[CanMessage] {
        let start = self.message_log.len().saturating_sub(count);
        &self.message_log[start..]
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("session not found: {0}")]
    SessionNotFound(EntityId),
    #[error("invalid data: {0}")]
    InvalidData(String),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_handler() -> ProtocolHandler {
        ProtocolHandler::new(100)
    }

    #[test]
    fn negotiate_session() {
        let mut handler = test_handler();
        let session = handler.negotiate(
            CanBusType::CanFd,
            vec![ObdPid::EngineRpm, ObdPid::VehicleSpeed],
            Some("WBA12345678901234".into()),
        );
        assert!(session.active);
        assert_eq!(session.protocol, CanBusType::CanFd);
        assert_eq!(session.supported_pids.len(), 2);
    }

    #[test]
    fn close_session() {
        let mut handler = test_handler();
        let session = handler.negotiate(CanBusType::Can, vec![], None);
        handler.close_session(&session.id).unwrap();
        assert!(!handler.get_session(&session.id).unwrap().active);
        assert_eq!(handler.active_sessions().len(), 0);
    }

    #[test]
    fn decode_rpm() {
        let value = ProtocolHandler::decode_obd(ObdPid::EngineRpm, &[0x1A, 0xF8]).unwrap();
        // (0x1A * 256 + 0xF8) / 4 = (26*256 + 248) / 4 = 6904/4 = 1726
        assert!((value - 1726.0).abs() < 0.01);
    }

    #[test]
    fn decode_speed() {
        let value = ProtocolHandler::decode_obd(ObdPid::VehicleSpeed, &[120]).unwrap();
        assert!((value - 120.0).abs() < 0.01);
    }

    #[test]
    fn decode_coolant_temp() {
        let value = ProtocolHandler::decode_obd(ObdPid::CoolantTemp, &[130]).unwrap();
        assert!((value - 90.0).abs() < 0.01); // 130 - 40 = 90°C
    }

    #[test]
    fn decode_fuel_level() {
        let value = ProtocolHandler::decode_obd(ObdPid::FuelLevel, &[128]).unwrap();
        // 128 * 100 / 255 ≈ 50.2%
        assert!((value - 50.196).abs() < 0.01);
    }

    #[test]
    fn decode_empty_data_fails() {
        let result = ProtocolHandler::decode_obd(ObdPid::EngineRpm, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn decode_insufficient_bytes_fails() {
        let result = ProtocolHandler::decode_obd(ObdPid::EngineRpm, &[0x1A]);
        assert!(result.is_err());
    }

    #[test]
    fn log_message() {
        let mut handler = test_handler();
        handler.log_message(CanMessage {
            id: 0x7E8,
            data: vec![0x41, 0x0C, 0x1A, 0xF8],
            timestamp: Utc::now(),
            bus: CanBusType::Can,
            extended: false,
        });
        assert_eq!(handler.log_size(), 1);
    }

    #[test]
    fn log_eviction() {
        let mut handler = ProtocolHandler::new(3);
        for i in 0..5 {
            handler.log_message(CanMessage {
                id: i,
                data: vec![],
                timestamp: Utc::now(),
                bus: CanBusType::Can,
                extended: false,
            });
        }
        assert_eq!(handler.log_size(), 3);
        assert_eq!(handler.recent_messages(3)[0].id, 2);
    }

    #[test]
    fn supports_pid() {
        let mut handler = test_handler();
        let session = handler.negotiate(
            CanBusType::Can,
            vec![ObdPid::EngineRpm, ObdPid::VehicleSpeed],
            None,
        );
        assert!(handler
            .supports_pid(&session.id, ObdPid::EngineRpm)
            .unwrap());
        assert!(!handler
            .supports_pid(&session.id, ObdPid::FuelLevel)
            .unwrap());
    }

    #[test]
    fn pid_metadata() {
        assert_eq!(ObdPid::EngineRpm.code(), 0x0C);
        assert_eq!(ObdPid::EngineRpm.name(), "Engine RPM");
        assert_eq!(ObdPid::EngineRpm.unit(), "rpm");
    }
}
