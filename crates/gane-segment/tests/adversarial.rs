//! Adversarial tests for gane-segment.

use gane_segment::SegmentTree;

#[test]
fn adversarial_large_tree() {
    let data: Vec<i64> = (1..=1000).collect();
    let mut st = SegmentTree::from_slice(&data);
    // Sum of 1..=1000 = 500500
    assert_eq!(st.total_sum(), 500_500);
    assert_eq!(st.query(0, 999), 500_500);
}

#[test]
fn adversarial_all_zeros() {
    let mut st = SegmentTree::new(100);
    assert_eq!(st.total_sum(), 0);
    for i in 0..100 {
        assert_eq!(st.get(i), 0);
    }
}

#[test]
fn adversarial_negative_values() {
    let data = vec![-100, -50, 0, 50, 100];
    let mut st = SegmentTree::from_slice(&data);
    assert_eq!(st.total_sum(), 0);
    assert_eq!(st.query(0, 1), -150);
    assert_eq!(st.query(3, 4), 150);
}

#[test]
fn adversarial_single_element_repeated_updates() {
    let mut st = SegmentTree::from_slice(&[0]);
    for i in 0..1000i64 {
        st.update(0, i);
        assert_eq!(st.get(0), i);
    }
}

#[test]
fn adversarial_alternating_values() {
    let data: Vec<i64> = (0..100).map(|i| if i % 2 == 0 { 1 } else { -1 }).collect();
    let mut st = SegmentTree::from_slice(&data);
    assert_eq!(st.total_sum(), 0); // 50 ones + 50 negative ones
}

#[test]
fn adversarial_update_all_elements() {
    let mut st = SegmentTree::new(50);
    for i in 0..50 {
        st.update(i, 10);
    }
    assert_eq!(st.total_sum(), 500);
}

#[test]
#[should_panic]
fn adversarial_query_out_of_bounds() {
    let mut st = SegmentTree::from_slice(&[1, 2, 3]);
    st.query(0, 10);
}

#[test]
fn adversarial_large_values() {
    let data = vec![i64::MAX / 4, i64::MAX / 4, i64::MAX / 4, i64::MAX / 4];
    let mut st = SegmentTree::from_slice(&data);
    let sum = st.total_sum();
    // Should not panic, uses saturating_add
    assert!(sum > 0);
}
