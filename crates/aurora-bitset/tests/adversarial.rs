//! Adversarial tests for aurora-bitset

use aurora_bitset::bits::BitSet;
use aurora_bitset::ops;

#[test]
fn adversarial_set_ops_jaccard_subset() {
    // Create two overlapping sets
    let mut a = BitSet::new(256);
    let mut b = BitSet::new(256);

    // a = {0, 10, 20, 30, 40, 50, 64, 128}
    for &i in &[0, 10, 20, 30, 40, 50, 64, 128] {
        a.set(i);
    }
    // b = {10, 30, 50, 64, 100, 200}
    for &i in &[10, 30, 50, 64, 100, 200] {
        b.set(i);
    }

    assert_eq!(a.count(), 8);
    assert_eq!(b.count(), 6);

    // AND: intersection = {10, 30, 50, 64}
    let inter = ops::and(&a, &b);
    assert_eq!(inter.iter_set(), vec![10, 30, 50, 64]);
    assert_eq!(inter.count(), 4);

    // OR: union = {0, 10, 20, 30, 40, 50, 64, 100, 128, 200}
    let union_set = ops::or(&a, &b);
    assert_eq!(union_set.count(), 10);
    assert!(union_set.test(0));
    assert!(union_set.test(200));

    // XOR: symmetric difference = {0, 20, 40, 100, 128, 200}
    let sym_diff = ops::xor(&a, &b);
    assert_eq!(sym_diff.count(), 6);
    assert!(sym_diff.test(0));
    assert!(!sym_diff.test(10)); // in both — excluded

    // Difference: a - b = {0, 20, 40, 128}
    let diff = ops::difference(&a, &b);
    assert_eq!(diff.iter_set(), vec![0, 20, 40, 128]);

    // Subset: inter ⊆ a, inter ⊆ b
    assert!(ops::is_subset(&inter, &a));
    assert!(ops::is_subset(&inter, &b));
    assert!(!ops::is_subset(&a, &b)); // a has elements not in b

    // Intersection count
    assert_eq!(ops::intersection_count(&a, &b), 4);

    // Jaccard: |A ∩ B| / |A ∪ B| = 4/10 = 0.4
    let j = ops::jaccard(&a, &b);
    assert!((j - 0.4).abs() < f64::EPSILON, "Jaccard should be 0.4, got {}", j);

    // first_set / last_set
    assert_eq!(a.first_set(), Some(0));
    assert_eq!(a.last_set(), Some(128));

    // set_all then clear_all
    let mut c = BitSet::new(100);
    c.set_all();
    assert_eq!(c.count(), 100);
    assert!(c.test(99));
    assert!(!c.test(100)); // out of bounds
    c.clear_all();
    assert_eq!(c.count(), 0);
    assert!(!c.test(0));
}
