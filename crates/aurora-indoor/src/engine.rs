//! Indoor positioning system using WiFi, BLE beacons, and magnetic fingerprinting

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Indoor positioning technology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndoorTech {
    WiFiRtt,
    BleBeacon,
    MagneticFingerprint,
    UltraWideband,
    VisualInertial,
}

/// A beacon or access-point used for positioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beacon {
    pub id: String,
    pub tech: IndoorTech,
    pub x: f64,
    pub y: f64,
    pub floor: i32,
    pub tx_power_dbm: f64,
}

/// A signal measurement from a beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalMeasurement {
    pub beacon_id: String,
    pub rssi_dbm: f64,
    pub timestamp_ms: u64,
}

impl SignalMeasurement {
    /// Estimate distance from RSSI using log-distance path loss model.
    /// d = 10^((tx_power - rssi) / (10 * n))
    pub fn estimate_distance(&self, tx_power_dbm: f64, path_loss_exp: f64) -> f64 {
        let exp = (tx_power_dbm - self.rssi_dbm) / (10.0 * path_loss_exp);
        10.0_f64.powf(exp)
    }
}

/// Indoor position estimate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorPosition {
    pub x: f64,
    pub y: f64,
    pub floor: i32,
    pub accuracy_m: f64,
    pub tech_used: IndoorTech,
    pub confidence: f64,
}

/// Floor transition detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloorTransition {
    None,
    Elevator,
    Stairs,
    Escalator,
}

/// Magnetic fingerprint entry -- stores magnetic field vector at a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagneticFingerprint {
    pub x: f64,
    pub y: f64,
    pub floor: i32,
    pub mag_x: f64,
    pub mag_y: f64,
    pub mag_z: f64,
}

impl MagneticFingerprint {
    /// Euclidean distance between magnetic vectors.
    pub fn magnetic_distance(&self, mx: f64, my: f64, mz: f64) -> f64 {
        ((self.mag_x - mx).powi(2) + (self.mag_y - my).powi(2) + (self.mag_z - mz).powi(2)).sqrt()
    }
}

/// Indoor positioning engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorEngine {
    beacons: HashMap<String, Beacon>,
    fingerprints: Vec<MagneticFingerprint>,
    current_position: Option<IndoorPosition>,
    floor_transition: FloorTransition,
    path_loss_exponent: f64,
    min_beacons_for_fix: usize,
    is_indoor: bool,
    positions_computed: u64,
}

impl Default for IndoorEngine {
    fn default() -> Self {
        Self {
            beacons: HashMap::new(),
            fingerprints: Vec::new(),
            current_position: None,
            floor_transition: FloorTransition::None,
            path_loss_exponent: 2.5,
            min_beacons_for_fix: 3,
            is_indoor: false,
            positions_computed: 0,
        }
    }
}

impl IndoorEngine {
    /// Create engine with custom path-loss exponent.
    pub fn new(path_loss_exponent: f64) -> Self {
        Self {
            path_loss_exponent,
            ..Default::default()
        }
    }

    /// Register a beacon.
    pub fn add_beacon(&mut self, beacon: Beacon) {
        self.beacons.insert(beacon.id.clone(), beacon);
    }

    /// Register a magnetic fingerprint.
    pub fn add_fingerprint(&mut self, fp: MagneticFingerprint) {
        self.fingerprints.push(fp);
    }

    /// Number of registered beacons.
    pub fn beacon_count(&self) -> usize {
        self.beacons.len()
    }

    /// Whether we believe we are indoors.
    pub fn is_indoor(&self) -> bool {
        self.is_indoor
    }

    /// Set indoor/outdoor state.
    pub fn set_indoor(&mut self, indoor: bool) {
        self.is_indoor = indoor;
    }

    /// Current position estimate.
    pub fn position(&self) -> Option<&IndoorPosition> {
        self.current_position.as_ref()
    }

    /// Current floor transition.
    pub fn floor_transition(&self) -> FloorTransition {
        self.floor_transition
    }

    /// Total positions computed.
    pub fn positions_computed(&self) -> u64 {
        self.positions_computed
    }

