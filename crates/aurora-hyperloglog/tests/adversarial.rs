//! Adversarial tests for aurora-hyperloglog.

use aurora_hyperloglog::HyperLogLog;

#[test]
fn adversarial_empty() {
    let hll = HyperLogLog::new(10);
    assert_eq!(hll.estimate(), 0.0);
    assert_eq!(hll.num_registers(), 1024);
}

#[test]
fn adversarial_precision_bounds() {
    let low = HyperLogLog::new(0);
    assert_eq!(low.precision(), 4);
    let high = HyperLogLog::new(255);
    assert_eq!(high.precision(), 16);
}

#[test]
fn adversarial_merge_different_precision() {
    let mut hll1 = HyperLogLog::new(10);
    let hll2 = HyperLogLog::new(12);
    hll1.add(&1);
    hll1.merge(&hll2); // should be no-op
    assert!(hll1.estimate() > 0.0);
}

#[test]
fn adversarial_large_cardinality() {
    let mut hll = HyperLogLog::new(14);
    for i in 0..10_000 {
        hll.add(&i);
    }
    let est = hll.estimate();
    assert!(est > 8_000.0, "estimate {} too low for 10k items", est);
    assert!(est < 12_000.0, "estimate {} too high for 10k items", est);
}

#[test]
fn adversarial_all_same_items() {
    let mut hll = HyperLogLog::new(10);
    for _ in 0..10_000 {
        hll.add(&"same");
    }
    assert!(hll.estimate() < 5.0);
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut hll = HyperLogLog::new(10);
    for i in 0..100 {
        hll.add(&i);
    }
    hll.clear();
    assert_eq!(hll.estimate(), 0.0);
    hll.add(&999);
    assert!(hll.estimate() > 0.0);
}

#[test]
fn adversarial_merge_self_like() {
    let mut hll1 = HyperLogLog::new(10);
    let mut hll2 = HyperLogLog::new(10);
    for i in 0..100 {
        hll1.add(&i);
        hll2.add(&i);
    }
    let est_before = hll1.estimate();
    hll1.merge(&hll2);
    let est_after = hll1.estimate();
    let diff = (est_after - est_before).abs();
    assert!(
        diff < est_before * 0.5,
        "merge of same set changed estimate too much"
    );
}

#[test]
fn adversarial_memory_usage() {
    let hll = HyperLogLog::new(4);
    assert_eq!(hll.memory_bytes(), 16);
    let hll = HyperLogLog::new(16);
    assert_eq!(hll.memory_bytes(), 65536);
}
