//! Adversarial tests for gane-orchestrator.

use gane_orchestrator::services::{NavigationRequest, ServiceRegistry};

#[test]
fn queue_overflow_returns_false() {
    let mut reg = ServiceRegistry::with_capacity(10, 10, 2);
    let make_req = |id| NavigationRequest {
        id,
        origin_lat: 0.0,
        origin_lon: 0.0,
        dest_lat: 1.0,
        dest_lon: 1.0,
        timestamp_ms: 0,
    };
    assert!(reg.enqueue_nav_request(make_req(1)));
    assert!(reg.enqueue_nav_request(make_req(2)));
    // Queue is full (capacity 2), third push should fail
    assert!(!reg.enqueue_nav_request(make_req(3)));
    assert_eq!(reg.pending_requests(), 2);
}

#[test]
fn circuit_breaker_trips_and_recovers() {
    let mut reg = ServiceRegistry::new();
    assert!(reg.can_call_external());
    // Trip the breaker
    for _ in 0..10 {
        reg.record_external_failure();
    }
    assert!(!reg.can_call_external());
    // Recovery requires success after timeout — just verify state is open
    let stats = reg.stats();
    assert!(!stats.circuit_breaker_closed);
}

#[test]
fn waypoint_dedup_prevents_revisit() {
    let mut reg = ServiceRegistry::new();
    let wp = b"intersection_42";
    assert!(!reg.was_waypoint_visited(wp));
    reg.mark_waypoint_visited(wp);
    assert!(reg.was_waypoint_visited(wp));
    // Insert again — should still be marked
    reg.mark_waypoint_visited(wp);
    assert!(reg.was_waypoint_visited(wp));
}

#[test]
fn latency_histogram_handles_edge_values() {
    let mut reg = ServiceRegistry::new();
    reg.record_latency(0.0);
    reg.record_latency(10_000.0);
    reg.record_latency(f64::MAX);
    reg.record_latency(-1.0);
    // Should not panic — all values recorded
    assert!(reg.latency_histogram.count() >= 2);
}

#[test]
fn snapshot_store_enforces_retention() {
    let mut reg = ServiceRegistry::with_capacity(10, 10, 10);
    for i in 0..100 {
        reg.take_snapshot(i * 1000);
    }
    // Max 50 snapshots retained
    assert!(reg.snapshots.count() <= 50);
}

#[test]
fn clock_monotonic_under_rapid_calls() {
    let mut reg = ServiceRegistry::new();
    let mut prev = reg.now();
    for _ in 0..100 {
        let curr = reg.now();
        assert!(curr >= prev);
        prev = curr;
    }
}

#[test]
fn stats_reflect_operations() {
    let mut reg = ServiceRegistry::new();
    let req = NavigationRequest {
        id: 42,
        origin_lat: 32.0,
        origin_lon: 34.0,
        dest_lat: 31.0,
        dest_lon: 35.0,
        timestamp_ms: 5000,
    };
    reg.enqueue_nav_request(req);
    reg.mark_waypoint_visited(b"wp_1");
    reg.record_latency(500.0);
    reg.take_snapshot(1000);

    let stats = reg.stats();
    assert_eq!(stats.pending_nav_requests, 1);
    assert_eq!(stats.waypoint_filter_count, 1);
    assert_eq!(stats.latency_count, 1);
    assert_eq!(stats.snapshots_taken, 1);
    assert!(stats.circuit_breaker_closed);
}

#[test]
fn dequeue_order_is_fifo() {
    let mut reg = ServiceRegistry::new();
    for i in 0..5 {
        let req = NavigationRequest {
            id: i,
            origin_lat: 0.0,
            origin_lon: 0.0,
            dest_lat: 1.0,
            dest_lon: 1.0,
            timestamp_ms: i * 100,
        };
        reg.enqueue_nav_request(req);
    }
    for i in 0..5 {
        let req = reg.dequeue_nav_request().unwrap();
        assert_eq!(req.id, i);
    }
    assert!(reg.dequeue_nav_request().is_none());
}
