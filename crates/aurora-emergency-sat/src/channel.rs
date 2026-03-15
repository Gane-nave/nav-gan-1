//! Multi-channel failover — manages satellite, mesh, and radio communication
//! channels with automatic failover and priority-based channel selection.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Communication channel type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    /// Iridium satellite (global coverage, high latency)
    IridiumSat,
    /// Globalstar satellite
    GlobalstarSat,
    /// Cospas-Sarsat (dedicated SAR satellite)
    CospasSarsat,
    /// LoRa mesh radio (short-range, low power)
    LoraMesh,
    /// VHF radio (marine/aviation)
    VhfRadio,
    /// UHF radio (terrestrial)
    UhfRadio,
    /// Wi-Fi direct (peer-to-peer, very short range)
    WifiDirect,
    /// Bluetooth mesh (very short range)
    BluetoothMesh,
}

impl ChannelType {
    /// Nominal range in kilometers.
    pub fn nominal_range_km(&self) -> f64 {
        match self {
            Self::IridiumSat | Self::GlobalstarSat | Self::CospasSarsat => 40_000.0,
            Self::LoraMesh => 15.0,
            Self::VhfRadio => 80.0,
            Self::UhfRadio => 40.0,
            Self::WifiDirect => 0.1,
            Self::BluetoothMesh => 0.05,
        }
    }

    /// Maximum bandwidth in bytes per second.
    pub fn max_bandwidth_bps(&self) -> u32 {
        match self {
            Self::IridiumSat => 2400,
            Self::GlobalstarSat => 9600,
            Self::CospasSarsat => 400,
            Self::LoraMesh => 300,
            Self::VhfRadio => 1200,
            Self::UhfRadio => 9600,
            Self::WifiDirect => 1_000_000,
            Self::BluetoothMesh => 250_000,
        }
    }

    /// Default priority (lower = higher priority for emergency use).
    pub fn default_priority(&self) -> u8 {
        match self {
            Self::CospasSarsat => 1,
            Self::IridiumSat => 2,
            Self::GlobalstarSat => 3,
            Self::VhfRadio => 4,
            Self::UhfRadio => 5,
            Self::LoraMesh => 6,
            Self::WifiDirect => 7,
            Self::BluetoothMesh => 8,
        }
    }
}

/// Current health status of a communication channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelStatus {
    /// Channel is available and functioning
    Available,
    /// Channel is degraded (partial connectivity)
    Degraded,
    /// Channel is unavailable
    Unavailable,
    /// Channel status unknown (not yet probed)
    Unknown,
}

/// A single communication channel instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel type
    pub channel_type: ChannelType,
    /// Current status
    pub status: ChannelStatus,
    /// Override priority (if set, overrides default)
    pub priority: u8,
    /// Signal strength (0.0 to 1.0)
    pub signal_strength: f64,
    /// Last successful transmission timestamp
    pub last_tx_success: Option<DateTime<Utc>>,
    /// Last probe timestamp
    pub last_probe: Option<DateTime<Utc>>,
    /// Consecutive failure count
    pub failure_count: u32,
    /// Maximum consecutive failures before marking unavailable
    pub max_failures: u32,
    /// Messages sent via this channel
    pub messages_sent: u64,
    /// Messages received via this channel
    pub messages_received: u64,
}

