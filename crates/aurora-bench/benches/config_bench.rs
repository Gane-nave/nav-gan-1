//! Benchmarks for configuration building and validation.

use aurora_config::{AuroraConfig, ConfigBuilder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_config_default(c: &mut Criterion) {
    c.bench_function("AuroraConfig::default", |b| {
        b.iter(|| black_box(AuroraConfig::default()));
    });
}

fn bench_config_builder_simple(c: &mut Criterion) {
    c.bench_function("ConfigBuilder_simple_build", |b| {
        b.iter(|| {
            black_box(
                ConfigBuilder::new()
                    .api_port(9090)
                    .instance_name("bench-node")
                    .build()
                    .unwrap(),
            )
        });
    });
}

fn bench_config_builder_full(c: &mut Criterion) {
    c.bench_function("ConfigBuilder_full_build", |b| {
        b.iter(|| {
            black_box(
                ConfigBuilder::new()
                    .api_port(9090)
                    .api_host("0.0.0.0")
                    .instance_name("bench-node")
                    .log_level("debug")
                    .worker_threads(8)
                    .min_satellites(6)
                    .elevation_mask_deg(10.0)
                    .enable_gps(true)
                    .enable_galileo(true)
                    .enable_glonass(false)
                    .enable_beidou(true)
                    .telemetry_buffer(10_000)
                    .fleet_enabled(true)
                    .satellite_enabled(false)
                    .twin_enabled(true)
                    .vehicle_enabled(true)
                    .payments_enabled(false)
                    .process_noise_scale(1.5)
                    .risk_aversion(0.7)
                    .max_alternatives(5)
                    .build()
                    .unwrap(),
            )
        });
    });
}

fn bench_config_serialise(c: &mut Criterion) {
    let config = AuroraConfig::default();
    c.bench_function("config_serialise_json", |b| {
        b.iter(|| black_box(serde_json::to_string(&config).unwrap()));
    });
}

fn bench_config_deserialise(c: &mut Criterion) {
    let config = AuroraConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    c.bench_function("config_deserialise_json", |b| {
        b.iter(|| black_box(serde_json::from_str::<AuroraConfig>(&json).unwrap()));
    });
}

criterion_group!(
    benches,
    bench_config_default,
    bench_config_builder_simple,
    bench_config_builder_full,
    bench_config_serialise,
    bench_config_deserialise,
);
criterion_main!(benches);
