//! Adversarial tests for gane-slab.

use gane_slab::Slab;

#[test]
fn adversarial_insert_remove_reuse() {
    let mut slab = Slab::new();
    let mut keys = Vec::new();
    for i in 0..100u32 {
        keys.push(slab.insert(i));
    }
    // Remove even indices
    for &k in keys.iter().step_by(2) {
        slab.remove(k);
    }
    assert_eq!(slab.len(), 50);
    // Insert more — should reuse freed slots
    let mut new_keys = Vec::new();
    for i in 100..150u32 {
        new_keys.push(slab.insert(i));
    }
    // New keys should reuse old slots
    for &k in &new_keys {
        assert!(slab.contains(k));
    }
    assert_eq!(slab.len(), 100);
}

#[test]
fn adversarial_remove_nonexistent() {
    let mut slab: Slab<u32> = Slab::new();
    assert_eq!(slab.remove(0), None);
    assert_eq!(slab.remove(999), None);
}

#[test]
fn adversarial_double_remove() {
    let mut slab = Slab::new();
    let idx = slab.insert(42u32);
    assert_eq!(slab.remove(idx), Some(42));
    assert_eq!(slab.remove(idx), None); // already removed
}

#[test]
fn adversarial_compact() {
    let mut slab = Slab::new();
    for i in 0..10u32 {
        slab.insert(i);
    }
    // Remove last 5
    for i in 5..10 {
        slab.remove(i);
    }
    let reclaimed = slab.compact();
    assert_eq!(reclaimed, 5);
    assert_eq!(slab.capacity(), 5);
    assert_eq!(slab.len(), 5);
}

#[test]
fn adversarial_compact_with_gaps() {
    let mut slab = Slab::new();
    for i in 0..10u32 {
        slab.insert(i);
    }
    // Remove middle elements
    slab.remove(3);
    slab.remove(5);
    slab.remove(7);
    let reclaimed = slab.compact();
    // Only trailing free slots compacted
    assert_eq!(reclaimed, 0); // slot 9 is occupied, no trailing free
    assert_eq!(slab.len(), 7);
}

#[test]
fn adversarial_iter_with_gaps() {
    let mut slab = Slab::new();
    slab.insert(10u32);
    slab.insert(20);
    slab.insert(30);
    slab.insert(40);
    slab.remove(1);
    slab.remove(3);
    let items: Vec<(usize, &u32)> = slab.iter().collect();
    assert_eq!(items.len(), 2);
    assert_eq!(*items[0].1, 10);
    assert_eq!(*items[1].1, 30);
}

#[test]
fn adversarial_large_slab() {
    let mut slab = Slab::new();
    for i in 0..10_000u64 {
        slab.insert(i);
    }
    assert_eq!(slab.len(), 10_000);
    // Remove all
    for i in 0..10_000 {
        slab.remove(i);
    }
    assert!(slab.is_empty());
    assert_eq!(slab.free_count(), 10_000);
}

#[test]
fn adversarial_get_after_remove() {
    let mut slab = Slab::new();
    let idx = slab.insert("hello");
    slab.remove(idx);
    assert_eq!(slab.get(idx), None);
    assert_eq!(slab.get_mut(idx), None);
}