impl Channel {
    /// Create a new channel with default settings.
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            priority: channel_type.default_priority(),
            channel_type,
            status: ChannelStatus::Unknown,
            signal_strength: 0.0,
            last_tx_success: None,
            last_probe: None,
            failure_count: 0,
            max_failures: 5,
            messages_sent: 0,
            messages_received: 0,
        }
    }

    /// Record a successful transmission.
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.last_tx_success = Some(Utc::now());
        self.messages_sent += 1;
        if self.status != ChannelStatus::Available {
            self.status = ChannelStatus::Available;
        }
    }

    /// Record a failed transmission.
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        if self.failure_count >= self.max_failures {
            self.status = ChannelStatus::Unavailable;
        } else if self.failure_count >= self.max_failures / 2 {
            self.status = ChannelStatus::Degraded;
        }
    }

    /// Update signal strength and adjust status accordingly.
    pub fn update_signal(&mut self, strength: f64) {
        self.signal_strength = strength.clamp(0.0, 1.0);
        self.last_probe = Some(Utc::now());
        if self.signal_strength < 0.1 {
            self.status = ChannelStatus::Unavailable;
        } else if self.signal_strength < 0.3 {
            self.status = ChannelStatus::Degraded;
        } else {
            self.status = ChannelStatus::Available;
        }
    }

    /// Whether this channel is usable for transmission.
    pub fn is_usable(&self) -> bool {
        matches!(
            self.status,
            ChannelStatus::Available | ChannelStatus::Degraded
        )
    }
}

/// Multi-channel failover manager.
///
/// Maintains a set of communication channels and selects the best available
/// one for each transmission attempt, with automatic failover.
#[derive(Debug)]
pub struct ChannelManager {
    channels: Vec<Channel>,
    /// Index of the currently active channel
    active_idx: Option<usize>,
}

impl ChannelManager {
    /// Create a manager with the given channel types.
    pub fn new(types: &[ChannelType]) -> Self {
        let channels = types.iter().map(|t| Channel::new(*t)).collect();
        Self {
            channels,
            active_idx: None,
        }
    }

    /// Create a manager with all available channel types.
    pub fn all_channels() -> Self {
        Self::new(&[
            ChannelType::CospasSarsat,
            ChannelType::IridiumSat,
            ChannelType::GlobalstarSat,
            ChannelType::VhfRadio,
            ChannelType::UhfRadio,
            ChannelType::LoraMesh,
            ChannelType::WifiDirect,
            ChannelType::BluetoothMesh,
        ])
    }

    /// Select the best available channel (lowest priority number, usable).
    pub fn select_best(&mut self) -> Result<&Channel, ChannelError> {
        let best_idx = self
            .channels
            .iter()
            .enumerate()
            .filter(|(_, ch)| ch.is_usable())
            .min_by_key(|(_, ch)| ch.priority)
            .map(|(i, _)| i);

        match best_idx {
            Some(idx) => {
                self.active_idx = Some(idx);
                Ok(&self.channels[idx])
            }
            None => Err(ChannelError::NoChannelAvailable),
        }
    }

    /// Failover to the next best channel (skip current).
    pub fn failover(&mut self) -> Result<&Channel, ChannelError> {
        if let Some(active) = self.active_idx {
            self.channels[active].record_failure();
        }

        let skip_idx = self.active_idx;
        let next_idx = self
            .channels
            .iter()
            .enumerate()
            .filter(|(i, ch)| ch.is_usable() && Some(*i) != skip_idx)
            .min_by_key(|(_, ch)| ch.priority)
            .map(|(i, _)| i);

        match next_idx {
            Some(idx) => {
                self.active_idx = Some(idx);
                Ok(&self.channels[idx])
            }
            None => Err(ChannelError::NoChannelAvailable),
        }
    }

    /// Get the currently active channel.
    pub fn active_channel(&self) -> Option<&Channel> {
        self.active_idx.map(|i| &self.channels[i])
    }

    /// Record success on the active channel.
    pub fn record_success(&mut self) {
        if let Some(idx) = self.active_idx {
            self.channels[idx].record_success();
        }
    }

    /// Record failure on the active channel and attempt failover.
    pub fn record_failure_and_failover(&mut self) -> Result<&Channel, ChannelError> {
        self.failover()
    }

    /// Get all channels with their current status.
    pub fn all_statuses(&self) -> Vec<(ChannelType, ChannelStatus, f64)> {
        self.channels
            .iter()
            .map(|ch| (ch.channel_type, ch.status, ch.signal_strength))
            .collect()
    }

    /// Update signal strength for a specific channel type.
    pub fn update_signal(&mut self, channel_type: ChannelType, strength: f64) {
        if let Some(ch) = self
            .channels
            .iter_mut()
            .find(|ch| ch.channel_type == channel_type)
        {
            ch.update_signal(strength);
        }
    }

