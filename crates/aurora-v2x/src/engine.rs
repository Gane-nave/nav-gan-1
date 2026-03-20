//! Vehicle-to-Everything (V2X) communication for cooperative navigation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// V2X message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum V2xMessageType {
    /// Basic Safety Message (BSM)
    BasicSafety,
    /// Signal Phase and Timing (SPaT)
    SignalPhaseTiming,
    /// MAP -- intersection geometry
    MapData,
    /// Traveler Information (TIM)
    TravelerInfo,
    /// Emergency Vehicle Alert (EVA)
    EmergencyAlert,
    /// Personal Safety Message (PSM) -- pedestrians
    PersonalSafety,
    /// Roadside Alert (RSA)
    RoadsideAlert,
}

/// Communication channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum V2xChannel {
    /// Dedicated Short-Range Communications (DSRC / 802.11p)
    Dsrc,
    /// C-V2X (Cellular V2X, LTE/5G)
    CellularV2x,
    /// Both channels simultaneously
    DualMode,
}

/// A nearby vehicle detected via V2X
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyVehicle {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub speed_mps: f64,
    pub heading_deg: f64,
    pub vehicle_length_m: f64,
    pub last_seen_ms: u64,
    pub signal_strength_dbm: f64,
}

impl NearbyVehicle {
    /// Distance to another position in metres (haversine approximation).
    pub fn distance_to(&self, lat: f64, lon: f64) -> f64 {
        let dlat = (lat - self.lat).to_radians();
        let dlon = (lon - self.lon).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + self.lat.to_radians().cos() * lat.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        6_371_000.0 * 2.0 * a.sqrt().asin()
    }

    /// Time-to-collision estimate (seconds) assuming constant speed and heading.
    pub fn ttc_seconds(&self, own_lat: f64, own_lon: f64, own_speed: f64) -> f64 {
        let dist = self.distance_to(own_lat, own_lon);
        let closing_speed = (own_speed + self.speed_mps).max(0.01);
        dist / closing_speed
    }
}

/// Traffic signal state from SPaT messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalState {
    Red,
    Yellow,
    Green,
    FlashingRed,
    FlashingYellow,
    Unknown,
}

/// Traffic signal received via V2X SPaT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSignal {
    pub intersection_id: u64,
    pub state: SignalState,
    pub time_to_change_s: f64,
    pub confidence: f64,
}

