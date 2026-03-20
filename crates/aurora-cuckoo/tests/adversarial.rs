//! Adversarial tests for aurora-cuckoo.

use aurora_cuckoo::CuckooFilter;

#[test]
fn adversarial_empty_filter() {
    let cf = CuckooFilter::new(100);
    assert!(cf.is_empty());
    assert!(!cf.contains(&42));
}

#[test]
fn adversarial_insert_remove_cycle() {
    let mut cf = CuckooFilter::new(100);
    for i in 0..50 {
        cf.insert(&i);
    }
    assert_eq!(cf.len(), 50);
    for i in 0..50 {
        assert!(cf.remove(&i));
    }
    assert!(cf.is_empty());
}

#[test]
fn adversarial_capacity_stress() {
    let mut cf = CuckooFilter::new(16);
    let mut inserted = 0;
    for i in 0..100 {
        if cf.insert(&i) {
            inserted += 1;
        }
    }
    assert!(inserted > 0);
}

#[test]
fn adversarial_remove_nonexistent() {
    let mut cf = CuckooFilter::new(100);
    cf.insert(&1);
    assert!(!cf.remove(&999));
    assert_eq!(cf.len(), 1);
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut cf = CuckooFilter::new(100);
    for i in 0..50 {
        cf.insert(&i);
    }
    cf.clear();
    assert!(cf.is_empty());
    cf.insert(&999);
    assert!(cf.contains(&999));
    assert_eq!(cf.len(), 1);
}

#[test]
fn adversarial_load_factor_bounds() {
    let mut cf = CuckooFilter::new(100);
    assert_eq!(cf.load_factor(), 0.0);
    cf.insert(&1);
    let lf = cf.load_factor();
    assert!(lf > 0.0);
    assert!(lf <= 1.0);
}

#[test]
fn adversarial_many_types() {
    let mut cf = CuckooFilter::new(100);
    cf.insert(&42_i32);
    cf.insert(&"hello");
    cf.insert(&true);
    assert_eq!(cf.len(), 3);
}

#[test]
fn adversarial_min_capacity() {
    let mut cf = CuckooFilter::new(0);
    cf.insert(&1);
    assert!(cf.contains(&1));
}