    /// Compute position using trilateration from signal measurements.
    pub fn compute_position(
        &mut self,
        measurements: &[SignalMeasurement],
    ) -> Option<IndoorPosition> {
        if measurements.len() < self.min_beacons_for_fix {
            return None;
        }

        let mut pairs: Vec<(&Beacon, f64)> = Vec::new();
        for m in measurements {
            if let Some(b) = self.beacons.get(&m.beacon_id) {
                let dist = m.estimate_distance(b.tx_power_dbm, self.path_loss_exponent);
                pairs.push((b, dist));
            }
        }

        if pairs.len() < self.min_beacons_for_fix {
            return None;
        }

        let mut wx = 0.0;
        let mut wy = 0.0;
        let mut wt = 0.0;
        let mut floor_votes: HashMap<i32, usize> = HashMap::new();

        for (b, dist) in &pairs {
            let w = 1.0 / (dist + 0.1);
            wx += b.x * w;
            wy += b.y * w;
            wt += w;
            *floor_votes.entry(b.floor).or_insert(0) += 1;
        }

        let x = wx / wt;
        let y = wy / wt;
        let floor = floor_votes
            .into_iter()
            .max_by_key(|(_, c)| *c)
            .map(|(f, _)| f)
            .unwrap_or(0);

        let avg_dist: f64 = pairs.iter().map(|(_, d)| d).sum::<f64>() / pairs.len() as f64;
        let accuracy = (avg_dist / pairs.len() as f64).max(1.0);
        let confidence = (pairs.len() as f64 / 6.0).min(1.0);

        let pos = IndoorPosition {
            x,
            y,
            floor,
            accuracy_m: accuracy,
            tech_used: IndoorTech::BleBeacon,
            confidence,
        };

        self.current_position = Some(pos.clone());
        self.positions_computed += 1;
        Some(pos)
    }

