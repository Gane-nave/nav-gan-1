//! Adversarial tests for aurora-rcu.

use aurora_rcu::RcuCell;

#[test]
fn adversarial_empty_read() {
    let cell = RcuCell::new(0);
    assert_eq!(*cell.read(), 0);
    assert_eq!(cell.version_count(), 1);
}

#[test]
fn adversarial_many_updates() {
    let mut cell = RcuCell::new(0_u64);
    for i in 1..=1000 {
        cell.update(|v| v + 1);
        assert_eq!(*cell.read(), i);
    }
}

#[test]
fn adversarial_max_versions_respected() {
    let mut cell = RcuCell::with_max_versions(0, 3);
    for _ in 0..100 {
        cell.update(|v| v + 1);
    }
    assert!(cell.version_count() <= 3);
    assert_eq!(*cell.read(), 100);
}

#[test]
fn adversarial_reclaim_preserves_current() {
    let mut cell = RcuCell::new(0);
    cell.update(|v| v + 42);
    cell.reclaim();
    assert_eq!(*cell.read(), 42);
    assert_eq!(cell.version_count(), 1);
}

#[test]
fn adversarial_arc_snapshot_survives_update() {
    let mut cell = RcuCell::new(100);
    let snapshot = cell.read_arc();
    cell.update(|v| v + 1);
    assert_eq!(*snapshot, 100);
    assert_eq!(*cell.read(), 101);
}

#[test]
fn adversarial_replace_replaces() {
    let mut cell = RcuCell::new(String::from("old"));
    cell.replace(String::from("new"));
    assert_eq!(cell.read().as_str(), "new");
}

#[test]
fn adversarial_version_history_order() {
    let mut cell = RcuCell::new(0);
    cell.update(|v| v + 10);
    cell.update(|v| v + 20);
    let history = cell.version_history();
    assert_eq!(*history[0], 0);
    assert_eq!(*history[1], 10);
    assert_eq!(*history[2], 30);
}

#[test]
fn adversarial_min_max_versions() {
    let mut cell = RcuCell::with_max_versions(0, 0); // should clamp to 1
    assert_eq!(cell.max_versions(), 1);
    cell.update(|v| v + 1);
    assert!(cell.version_count() <= 1);
}
