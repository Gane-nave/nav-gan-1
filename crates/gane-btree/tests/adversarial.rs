//! Adversarial tests for gane-btree.

use gane_btree::BTree;

#[test]
fn adversarial_empty_operations() {
    let tree: BTree<i32, i32> = BTree::new(2);
    assert!(tree.is_empty());
    assert_eq!(tree.get(&0), None);
    assert_eq!(tree.rank(&0), None);
    assert_eq!(tree.select(0), None);
    assert_eq!(tree.min_key(), None);
    assert_eq!(tree.max_key(), None);
    assert!(tree.range(&0, &100).is_empty());
}

#[test]
fn adversarial_large_sequential_insert() {
    let mut tree = BTree::new(2);
    for i in 0..1000 {
        tree.insert(i, i * 10);
    }
    assert_eq!(tree.len(), 1000);
    for i in 0..1000 {
        assert_eq!(tree.get(&i), Some(&(i * 10)));
        assert_eq!(tree.rank(&i), Some(i));
        assert_eq!(tree.select(i), Some(&i));
    }
}

#[test]
fn adversarial_reverse_insert() {
    let mut tree = BTree::new(3);
    for i in (0..500).rev() {
        tree.insert(i, i);
    }
    assert_eq!(tree.len(), 500);
    let sorted = tree.keys_sorted();
    for (i, key) in sorted.iter().enumerate().take(500) {
        assert_eq!(*key, i);
    }
}

#[test]
fn adversarial_duplicate_keys() {
    let mut tree = BTree::new(2);
    tree.insert(42, "first");
    let old = tree.insert(42, "second");
    assert_eq!(old, Some("first"));
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get(&42), Some(&"second"));
}

#[test]
fn adversarial_min_order() {
    let mut tree = BTree::new(1); // Should clamp to 2
    for i in 0..100 {
        tree.insert(i, i);
    }
    assert_eq!(tree.len(), 100);
    assert_eq!(tree.min_key(), Some(&0));
    assert_eq!(tree.max_key(), Some(&99));
}

#[test]
fn adversarial_large_order() {
    let mut tree = BTree::new(50);
    for i in 0..200 {
        tree.insert(i, i);
    }
    assert_eq!(tree.len(), 200);
    let stats = tree.stats();
    assert!(stats.height <= 3);
}

#[test]
fn adversarial_range_empty_result() {
    let mut tree = BTree::new(2);
    for i in 0..10 {
        tree.insert(i * 10, i);
    }
    let range = tree.range(&5, &9);
    assert!(range.is_empty());
}

#[test]
fn adversarial_select_out_of_bounds() {
    let mut tree = BTree::new(2);
    tree.insert(1, 1);
    tree.insert(2, 2);
    assert_eq!(tree.select(0), Some(&1));
    assert_eq!(tree.select(1), Some(&2));
    assert_eq!(tree.select(2), None);
    assert_eq!(tree.select(usize::MAX), None);
}