    /// Number of usable channels.
    pub fn usable_count(&self) -> usize {
        self.channels.iter().filter(|ch| ch.is_usable()).count()
    }

    /// Total number of channels.
    pub fn total_count(&self) -> usize {
        self.channels.len()
    }
}

/// Channel-related errors.
#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("no communication channel available")]
    NoChannelAvailable,

    #[error("channel {0:?} is not usable")]
    ChannelNotUsable(ChannelType),

    #[error("transmission failed on channel {0:?}: {1}")]
    TransmissionFailed(ChannelType, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let ch = Channel::new(ChannelType::IridiumSat);
        assert_eq!(ch.channel_type, ChannelType::IridiumSat);
        assert_eq!(ch.status, ChannelStatus::Unknown);
        assert_eq!(ch.priority, 2);
    }

    #[test]
    fn test_channel_signal_update() {
        let mut ch = Channel::new(ChannelType::LoraMesh);
        ch.update_signal(0.8);
        assert_eq!(ch.status, ChannelStatus::Available);

        ch.update_signal(0.2);
        assert_eq!(ch.status, ChannelStatus::Degraded);

        ch.update_signal(0.05);
        assert_eq!(ch.status, ChannelStatus::Unavailable);
    }

    #[test]
    fn test_channel_failure_escalation() {
        let mut ch = Channel::new(ChannelType::VhfRadio);
        ch.status = ChannelStatus::Available;
        ch.max_failures = 4;

        ch.record_failure();
        assert_eq!(ch.status, ChannelStatus::Available);

        ch.record_failure();
        assert_eq!(ch.status, ChannelStatus::Degraded);

        ch.record_failure();
        ch.record_failure();
        assert_eq!(ch.status, ChannelStatus::Unavailable);
    }

    #[test]
    fn test_channel_success_resets_failures() {
        let mut ch = Channel::new(ChannelType::UhfRadio);
        ch.status = ChannelStatus::Degraded;
        ch.failure_count = 3;

        ch.record_success();
        assert_eq!(ch.failure_count, 0);
        assert_eq!(ch.status, ChannelStatus::Available);
    }

    #[test]
    fn test_manager_select_best() {
        let mut mgr = ChannelManager::new(&[
            ChannelType::LoraMesh,
            ChannelType::IridiumSat,
            ChannelType::CospasSarsat,
        ]);
        // Mark all as available
        mgr.update_signal(ChannelType::LoraMesh, 0.9);
        mgr.update_signal(ChannelType::IridiumSat, 0.7);
        mgr.update_signal(ChannelType::CospasSarsat, 0.5);

        let best = mgr.select_best().unwrap();
        // Cospas-Sarsat has priority=1 (highest emergency priority)
        assert_eq!(best.channel_type, ChannelType::CospasSarsat);
    }

    #[test]
    fn test_manager_failover() {
        let mut mgr = ChannelManager::new(&[ChannelType::CospasSarsat, ChannelType::IridiumSat]);
        mgr.update_signal(ChannelType::CospasSarsat, 0.8);
        mgr.update_signal(ChannelType::IridiumSat, 0.7);

        mgr.select_best().unwrap();
        let next = mgr.failover().unwrap();
        assert_eq!(next.channel_type, ChannelType::IridiumSat);
    }

    #[test]
    fn test_manager_no_channel_available() {
        let mut mgr = ChannelManager::new(&[ChannelType::LoraMesh]);
        // Don't update signal — stays Unknown, not usable
        assert!(mgr.select_best().is_err());
    }

    #[test]
    fn test_channel_type_properties() {
        assert!(ChannelType::IridiumSat.nominal_range_km() > 1000.0);
        assert!(ChannelType::BluetoothMesh.nominal_range_km() < 1.0);
        assert!(
            ChannelType::CospasSarsat.default_priority()
                < ChannelType::BluetoothMesh.default_priority()
        );
    }
}
