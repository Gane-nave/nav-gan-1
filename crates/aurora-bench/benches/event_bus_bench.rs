//! Benchmarks for the event bus publish/subscribe throughput.

use aurora_events::bus::EventBus;
use aurora_events::envelope::EventEnvelope;
use aurora_events::event_type::EventType;
use aurora_events::subscriber::EventHandler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use uuid::Uuid;

struct NoOpHandler;

impl EventHandler for NoOpHandler {
    fn handle(&self, _event: &EventEnvelope) {}
    fn subscribed_types(&self) -> Option<Vec<EventType>> {
        None
    }
}

struct CountingHandler {
    count: AtomicU64,
}

impl CountingHandler {
    fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
        }
    }
}

impl EventHandler for CountingHandler {
    fn handle(&self, _event: &EventEnvelope) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
    fn subscribed_types(&self) -> Option<Vec<EventType>> {
        None
    }
}

struct FilteredHandler;

impl EventHandler for FilteredHandler {
    fn handle(&self, _event: &EventEnvelope) {}
    fn subscribed_types(&self) -> Option<Vec<EventType>> {
        Some(vec![EventType::PositionUpdate])
    }
}

fn make_event(et: EventType) -> EventEnvelope {
    EventEnvelope::new("bench", "Bench", Uuid::new_v4(), et, serde_json::json!({}))
}

fn bench_bus_creation(c: &mut Criterion) {
    c.bench_function("EventBus::new", |b| {
        b.iter(|| black_box(EventBus::new()));
    });
}

fn bench_publish_no_subscribers(c: &mut Criterion) {
    let bus = EventBus::new();
    let event = make_event(EventType::PositionUpdate);
    c.bench_function("publish_no_subscribers", |b| {
        b.iter(|| bus.publish(black_box(&event)));
    });
}

fn bench_publish_1_subscriber(c: &mut Criterion) {
    let bus = EventBus::new();
    bus.subscribe(Arc::new(NoOpHandler));
    let event = make_event(EventType::PositionUpdate);
    c.bench_function("publish_1_subscriber", |b| {
        b.iter(|| bus.publish(black_box(&event)));
    });
}

fn bench_publish_10_subscribers(c: &mut Criterion) {
    let bus = EventBus::new();
    for _ in 0..10 {
        bus.subscribe(Arc::new(NoOpHandler));
    }
    let event = make_event(EventType::PositionUpdate);
    c.bench_function("publish_10_subscribers", |b| {
        b.iter(|| bus.publish(black_box(&event)));
    });
}

fn bench_publish_filtered(c: &mut Criterion) {
    let bus = EventBus::new();
    for _ in 0..5 {
        bus.subscribe(Arc::new(FilteredHandler));
    }
    for _ in 0..5 {
        bus.subscribe(Arc::new(NoOpHandler));
    }
    let matching = make_event(EventType::PositionUpdate);
    let non_matching = make_event(EventType::GnssMeasurementReceived);

    c.bench_function("publish_matching_event", |b| {
        b.iter(|| bus.publish(black_box(&matching)));
    });

    c.bench_function("publish_non_matching_event", |b| {
        b.iter(|| bus.publish(black_box(&non_matching)));
    });
}

fn bench_subscribe_unsubscribe(c: &mut Criterion) {
    let bus = EventBus::new();
    c.bench_function("subscribe_unsubscribe_cycle", |b| {
        b.iter(|| {
            let sub = bus.subscribe(Arc::new(NoOpHandler));
            bus.unsubscribe(&sub);
        });
    });
}

fn bench_burst_publish(c: &mut Criterion) {
    let bus = EventBus::new();
    let handler = Arc::new(CountingHandler::new());
    bus.subscribe(handler);
    let event = make_event(EventType::PositionUpdate);

    c.bench_function("burst_100_events", |b| {
        b.iter(|| {
            for _ in 0..100 {
                bus.publish(black_box(&event));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_bus_creation,
    bench_publish_no_subscribers,
    bench_publish_1_subscriber,
    bench_publish_10_subscribers,
    bench_publish_filtered,
    bench_subscribe_unsubscribe,
    bench_burst_publish,
);
criterion_main!(benches);
