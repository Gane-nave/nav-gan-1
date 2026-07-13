use gane_ekf::filter::*;
fn m(src: SensorSource, lat: f64, lon: f64, acc: f64) -> SensorMeasurement {
    SensorMeasurement {
        source: src,
        value: [lat, lon, 0.0],
        accuracy: acc,
        timestamp_ms: 0,
        weight: 1.0,
    }
}
#[test]
fn empty_upd() {
    let mut e = EkfFusion::new();
    let f = e.update();
    assert_eq!(f.sources_used.len(), 0);
}
#[test]
fn stress() {
    let mut e = EkfFusion::new();
    for i in 0..1000 {
        e.add_measurement(m(SensorSource::Gnss, 32.0 + i as f64 * 0.0001, 34.0, 2.5));
        e.update();
    }
    assert_eq!(e.update_count(), 1000);
}
#[test]
fn predict_many() {
    let mut e = EkfFusion::new();
    for _ in 0..100 {
        e.predict(0.1);
    }
}
#[test]
fn low_acc() {
    let mut e = EkfFusion::new();
    e.add_measurement(m(SensorSource::Gnss, 32.0, 34.0, 0.001));
    let f = e.update();
    assert!(f.latitude_deg != 0.0);
}
#[test]
fn all_src() {
    let mut e = EkfFusion::new();
    for s in [
        SensorSource::Gnss,
        SensorSource::Imu,
        SensorSource::Magnetometer,
        SensorSource::Barometer,
        SensorSource::CellTower,
        SensorSource::WiFi,
        SensorSource::Odometry,
        SensorSource::MapMatch,
    ] {
        e.add_measurement(m(s, 32.0, 34.0, 5.0));
    }
    assert_eq!(e.update().sources_used.len(), 8);
}
#[test]
fn fused_ser() {
    let f = FusedState {
        latitude_deg: 32.0,
        longitude_deg: 34.0,
        altitude_m: 0.0,
        velocity_north_mps: 0.0,
        velocity_east_mps: 0.0,
        velocity_down_mps: 0.0,
        heading_deg: 0.0,
        accuracy_m: 2.5,
        confidence: 0.9,
        sources_used: vec![SensorSource::Gnss],
        timestamp_ms: 0,
    };
    assert!(serde_json::to_string(&f).unwrap().contains("confidence"));
}
#[test]
fn set_w() {
    let mut e = EkfFusion::new();
    e.set_source_weight(SensorSource::Gnss, 0.5);
}
#[test]
fn pred_upd() {
    let mut e = EkfFusion::new();
    e.predict(1.0);
    e.add_measurement(m(SensorSource::Gnss, 32.0, 34.0, 2.5));
    assert!(e.update().sources_used.contains(&SensorSource::Gnss));
}
