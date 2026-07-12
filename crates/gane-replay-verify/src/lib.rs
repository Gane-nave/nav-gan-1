//! Golden-trace replay: drive a sensor log through the canonical ESKF and
//! produce a deterministic final state. The same JSONL format replays through
//! the native filter (this crate / its CLI) and the WASM engine — numerical
//! agreement between the two is the TD-1 acceptance criterion ("one engine,
//! zero drift"). Doubles as the forensic-replay entry point required by the
//! evidence-chain contract.

use aurora_fusion::eskf15::Eskf15;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// One event in a replay log (JSONL: one event per line).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReplayEvent {
    /// Strapdown IMU sample, body frame.
    Imu {
        accel: [f64; 3],
        gyro: [f64; 3],
        dt_s: f64,
    },
    /// GNSS position fix, local ENU metres.
    GnssPosition { enu: [f64; 3], sigma_m: f64 },
    /// Zero-velocity update from the stationarity detector.
    Zupt { sigma_mps: f64 },
}

/// Deterministic summary of the filter after a replay.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReplayResult {
    pub events: usize,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub heading_rad: f64,
    pub accel_bias: [f64; 3],
    pub gyro_bias: [f64; 3],
    pub horizontal_uncertainty_m: f64,
}

/// Parse a JSONL log.
pub fn parse_log(jsonl: &str) -> Result<Vec<ReplayEvent>, serde_json::Error> {
    jsonl
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str)
        .collect()
}

/// Replay a log through a fresh canonical ESKF.
pub fn replay(events: &[ReplayEvent]) -> ReplayResult {
    let mut f = Eskf15::new();
    for ev in events {
        match ev {
            ReplayEvent::Imu { accel, gyro, dt_s } => {
                f.predict(Vector3::from(*accel), Vector3::from(*gyro), *dt_s)
            }
            ReplayEvent::GnssPosition { enu, sigma_m } => {
                f.update_position(Vector3::from(*enu), *sigma_m);
            }
            ReplayEvent::Zupt { sigma_mps } => {
                f.update_zupt(*sigma_mps);
            }
        }
    }
    ReplayResult {
        events: events.len(),
        position: f.position.into(),
        velocity: f.velocity.into(),
        heading_rad: f.heading_rad(),
        accel_bias: f.accel_bias.into(),
        gyro_bias: f.gyro_bias.into(),
        horizontal_uncertainty_m: f.horizontal_uncertainty_m(),
    }
}

/// The canonical golden trace: 30 s stationary with a +0.2 m/s² vertical
/// accel bias, 20 Hz IMU, 1 Hz GNSS at the origin, ZUPT every second.
/// Fully deterministic — no randomness, no clock.
pub fn golden_trace() -> Vec<ReplayEvent> {
    let mut events = Vec::new();
    for i in 0..600 {
        events.push(ReplayEvent::Imu {
            accel: [0.0, 0.0, 9.81 + 0.2],
            gyro: [0.0, 0.0, 0.0],
            dt_s: 0.05,
        });
        if i % 20 == 19 {
            events.push(ReplayEvent::GnssPosition {
                enu: [0.0, 0.0, 0.0],
                sigma_m: 1.5,
            });
            events.push(ReplayEvent::Zupt { sigma_mps: 0.03 });
        }
    }
    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_trace_is_deterministic() {
        let a = replay(&golden_trace());
        let b = replay(&golden_trace());
        assert_eq!(a, b, "replay must be bit-for-bit reproducible");
    }

    #[test]
    fn golden_trace_regression_snapshot() {
        let r = replay(&golden_trace());
        assert_eq!(r.events, 660);
        // Physics-derived envelope, not exact pins: stationary + aided.
        assert!(r.position.iter().all(|c| c.abs() < 0.5), "{:?}", r.position);
        assert!(
            r.velocity.iter().all(|c| c.abs() < 0.05),
            "{:?}",
            r.velocity
        );
        assert!(
            (r.accel_bias[2] - 0.2).abs() < 0.05,
            "vertical bias must be learned: {:?}",
            r.accel_bias
        );
        assert!(r.horizontal_uncertainty_m < 2.0);
    }

    #[test]
    fn jsonl_roundtrip() {
        let events = golden_trace();
        let jsonl: String = events
            .iter()
            .map(|e| serde_json::to_string(e).unwrap() + "\n")
            .collect();
        let parsed = parse_log(&jsonl).unwrap();
        assert_eq!(parsed.len(), events.len());
        assert_eq!(replay(&parsed), replay(&events));
    }
}
