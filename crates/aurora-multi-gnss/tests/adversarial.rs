use aurora_multi_gnss::engine::*;
fn sat(c: Constellation, prn: u16, snr: f64, elev: f64) -> SatelliteInfo {
    SatelliteInfo {
        prn,
        constellation: c,
        elevation_deg: elev,
        azimuth_deg: 180.0,
        snr_dbhz: snr,
        used_in_fix: true,
        frequency: FrequencyBand::L1,
        health: SatHealth::Healthy,
    }
}
#[test]
fn low_elev() {
    let mut e = MultiGnssEngine::new();
    e.update_satellites(vec![sat(Constellation::Gps, 1, 40.0, 5.0)]);
    assert_eq!(e.usable_satellites().len(), 0);
}
#[test]
fn low_snr() {
    let mut e = MultiGnssEngine::new();
    e.update_satellites(vec![sat(Constellation::Gps, 1, 10.0, 45.0)]);
    assert_eq!(e.usable_satellites().len(), 0);
}
#[test]
fn unhealthy() {
    let mut e = MultiGnssEngine::new();
    let mut s = sat(Constellation::Gps, 1, 40.0, 45.0);
    s.health = SatHealth::Unhealthy;
    e.update_satellites(vec![s]);
    assert_eq!(e.usable_satellites().len(), 0);
}
#[test]
fn multi_fix() {
    let mut e = MultiGnssEngine::new();
    e.update_satellites(vec![
        sat(Constellation::Gps, 1, 40.0, 45.0),
        sat(Constellation::Glonass, 1, 38.0, 50.0),
        sat(Constellation::BeiDou, 1, 42.0, 55.0),
        sat(Constellation::Galileo, 1, 44.0, 60.0),
    ]);
    let f = e.compute_fix().unwrap();
    assert!(f.constellations_used.len() >= 4);
}
#[test]
fn score_range() {
    let mut e = MultiGnssEngine::new();
    e.update_satellites(
        (1..=12)
            .map(|i| sat(Constellation::Gps, i, 45.0, 45.0))
            .collect(),
    );
    let f = e.compute_fix().unwrap();
    assert!((0.0..=1.0).contains(&f.quality_score));
}
#[test]
fn clamp_weight() {
    let mut e = MultiGnssEngine::new();
    e.set_weight(Constellation::Gps, 2.0);
    e.set_weight(Constellation::Glonass, -1.0);
}
#[test]
fn fix_ser() {
    let f = GnssFix {
        latitude_deg: 32.0,
        longitude_deg: 34.0,
        altitude_m: 100.0,
        hdop: 1.0,
        vdop: 1.5,
        pdop: 1.8,
        accuracy_m: 2.5,
        fix_type: FixType::Fix3D,
        satellites_used: 12,
        constellations_used: vec![Constellation::Gps, Constellation::Galileo],
        timestamp_ms: 1000,
        quality_score: 0.9,
        dual_frequency: true,
    };
    assert!(serde_json::to_string(&f).unwrap().contains("quality_score"));
}
#[test]
fn stress() {
    let mut e = MultiGnssEngine::new();
    e.update_satellites(
        (1..=50)
            .map(|i| {
                let c = match i % 4 {
                    0 => Constellation::Gps,
                    1 => Constellation::Glonass,
                    2 => Constellation::BeiDou,
                    _ => Constellation::Galileo,
                };
                sat(c, i, 30.0 + (i as f64) * 0.5, 20.0 + (i as f64) * 0.5)
            })
            .collect(),
    );
    let f = e.compute_fix().unwrap();
    assert!(f.satellites_used > 0);
}