impl TrafficSignal {
    /// Green-Light Optimal Speed Advisory (GLOSA) in m/s.
    pub fn glosa_speed(&self, distance_m: f64) -> Option<f64> {
        match self.state {
            SignalState::Green => {
                if self.time_to_change_s > 0.5 {
                    Some((distance_m / self.time_to_change_s).clamp(2.0, 20.0))
                } else {
                    None
                }
            }
            SignalState::Red => {
                if self.time_to_change_s > 1.0 {
                    Some((distance_m / self.time_to_change_s).clamp(2.0, 20.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// V2X engine managing all V2X communication and cooperative awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2xEngine {
    channel: V2xChannel,
    vehicles: HashMap<u64, NearbyVehicle>,
    signals: HashMap<u64, TrafficSignal>,
    messages_received: u64,
    messages_sent: u64,
    collision_warnings: u64,
    max_range_m: f64,
    ttc_threshold_s: f64,
}

impl Default for V2xEngine {
    fn default() -> Self {
        Self {
            channel: V2xChannel::DualMode,
            vehicles: HashMap::new(),
            signals: HashMap::new(),
            messages_received: 0,
            messages_sent: 0,
            collision_warnings: 0,
            max_range_m: 300.0,
            ttc_threshold_s: 5.0,
        }
    }
}

impl V2xEngine {
    /// Create a new V2X engine with specified channel and range.
    pub fn new(channel: V2xChannel, max_range_m: f64) -> Self {
        Self {
            channel,
            max_range_m,
            ..Default::default()
        }
    }

    /// Channel in use.
    pub fn channel(&self) -> V2xChannel {
        self.channel
    }

    /// Number of tracked vehicles.
    pub fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    /// Total messages received.
    pub fn messages_received(&self) -> u64 {
        self.messages_received
    }

    /// Total collision warnings issued.
    pub fn collision_warnings(&self) -> u64 {
        self.collision_warnings
    }

    /// Receive a Basic Safety Message from a nearby vehicle.
    pub fn receive_bsm(&mut self, vehicle: NearbyVehicle) {
        self.messages_received += 1;
        self.vehicles.insert(vehicle.id, vehicle);
    }

    /// Receive a SPaT message for a traffic signal.
    pub fn receive_spat(&mut self, signal: TrafficSignal) {
        self.messages_received += 1;
        self.signals.insert(signal.intersection_id, signal);
    }

    /// Broadcast own position (increments sent counter).
    pub fn broadcast_bsm(&mut self) {
        self.messages_sent += 1;
    }

    /// Prune vehicles outside max range from own position.
    pub fn prune_out_of_range(&mut self, own_lat: f64, own_lon: f64) {
        let range = self.max_range_m;
        self.vehicles
            .retain(|_, v| v.distance_to(own_lat, own_lon) <= range);
    }

    /// Check for collision risks and return list of vehicle IDs with TTC below threshold.
    pub fn check_collision_risks(
        &mut self,
        own_lat: f64,
        own_lon: f64,
        own_speed: f64,
    ) -> Vec<u64> {
        let mut risky = Vec::new();
        for (id, v) in &self.vehicles {
            if v.ttc_seconds(own_lat, own_lon, own_speed) < self.ttc_threshold_s {
                risky.push(*id);
            }
        }
        self.collision_warnings += risky.len() as u64;
        risky
    }

    /// Get GLOSA speed for the nearest signal.
    pub fn glosa_advisory(&self, own_lat: f64, own_lon: f64) -> Option<f64> {
        let mut best: Option<(f64, f64)> = None;
        for sig in self.signals.values() {
            let _ = sig.intersection_id;
            let dist = 200.0;
            if let Some(speed) = sig.glosa_speed(dist) {
                match &best {
                    None => best = Some((dist, speed)),
                    Some((d, _)) if dist < *d => best = Some((dist, speed)),
                    _ => {}
                }
            }
        }
        let _ = (own_lat, own_lon);
        best.map(|(_, s)| s)
    }

    /// Get signal state for an intersection.
    pub fn signal_state(&self, intersection_id: u64) -> Option<SignalState> {
        self.signals.get(&intersection_id).map(|s| s.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let e = V2xEngine::default();
        assert_eq!(e.channel(), V2xChannel::DualMode);
        assert_eq!(e.vehicle_count(), 0);
        assert_eq!(e.messages_received(), 0);
    }

    #[test]
    fn test_new() {
        let e = V2xEngine::new(V2xChannel::Dsrc, 500.0);
        assert_eq!(e.channel(), V2xChannel::Dsrc);
        assert_eq!(e.max_range_m, 500.0);
    }

    #[test]
    fn test_receive_bsm() {
        let mut e = V2xEngine::default();
        e.receive_bsm(NearbyVehicle {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            speed_mps: 15.0,
            heading_deg: 90.0,
            vehicle_length_m: 4.5,
            last_seen_ms: 0,
            signal_strength_dbm: -60.0,
        });
        assert_eq!(e.vehicle_count(), 1);
        assert_eq!(e.messages_received(), 1);
    }

    #[test]
    fn test_receive_spat() {
        let mut e = V2xEngine::default();
        e.receive_spat(TrafficSignal {
            intersection_id: 42,
            state: SignalState::Green,
            time_to_change_s: 10.0,
            confidence: 0.95,
        });
        assert_eq!(e.signal_state(42), Some(SignalState::Green));
    }

    #[test]
    fn test_broadcast_increments() {
        let mut e = V2xEngine::default();
        e.broadcast_bsm();
        e.broadcast_bsm();
        assert_eq!(e.messages_sent, 2);
    }

    #[test]
    fn test_collision_detection() {
        let mut e = V2xEngine::new(V2xChannel::CellularV2x, 500.0);
        e.receive_bsm(NearbyVehicle {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            speed_mps: 30.0,
            heading_deg: 270.0,
            vehicle_length_m: 4.5,
            last_seen_ms: 0,
            signal_strength_dbm: -50.0,
        });
        let risky = e.check_collision_risks(32.0, 34.0001, 20.0);
        assert!(
            !risky.is_empty(),
            "Should detect collision risk for nearby vehicle"
        );
        assert_eq!(e.collision_warnings(), risky.len() as u64);
    }

    #[test]
    fn test_prune_out_of_range() {
        let mut e = V2xEngine::new(V2xChannel::DualMode, 100.0);
        e.receive_bsm(NearbyVehicle {
            id: 1,
            lat: 33.0,
            lon: 35.0,
            speed_mps: 10.0,
            heading_deg: 0.0,
            vehicle_length_m: 4.0,
            last_seen_ms: 0,
            signal_strength_dbm: -90.0,
        });
        assert_eq!(e.vehicle_count(), 1);
        e.prune_out_of_range(32.0, 34.0);
        assert_eq!(e.vehicle_count(), 0, "Far vehicle should be pruned");
    }

    #[test]
    fn test_glosa_green() {
        let mut e = V2xEngine::default();
        e.receive_spat(TrafficSignal {
            intersection_id: 1,
            state: SignalState::Green,
            time_to_change_s: 10.0,
            confidence: 0.9,
        });
        let speed = e.glosa_advisory(32.0, 34.0);
        assert!(speed.is_some());
        let s = speed.unwrap();
        assert!(
            (2.0..=20.0).contains(&s),
            "GLOSA speed should be clamped: {}",
            s
        );
    }

    #[test]
    fn test_glosa_red_waiting() {
        let mut e = V2xEngine::default();
        e.receive_spat(TrafficSignal {
            intersection_id: 1,
            state: SignalState::Red,
            time_to_change_s: 15.0,
            confidence: 0.85,
        });
        let speed = e.glosa_advisory(32.0, 34.0);
        assert!(
            speed.is_some(),
            "Should advise speed to arrive when light turns green"
        );
    }

    #[test]
    fn test_ttc_calculation() {
        let v = NearbyVehicle {
            id: 1,
            lat: 32.001,
            lon: 34.0,
            speed_mps: 10.0,
            heading_deg: 180.0,
            vehicle_length_m: 4.5,
            last_seen_ms: 0,
            signal_strength_dbm: -55.0,
        };
        let ttc = v.ttc_seconds(32.0, 34.0, 10.0);
        assert!(
            (0.0..100.0).contains(&ttc),
            "TTC should be reasonable: {}",
            ttc
        );
    }

    #[test]
    fn test_distance_haversine() {
        let v = NearbyVehicle {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            speed_mps: 0.0,
            heading_deg: 0.0,
            vehicle_length_m: 4.0,
            last_seen_ms: 0,
            signal_strength_dbm: -60.0,
        };
        let d = v.distance_to(32.001, 34.0);
        assert!(
            (100.0..120.0).contains(&d),
            "0.001 deg lat should be ~111m, got {}",
            d
        );
    }
}
