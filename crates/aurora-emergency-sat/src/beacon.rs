//! SOS beacon management — periodic distress signal broadcasting,
//! beacon lifecycle, and multi-frequency transmission scheduling.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::protocol::{EmergencyPriority, MessageType, SatMessage};

/// Beacon state in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeaconState {
    /// Beacon is idle, not transmitting
    Idle,
    /// Beacon is actively broadcasting SOS
    Active,
    /// Beacon acknowledged by rescue services
    Acknowledged,
    /// Beacon cancelled by user
    Cancelled,
    /// Beacon expired (TTL reached)
    Expired,
}

/// Configuration for SOS beacon broadcasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconConfig {
    /// Interval between beacon transmissions (seconds)
    pub interval_s: u32,
    /// Total beacon lifetime before auto-expiry (seconds)
    pub lifetime_s: u32,
    /// Include altitude in beacon
    pub include_altitude: bool,
    /// Include speed/heading in beacon
    pub include_motion: bool,
    /// Burst repeat count — how many times to repeat each burst
    pub burst_repeats: u8,
}

impl Default for BeaconConfig {
    fn default() -> Self {
        Self {
            interval_s: 30,
            lifetime_s: 86400, // 24 hours
            include_altitude: true,
            include_motion: true,
            burst_repeats: 3,
        }
    }
}

/// An active SOS beacon instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SosBeacon {
    /// Unique beacon identifier
    pub id: Uuid,
    /// Device that activated the beacon
    pub device_id: Uuid,
    /// Current beacon state
    pub state: BeaconState,
    /// Last known latitude
    pub lat: f64,
    /// Last known longitude
    pub lon: f64,
    /// Last known altitude (meters)
    pub alt_m: Option<f32>,
    /// Emergency description
    pub description: String,
    /// Number of people requiring assistance
    pub persons_count: u32,
    /// Beacon configuration
    pub config: BeaconConfig,
    /// Timestamp when beacon was activated
    pub activated_at: DateTime<Utc>,
    /// Timestamp of last transmission
    pub last_tx_at: Option<DateTime<Utc>>,
    /// Total transmissions sent
    pub tx_count: u64,
    /// Number of acknowledgements received
    pub ack_count: u32,
}

impl SosBeacon {
    /// Create and activate a new SOS beacon.
    pub fn activate(
        device_id: Uuid,
        lat: f64,
        lon: f64,
        description: &str,
        persons_count: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            device_id,
            state: BeaconState::Active,
            lat,
            lon,
            alt_m: None,
            description: description.to_string(),
            persons_count,
            config: BeaconConfig::default(),
            activated_at: Utc::now(),
            last_tx_at: None,
            tx_count: 0,
            ack_count: 0,
        }
    }

    /// Check if the beacon should transmit now based on its interval.
    pub fn should_transmit(&self) -> bool {
        if self.state != BeaconState::Active {
            return false;
        }
        if self.is_expired() {
            return false;
        }
        match self.last_tx_at {
            None => true,
            Some(last) => {
                let elapsed = Utc::now().signed_duration_since(last).num_seconds();
                elapsed >= i64::from(self.config.interval_s)
            }
        }
    }

    /// Generate the next SOS message for transmission.
    pub fn generate_message(&mut self) -> Option<SatMessage> {
        if !self.should_transmit() {
            return None;
        }

        let payload = format!(
            "SOS|{}|persons:{}|{}",
            self.id, self.persons_count, self.description
        );
        let mut msg = SatMessage::new(
            self.device_id,
            MessageType::SosBeacon,
            EmergencyPriority::Distress,
            self.lat,
            self.lon,
            &payload,
        );
        msg.alt_m = self.alt_m;
        msg.ttl_s = self.config.lifetime_s;
        msg.truncate_to_fit();

        self.last_tx_at = Some(Utc::now());
        self.tx_count += 1;

        Some(msg)
    }

    /// Update beacon position (for moving emergencies).
    pub fn update_position(&mut self, lat: f64, lon: f64, alt_m: Option<f32>) {
        if self.state == BeaconState::Active {
            self.lat = lat;
            self.lon = lon;
            self.alt_m = alt_m;
        }
    }

    /// Record an acknowledgement from rescue services.
    pub fn acknowledge(&mut self) {
        if self.state == BeaconState::Active {
            self.ack_count += 1;
            self.state = BeaconState::Acknowledged;
        }
    }

    /// Cancel the beacon (false alarm or resolved).
    pub fn cancel(&mut self) {
        if self.state == BeaconState::Active || self.state == BeaconState::Acknowledged {
            self.state = BeaconState::Cancelled;
        }
    }

    /// Check if the beacon has exceeded its lifetime.
    pub fn is_expired(&self) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.activated_at)
            .num_seconds();
        elapsed >= i64::from(self.config.lifetime_s)
    }

    /// Duration since beacon activation.
    pub fn active_duration(&self) -> Duration {
        Utc::now().signed_duration_since(self.activated_at)
    }
}

