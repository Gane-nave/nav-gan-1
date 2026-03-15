//! Benchmarks for the EKF fusion engine — predict/update cycles.
#![allow(unknown_lints)]
#![allow(clippy::manual_is_multiple_of)]

use aurora_fusion::ekf::NavigationEkf;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_ekf_creation(c: &mut Criterion) {
    c.bench_function("NavigationEkf::new", |b| {
        b.iter(|| black_box(NavigationEkf::new()));
    });
}

fn bench_ekf_predict(c: &mut Criterion) {
    let mut ekf = NavigationEkf::new();
    c.bench_function("ekf_predict_1s", |b| {
        b.iter(|| ekf.predict(black_box(1.0)));
    });
}

fn bench_ekf_update_position(c: &mut Criterion) {
    let mut ekf = NavigationEkf::new();
    ekf.predict(1.0);
    c.bench_function("ekf_update_position", |b| {
        b.iter(|| {
            ekf.update_position(
                black_box(100.0),
                black_box(200.0),
                black_box(50.0),
                black_box(5.0),
            );
        });
    });
}

fn bench_ekf_update_velocity(c: &mut Criterion) {
    let mut ekf = NavigationEkf::new();
    ekf.predict(1.0);
    c.bench_function("ekf_update_velocity", |b| {
        b.iter(|| {
            ekf.update_velocity(
                black_box(10.0),
                black_box(5.0),
                black_box(0.0),
                black_box(1.0),
            );
        });
    });
}

fn bench_ekf_update_heading(c: &mut Criterion) {
    let mut ekf = NavigationEkf::new();
    ekf.predict(1.0);
    c.bench_function("ekf_update_heading", |b| {
        b.iter(|| {
            ekf.update_heading(black_box(1.57), black_box(0.1));
        });
    });
}

fn bench_ekf_full_cycle(c: &mut Criterion) {
    c.bench_function("ekf_full_predict_update_cycle", |b| {
        b.iter(|| {
            let mut ekf = NavigationEkf::new();
            for _ in 0..10 {
                ekf.predict(0.1);
                ekf.update_position(100.0, 200.0, 50.0, 5.0);
                ekf.update_velocity(10.0, 5.0, 0.0, 1.0);
                ekf.update_heading(1.57, 0.1);
            }
            black_box(ekf.position_enu())
        });
    });
}

fn bench_ekf_convergence(c: &mut Criterion) {
    c.bench_function("ekf_convergence_50_epochs", |b| {
        b.iter(|| {
            let mut ekf = NavigationEkf::new();
            for i in 0..50 {
                ekf.predict(1.0);
                ekf.update_position(100.0, 200.0, 50.0, 2.0);
                if i % 5 == 0 {
                    ekf.update_speed(15.0, 0.5);
                }
            }
            black_box(ekf.position_uncertainty_m())
        });
    });
}

criterion_group!(
    benches,
    bench_ekf_creation,
    bench_ekf_predict,
    bench_ekf_update_position,
    bench_ekf_update_velocity,
    bench_ekf_update_heading,
    bench_ekf_full_cycle,
    bench_ekf_convergence,
);
criterion_main!(benches);
