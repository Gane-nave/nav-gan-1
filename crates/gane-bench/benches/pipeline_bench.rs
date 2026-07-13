//! Benchmarks for the full navigation pipeline and health aggregation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gane_app::health::build_health_report;
use gane_app::pipeline::NavigationPipeline;
use gane_config::AuroraConfig;

fn bench_pipeline_creation(c: &mut Criterion) {
    c.bench_function("NavigationPipeline::new", |b| {
        b.iter(|| {
            let config = AuroraConfig::default();
            black_box(NavigationPipeline::new(config))
        });
    });
}

fn bench_health_report(c: &mut Criterion) {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    c.bench_function("build_health_report", |b| {
        b.iter(|| black_box(build_health_report(&pipeline)));
    });
}

fn bench_health_report_serialise(c: &mut Criterion) {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    let report = build_health_report(&pipeline);
    c.bench_function("health_report_serialise_json", |b| {
        b.iter(|| black_box(serde_json::to_string(&report).unwrap()));
    });
}

fn bench_pipeline_tracked_satellites(c: &mut Criterion) {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    c.bench_function("pipeline_tracked_satellites", |b| {
        b.iter(|| black_box(pipeline.tracked_satellites()));
    });
}

criterion_group!(
    benches,
    bench_pipeline_creation,
    bench_health_report,
    bench_health_report_serialise,
    bench_pipeline_tracked_satellites,
);
criterion_main!(benches);
