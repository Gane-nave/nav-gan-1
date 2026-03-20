//! Adversarial tests for aurora-skiplist

use aurora_skiplist::list::SkipList;

#[test]
fn adversarial_sorted_insert_range_update_remove() {
    let mut sl = SkipList::new();

    // Insert keys in reverse order — must still be sorted
    for i in (0..20).rev() {
        sl.insert(i * 10, i as u32);
    }
    assert_eq!(sl.len(), 20);
    let keys = sl.keys();
    assert_eq!(keys.len(), 20);
    // Verify sorted order
    for w in keys.windows(2) {
        assert!(w[0] < w[1], "Keys must be sorted: {} < {}", w[0], w[1]);
    }

    // First and last
    assert_eq!(sl.first(), Some((0, &0u32)));
    assert_eq!(sl.last(), Some((190, &19u32)));

    // Range query [50, 120) should return keys 50, 60, 70, 80, 90, 100, 110
    let range = sl.range(50, 120);
    assert_eq!(range.len(), 7);
    assert_eq!(range[0], (50, 5));
    assert_eq!(range[6], (110, 11));

    // Update existing key — should return old value
    let old = sl.insert(100, 999);
    assert_eq!(old, Some(10)); // old value was 10
    assert_eq!(sl.get(100), Some(&999));
    assert_eq!(sl.len(), 20); // no new entry

    // Remove middle key
    let removed = sl.remove(100);
    assert_eq!(removed, Some(999));
    assert_eq!(sl.len(), 19);
    assert_eq!(sl.get(100), None);
    assert!(!sl.contains(100));

    // Remove non-existent
    assert_eq!(sl.remove(999), None);

    // Stats
    assert_eq!(sl.total_inserts(), 21); // 20 + 1 update
    assert_eq!(sl.total_removes(), 1);
    assert!(sl.hit_rate() > 0.0);
}