    /// Match magnetic fingerprint to find closest known position.
    pub fn match_fingerprint(
        &mut self,
        mag_x: f64,
        mag_y: f64,
        mag_z: f64,
    ) -> Option<IndoorPosition> {
        if self.fingerprints.is_empty() {
            return None;
        }

        let best = self.fingerprints.iter().min_by(|a, b| {
            a.magnetic_distance(mag_x, mag_y, mag_z)
                .partial_cmp(&b.magnetic_distance(mag_x, mag_y, mag_z))
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;

        let dist = best.magnetic_distance(mag_x, mag_y, mag_z);
        let confidence = (1.0 - dist / 100.0).clamp(0.0, 1.0);

        let pos = IndoorPosition {
            x: best.x,
            y: best.y,
            floor: best.floor,
            accuracy_m: (dist * 2.0).max(2.0),
            tech_used: IndoorTech::MagneticFingerprint,
            confidence,
        };

        self.current_position = Some(pos.clone());
        self.positions_computed += 1;
        Some(pos)
    }

    /// Detect floor transition from barometer pressure change.
    pub fn detect_floor_transition(&mut self, pressure_delta_pa: f64) -> FloorTransition {
        let transition = if pressure_delta_pa.abs() < 5.0 {
            FloorTransition::None
        } else if pressure_delta_pa.abs() > 40.0 {
            FloorTransition::Elevator
        } else if pressure_delta_pa.abs() > 15.0 {
            FloorTransition::Stairs
        } else {
            FloorTransition::Escalator
        };
        self.floor_transition = transition;
        transition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_beacon(id: &str, x: f64, y: f64, floor: i32) -> Beacon {
        Beacon {
            id: id.to_string(),
            tech: IndoorTech::BleBeacon,
            x,
            y,
            floor,
            tx_power_dbm: -59.0,
        }
    }

    fn make_measurement(id: &str, rssi: f64) -> SignalMeasurement {
        SignalMeasurement {
            beacon_id: id.to_string(),
            rssi_dbm: rssi,
            timestamp_ms: 0,
        }
    }

    #[test]
    fn test_default() {
        let e = IndoorEngine::default();
        assert_eq!(e.beacon_count(), 0);
        assert!(!e.is_indoor());
        assert!(e.position().is_none());
    }

    #[test]
    fn test_add_beacon() {
        let mut e = IndoorEngine::default();
        e.add_beacon(make_beacon("b1", 0.0, 0.0, 0));
        e.add_beacon(make_beacon("b2", 10.0, 0.0, 0));
        assert_eq!(e.beacon_count(), 2);
    }

    #[test]
    fn test_too_few_measurements() {
        let mut e = IndoorEngine::default();
        e.add_beacon(make_beacon("b1", 0.0, 0.0, 0));
        let result = e.compute_position(&[make_measurement("b1", -70.0)]);
        assert!(result.is_none(), "Should need >= 3 beacons for a fix");
    }

    #[test]
    fn test_trilateration() {
        let mut e = IndoorEngine::default();
        e.add_beacon(make_beacon("b1", 0.0, 0.0, 0));
        e.add_beacon(make_beacon("b2", 10.0, 0.0, 0));
        e.add_beacon(make_beacon("b3", 5.0, 10.0, 0));
        let measurements = vec![
            make_measurement("b1", -65.0),
            make_measurement("b2", -65.0),
            make_measurement("b3", -65.0),
        ];
        let pos = e.compute_position(&measurements);
        assert!(pos.is_some());
        let p = pos.unwrap();
        assert!((p.x - 5.0).abs() < 2.0, "x should be near 5.0, got {}", p.x);
        assert!(p.floor == 0);
        assert!(p.confidence > 0.0);
        assert_eq!(e.positions_computed(), 1);
    }

    #[test]
    fn test_floor_voting() {
        let mut e = IndoorEngine::default();
        e.add_beacon(make_beacon("b1", 0.0, 0.0, 1));
        e.add_beacon(make_beacon("b2", 10.0, 0.0, 1));
        e.add_beacon(make_beacon("b3", 5.0, 10.0, 2));
        let measurements = vec![
            make_measurement("b1", -60.0),
            make_measurement("b2", -60.0),
            make_measurement("b3", -80.0),
        ];
        let pos = e.compute_position(&measurements).unwrap();
        assert_eq!(pos.floor, 1, "Majority floor should win");
    }

    #[test]
    fn test_rssi_distance() {
        let m = make_measurement("b1", -70.0);
        let d = m.estimate_distance(-59.0, 2.5);
        assert!(d > 2.0 && d < 4.0, "Expected ~2.75m, got {}", d);
    }

    #[test]
    fn test_magnetic_fingerprint() {
        let mut e = IndoorEngine::default();
        e.add_fingerprint(MagneticFingerprint {
            x: 10.0,
            y: 20.0,
            floor: 0,
            mag_x: 25.0,
            mag_y: -5.0,
            mag_z: 40.0,
        });
        e.add_fingerprint(MagneticFingerprint {
            x: 30.0,
            y: 40.0,
            floor: 1,
            mag_x: 15.0,
            mag_y: 10.0,
            mag_z: 35.0,
        });
        let pos = e.match_fingerprint(25.1, -4.9, 40.1);
        assert!(pos.is_some());
        let p = pos.unwrap();
        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 20.0);
        assert_eq!(p.floor, 0);
        assert_eq!(p.tech_used, IndoorTech::MagneticFingerprint);
    }

    #[test]
    fn test_floor_transition_none() {
        let mut e = IndoorEngine::default();
        assert_eq!(e.detect_floor_transition(2.0), FloorTransition::None);
    }

    #[test]
    fn test_floor_transition_elevator() {
        let mut e = IndoorEngine::default();
        assert_eq!(e.detect_floor_transition(50.0), FloorTransition::Elevator);
    }

    #[test]
    fn test_floor_transition_stairs() {
        let mut e = IndoorEngine::default();
        assert_eq!(e.detect_floor_transition(20.0), FloorTransition::Stairs);
    }

    #[test]
    fn test_floor_transition_escalator() {
        let mut e = IndoorEngine::default();
        assert_eq!(e.detect_floor_transition(10.0), FloorTransition::Escalator);
    }

    #[test]
    fn test_indoor_outdoor_toggle() {
        let mut e = IndoorEngine::default();
        assert!(!e.is_indoor());
        e.set_indoor(true);
        assert!(e.is_indoor());
        e.set_indoor(false);
        assert!(!e.is_indoor());
    }
}