/// Beacon registry — manages multiple active beacons.
#[derive(Debug, Default)]
pub struct BeaconRegistry {
    beacons: Vec<SosBeacon>,
}

impl BeaconRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            beacons: Vec::new(),
        }
    }

    /// Activate a new SOS beacon and register it.
    pub fn activate_beacon(
        &mut self,
        device_id: Uuid,
        lat: f64,
        lon: f64,
        description: &str,
        persons_count: u32,
    ) -> Uuid {
        let beacon = SosBeacon::activate(device_id, lat, lon, description, persons_count);
        let id = beacon.id;
        self.beacons.push(beacon);
        id
    }

    /// Get a beacon by its ID.
    pub fn get(&self, id: Uuid) -> Option<&SosBeacon> {
        self.beacons.iter().find(|b| b.id == id)
    }

    /// Get a mutable beacon by its ID.
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut SosBeacon> {
        self.beacons.iter_mut().find(|b| b.id == id)
    }

    /// Get all active beacons.
    pub fn active_beacons(&self) -> Vec<&SosBeacon> {
        self.beacons
            .iter()
            .filter(|b| b.state == BeaconState::Active)
            .collect()
    }

    /// Generate messages for all beacons that need to transmit.
    pub fn generate_all_messages(&mut self) -> Vec<SatMessage> {
        self.beacons
            .iter_mut()
            .filter_map(|b| b.generate_message())
            .collect()
    }

    /// Expire stale beacons and return the count of newly expired ones.
    pub fn expire_stale(&mut self) -> usize {
        let mut expired_count = 0;
        for beacon in &mut self.beacons {
            if beacon.state == BeaconState::Active && beacon.is_expired() {
                beacon.state = BeaconState::Expired;
                expired_count += 1;
            }
        }
        expired_count
    }

    /// Total number of registered beacons (all states).
    pub fn total_count(&self) -> usize {
        self.beacons.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beacon_activation() {
        let device = Uuid::new_v4();
        let beacon = SosBeacon::activate(device, 32.08, 34.78, "car accident", 2);
        assert_eq!(beacon.state, BeaconState::Active);
        assert_eq!(beacon.persons_count, 2);
        assert_eq!(beacon.tx_count, 0);
        assert!(!beacon.is_expired());
    }

    #[test]
    fn test_beacon_generate_message() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        let msg = beacon.generate_message();
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.msg_type, MessageType::SosBeacon);
        assert_eq!(msg.priority, EmergencyPriority::Distress);
        assert_eq!(beacon.tx_count, 1);
    }

    #[test]
    fn test_beacon_respects_interval() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        beacon.config.interval_s = 60;

        // First message should succeed
        assert!(beacon.generate_message().is_some());
        // Immediately after, should NOT transmit (interval not elapsed)
        assert!(beacon.generate_message().is_none());
    }

    #[test]
    fn test_beacon_acknowledge() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        beacon.acknowledge();
        assert_eq!(beacon.state, BeaconState::Acknowledged);
        assert_eq!(beacon.ack_count, 1);
    }

    #[test]
    fn test_beacon_cancel() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        beacon.cancel();
        assert_eq!(beacon.state, BeaconState::Cancelled);
        // Cancelled beacon should not transmit
        assert!(!beacon.should_transmit());
    }

    #[test]
    fn test_beacon_expired_no_transmit() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        beacon.config.lifetime_s = 0;
        assert!(beacon.is_expired());
        assert!(!beacon.should_transmit());
    }

    #[test]
    fn test_registry_activate_and_find() {
        let mut registry = BeaconRegistry::new();
        let device = Uuid::new_v4();
        let id = registry.activate_beacon(device, 32.08, 34.78, "flood", 5);
        assert_eq!(registry.total_count(), 1);
        assert_eq!(registry.active_beacons().len(), 1);
        assert!(registry.get(id).is_some());
    }

    #[test]
    fn test_registry_expire_stale() {
        let mut registry = BeaconRegistry::new();
        let device = Uuid::new_v4();
        let id = registry.activate_beacon(device, 32.08, 34.78, "test", 1);
        registry.get_mut(id).unwrap().config.lifetime_s = 0;

        let expired = registry.expire_stale();
        assert_eq!(expired, 1);
        assert_eq!(registry.active_beacons().len(), 0);
    }

    #[test]
    fn test_update_position() {
        let device = Uuid::new_v4();
        let mut beacon = SosBeacon::activate(device, 32.08, 34.78, "test", 1);
        beacon.update_position(33.0, 35.0, Some(100.0));
        assert!((beacon.lat - 33.0).abs() < f64::EPSILON);
        assert!((beacon.lon - 35.0).abs() < f64::EPSILON);
        assert_eq!(beacon.alt_m, Some(100.0));
    }
}
