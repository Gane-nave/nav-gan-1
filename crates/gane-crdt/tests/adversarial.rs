//! Adversarial tests for gane-crdt.

use gane_crdt::{GCounter, LWWRegister, ORSet, PNCounter};

#[test]
fn adversarial_gcounter_empty() {
    let c = GCounter::new();
    assert_eq!(c.value(), 0);
    assert_eq!(c.node_count("nonexistent"), 0);
}

#[test]
fn adversarial_gcounter_saturating() {
    let mut c = GCounter::new();
    c.increment_by("a", u64::MAX);
    c.increment("a");
    assert_eq!(c.node_count("a"), u64::MAX);
}

#[test]
fn adversarial_pncounter_negative() {
    let mut c = PNCounter::new();
    c.decrement("a");
    c.decrement("a");
    assert_eq!(c.value(), -2);
}

#[test]
fn adversarial_lww_same_timestamp() {
    let mut r = LWWRegister::new();
    r.set("first", 5);
    r.set("second", 5);
    assert_eq!(r.get(), Some(&"second"));
}

#[test]
fn adversarial_lww_empty() {
    let r: LWWRegister<String> = LWWRegister::new();
    assert_eq!(r.get(), None);
    assert_eq!(r.timestamp(), 0);
}

#[test]
fn adversarial_orset_double_insert() {
    let mut s = ORSet::new();
    s.insert("x");
    s.insert("x");
    assert!(s.contains(&"x"));
    s.remove(&"x");
    assert!(!s.contains(&"x"));
}

#[test]
fn adversarial_orset_clear() {
    let mut s = ORSet::new();
    s.insert(1);
    s.insert(2);
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn adversarial_orset_remove_nonexistent() {
    let mut s = ORSet::new();
    s.remove(&"nonexistent");
    assert!(s.is_empty());
}
