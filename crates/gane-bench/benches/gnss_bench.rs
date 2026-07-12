//! Benchmarks for GNSS receiver ingestion and constellation management.

use chrono::Utc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gane_core::gnss::{
    Constellation, GnssMeasurement, SatelliteId, SatelliteMeasurement, SignalType,
};
use gane_core::types::EntityId;
use gane_gnss::{ConstellationManager, GnssReceiver};

fn make_measurement(prn: u8, constellation: Constellation, cn0: f64) -> SatelliteMeasurement {
    SatelliteMeasurement {
        id: EntityId::new(),
        satellite: SatelliteId { constellation, prn },
        signal: SignalType::GpsL1CA,
        timestamp: Utc::now(),
        pseudorange_m: 20_000_000.0 + prn as f64 * 1000.0,
        carrier_phase_cycles: None,
        doppler_hz: Some(-1500.0),
        cn0_dbhz: cn0,
        healthy: true,
        satellite_position: None,
        elevation_deg: Some(45.0),
        azimuth_deg: Some(180.0),
    }
}

fn make_epoch(count: u8, constellation: Constellation) -> GnssMeasurement {
    let measurements: Vec<SatelliteMeasurement> = (1..=count)
        .map(|prn| make_measurement(prn, constellation, 42.0))
        .collect();
    GnssMeasurement {
        id: EntityId::new(),
        timestamp: Utc::now(),
        measurements,
        receiver_clock_bias_ns: None,
        receiver_clock_drift_nps: None,
    }
}

fn bench_receiver_creation(c: &mut Criterion) {
    c.bench_function("GnssReceiver::new", |b| {
        b.iter(|| black_box(GnssReceiver::new()));
    });
}

fn bench_constellation_manager_creation(c: &mut Criterion) {
    c.bench_function("ConstellationManager::new", |b| {
        b.iter(|| black_box(ConstellationManager::new()));
    });
}

fn bench_ingest_4_satellites(c: &mut Criterion) {
    let epoch = make_epoch(4, Constellation::Gps);
    c.bench_function("ingest_4_satellites", |b| {
        b.iter(|| {
            let mut rx = GnssReceiver::new();
            rx.ingest(black_box(&epoch));
        });
    });
}

fn bench_ingest_12_satellites(c: &mut Criterion) {
    let epoch = make_epoch(12, Constellation::Gps);
    c.bench_function("ingest_12_satellites", |b| {
        b.iter(|| {
            let mut rx = GnssReceiver::new();
            rx.ingest(black_box(&epoch));
        });
    });
}

fn bench_ingest_multi_constellation(c: &mut Criterion) {
    let mut measurements = Vec::new();
    for prn in 1..=8 {
        measurements.push(make_measurement(prn, Constellation::Gps, 42.0));
    }
    for prn in 1..=6 {
        measurements.push(make_measurement(prn, Constellation::Galileo, 40.0));
    }
    for prn in 1..=4 {
        measurements.push(make_measurement(prn, Constellation::Glonass, 38.0));
    }
    for prn in 1..=6 {
        measurements.push(make_measurement(prn, Constellation::BeiDou, 39.0));
    }
    let epoch = GnssMeasurement {
        id: EntityId::new(),
        timestamp: Utc::now(),
        measurements,
        receiver_clock_bias_ns: None,
        receiver_clock_drift_nps: None,
    };

    c.bench_function("ingest_24_sats_4_constellations", |b| {
        b.iter(|| {
            let mut rx = GnssReceiver::new();
            rx.ingest(black_box(&epoch));
        });
    });
}

fn bench_tracked_count(c: &mut Criterion) {
    let mut rx = GnssReceiver::new();
    let epoch = make_epoch(12, Constellation::Gps);
    rx.ingest(&epoch);

    c.bench_function("tracked_count_12_sats", |b| {
        b.iter(|| black_box(rx.tracked_count()));
    });
}

fn bench_process_epoch(c: &mut Criterion) {
    let epoch = make_epoch(8, Constellation::Gps);
    c.bench_function("ConstellationManager::process_epoch_8_sats", |b| {
        b.iter(|| {
            let mut mgr = ConstellationManager::new();
            let _ = mgr.process_epoch(black_box(&epoch));
        });
    });
}

criterion_group!(
    benches,
    bench_receiver_creation,
    bench_constellation_manager_creation,
    bench_ingest_4_satellites,
    bench_ingest_12_satellites,
    bench_ingest_multi_constellation,
    bench_tracked_count,
    bench_process_epoch,
);
criterion_main!(benches);
