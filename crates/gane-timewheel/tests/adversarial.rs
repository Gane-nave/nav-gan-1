//! Adversarial tests for gane-timewheel.

use gane_timewheel::TimerWheel;

#[test]
fn adversarial_cancel_after_fire() {
    let mut w = TimerWheel::new(16);
    let id = w.schedule(2, "test");
    w.advance(2); // fires
    assert!(!w.cancel(id), "cancelling already-fired timer should fail");
}

#[test]
fn adversarial_many_timers_same_slot() {
    let mut w = TimerWheel::new(4);
    let mut ids = Vec::new();
    for i in 0..100 {
        ids.push(w.schedule(4, &format!("t{}", i)));
    }
    assert_eq!(w.pending(), 100);
    let fired = w.advance(4);
    assert_eq!(fired.len(), 100);
    assert!(w.is_empty());
}

#[test]
fn adversarial_large_delay_wrap() {
    let mut w = TimerWheel::new(8);
    let id = w.schedule(1000, "far-future");
    assert!(w.has_timer(id));
    let fired = w.advance(1000);
    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0].label, "far-future");
}

#[test]
fn adversarial_cancel_all() {
    let mut w = TimerWheel::new(16);
    let ids: Vec<u64> = (0..50).map(|i| w.schedule(i + 1, "x")).collect();
    for id in ids {
        assert!(w.cancel(id));
    }
    assert!(w.is_empty());
    let fired = w.advance(100);
    assert!(fired.is_empty());
}

#[test]
fn adversarial_interleaved_schedule_cancel() {
    let mut w = TimerWheel::new(16);
    for i in 0..100u64 {
        let id = w.schedule(5, &format!("t{}", i));
        if i % 2 == 0 {
            w.cancel(id);
        }
    }
    let fired = w.advance(5);
    assert_eq!(fired.len(), 50, "only odd-indexed timers should fire");
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut w = TimerWheel::new(16);
    for i in 0..20 {
        w.schedule(i + 1, "old");
    }
    w.clear();
    assert!(w.is_empty());
    let _id = w.schedule(1, "new");
    let fired = w.advance(1);
    assert_eq!(fired.len(), 1);
    assert_eq!(fired[0].label, "new");
}

#[test]
fn adversarial_stats_accuracy() {
    let mut w = TimerWheel::new(16);
    for _ in 0..10 {
        w.schedule(1, "fire");
    }
    for _ in 0..5 {
        let id = w.schedule(10, "cancel");
        w.cancel(id);
    }
    w.advance(1);
    assert_eq!(w.total_scheduled(), 15);
    assert_eq!(w.total_fired(), 10);
    assert_eq!(w.total_cancelled(), 5);
}

#[test]
#[should_panic]
fn adversarial_zero_slots() {
    TimerWheel::new(0);
}
