//! Adversarial tests for aurora-clock.

use aurora_clock::HybridClock;

#[test]
fn adversarial_empty_clock() {
    let clock = HybridClock::new(0);
    let ts = clock.current();
    assert_eq!(ts.physical, 0);
    assert_eq!(ts.logical, 0);
    assert_eq!(ts.node_id, 0);
}

#[test]
fn adversarial_monotonic_same_wall() {
    let mut clock = HybridClock::new(1);
    let mut prev = clock.now(100);
    for _ in 0..100 {
        let next = clock.now(100);
        assert!(next > prev);
        prev = next;
    }
}

#[test]
fn adversarial_wall_clock_regression() {
    let mut clock = HybridClock::new(1);
    let t1 = clock.now(200);
    let t2 = clock.now(100); // wall clock went backwards
    assert!(t2 > t1); // logical clock should compensate
    assert_eq!(t2.physical, 200);
}

#[test]
fn adversarial_max_drift_rejection() {
    let mut clock = HybridClock::with_max_drift(1, 50);
    clock.now(100);
    // Receive from same time — should be fine
    let current = clock.current();
    let result = clock.receive(100, &current);
    assert!(result.is_some());
}

#[test]
fn adversarial_logical_saturating() {
    let mut clock = HybridClock::new(1);
    // Generate many events at same wall clock
    for _ in 0..10000 {
        clock.now(0);
    }
    let ts = clock.current();
    assert!(ts.logical > 0);
}

#[test]
fn adversarial_receive_updates_physical() {
    let mut clock1 = HybridClock::new(1);
    let mut clock2 = HybridClock::new(2);
    clock1.now(50);
    let remote_ts = clock2.now(200);
    let result = clock1.receive(60, &remote_ts);
    assert!(result.is_some());
    let ts = result.unwrap();
    assert_eq!(ts.physical, 200);
}

#[test]
fn adversarial_node_id_ordering() {
    let mut c1 = HybridClock::new(1);
    let mut c2 = HybridClock::new(2);
    let t1 = c1.now(100);
    let t2 = c2.now(100);
    // Same physical, same logical, different node_id
    assert!(t1 < t2); // node_id 1 < node_id 2
}

#[test]
fn adversarial_max_drift_zero() {
    let mut clock = HybridClock::with_max_drift(1, 0);
    assert_eq!(clock.max_drift(), 0);
    let ts = clock.now(100);
    assert_eq!(ts.physical, 100);
}
