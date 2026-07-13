//! Adversarial test: replay hash includes source_id
//! BUG: compute_observation_hash only hashed position/speed/heading, not source_id.
//! Two different sources with identical readings would trigger false DataReplay alert.

use chrono::Utc;
use gane_anti_manipulation::anomaly::*;
use gane_core::types::{EntityId, GeoPosition};

fn make_obs(source: EntityId, lat: f64, lon: f64, speed: f64, heading: f64) -> Observation {
    Observation {
        source_id: source,
        position: GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        },
        speed_mps: speed,
        heading_deg: heading,
        timestamp: Utc::now(),
    }
}

#[test]
fn different_sources_same_data_no_false_replay() {
    let mut detector = AnomalyDetector::new();
    let source_a = EntityId::new();
    let source_b = EntityId::new();

    // Source A sends data
    let anomalies_a = detector.process(&make_obs(source_a, 32.08, 34.78, 20.0, 90.0));
    let replay_a: Vec<_> = anomalies_a
        .iter()
        .filter(|a| a.anomaly_type == AnomalyType::DataReplay)
        .collect();
    assert_eq!(replay_a.len(), 0, "First observation should not be replay");

    // Source B sends IDENTICAL position/speed/heading — different source_id
    let anomalies_b = detector.process(&make_obs(source_b, 32.08, 34.78, 20.0, 90.0));
    let replay_b: Vec<_> = anomalies_b
        .iter()
        .filter(|a| a.anomaly_type == AnomalyType::DataReplay)
        .collect();
    assert_eq!(
        replay_b.len(),
        0,
        "BUG FIX: Different source_id must NOT trigger DataReplay. Old hash without source_id would false-positive here."
    );
}

#[test]
fn same_source_same_data_detects_replay() {
    let mut detector = AnomalyDetector::new();
    let source = EntityId::new();

    let obs = make_obs(source, 32.08, 34.78, 20.0, 90.0);

    // First observation — fine
    detector.process(&obs);

    // Same source, same data — should detect replay
    let anomalies = detector.process(&obs);
    let replays: Vec<_> = anomalies
        .iter()
        .filter(|a| a.anomaly_type == AnomalyType::DataReplay)
        .collect();
    assert_eq!(
        replays.len(),
        1,
        "Same source + same data must trigger DataReplay"
    );
}
