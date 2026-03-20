//! Adversarial tests for aurora-debounce

use aurora_debounce::batch::BatchCollector;
use aurora_debounce::debouncer::Debouncer;
use aurora_debounce::throttle::Throttle;

#[test]
fn adversarial_debounce_rapid_suppress_and_fire() {
    let mut d = Debouncer::new(500);

    // Rapid events — should suppress previous ones
    d.event(100);
    d.event(200);
    d.event(300);
    d.event(400);

    // Not enough quiet time
    assert!(!d.check_and_fire(800)); // 400ms since last event

    // Enough quiet time
    assert!(d.check_and_fire(900)); // 500ms since t=400

    // Already fired — should not fire again
    assert!(!d.check_and_fire(1000));

    assert_eq!(d.total_events(), 4);
    assert_eq!(d.total_suppressed(), 3);
    assert_eq!(d.total_fired(), 1);
    assert!((d.suppression_ratio() - 0.75).abs() < f64::EPSILON);
}

#[test]
fn adversarial_throttle_rate_limiting_with_reset() {
    let mut t = Throttle::new(1000);

    assert!(t.try_allow(100)); // allowed (first)
    assert!(!t.try_allow(500)); // throttled
    assert!(!t.try_allow(900)); // throttled
    assert!(t.try_allow(1100)); // allowed (1000ms since t=100)

    // Reset clears state
    t.reset();
    assert!(t.try_allow(1200)); // allowed immediately after reset

    assert_eq!(t.total_allowed(), 3);
    assert_eq!(t.total_throttled(), 2);
    assert!((t.throttle_ratio() - 0.4).abs() < f64::EPSILON);
}

#[test]
fn adversarial_batch_time_and_size_flush() {
    let mut bc = BatchCollector::new(3, 5000);

    // Add 2 events — not full yet
    assert!(!bc.add(vec![1, 2], 1000));
    assert!(!bc.add(vec![3], 2000));
    assert!(!bc.should_flush(3000)); // neither threshold

    // Time-based flush at t=6000 (5000ms since first event at t=1000)
    assert!(bc.should_flush_by_time(6000));
    let batch1 = bc.flush();
    assert_eq!(batch1.len(), 2);
    assert_eq!(bc.total_bytes_flushed(), 3); // 2 + 1 bytes

    // Size-based flush — add 3 events
    assert!(!bc.add(vec![4, 5, 6], 7000));
    assert!(!bc.add(vec![7], 8000));
    assert!(bc.add(vec![8, 9], 9000)); // full at max_size=3
    let batch2 = bc.flush();
    assert_eq!(batch2.len(), 3);

    assert_eq!(bc.total_flushes(), 2);
    assert_eq!(bc.total_events(), 5);
    assert_eq!(bc.total_bytes_flushed(), 9); // 3 + 3+1+2=6
}
