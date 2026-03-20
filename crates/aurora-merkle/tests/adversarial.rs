//! Adversarial tests for aurora-merkle.

use aurora_merkle::MerkleTree;

#[test]
fn adversarial_large_tree() {
    let items: Vec<u64> = (0..10_000).collect();
    let tree = MerkleTree::from_items(&items);
    assert_eq!(tree.leaf_count(), 10_000);
    assert!(tree.root_hash().is_some());
}

#[test]
fn adversarial_single_bit_change() {
    let items1: Vec<u32> = (0..100).collect();
    let mut items2 = items1.clone();
    items2[50] = 999; // change one element
    let tree1 = MerkleTree::from_items(&items1);
    let tree2 = MerkleTree::from_items(&items2);
    assert_ne!(tree1.root_hash(), tree2.root_hash());
}

#[test]
fn adversarial_order_matters() {
    let tree1 = MerkleTree::from_items(&[1u32, 2, 3]);
    let tree2 = MerkleTree::from_items(&[3u32, 2, 1]);
    assert_ne!(tree1.root_hash(), tree2.root_hash());
}

#[test]
fn adversarial_duplicate_items() {
    let tree = MerkleTree::from_items(&[42u32, 42, 42, 42]);
    assert_eq!(tree.leaf_count(), 4);
    // All leaves have same hash
    let h0 = tree.leaf_hash(0).unwrap();
    let h1 = tree.leaf_hash(1).unwrap();
    assert_eq!(h0, h1);
}

#[test]
fn adversarial_verify_correct() {
    let tree = MerkleTree::from_items(&[1u32, 2, 3, 4, 5]);
    let root = tree.root_hash().unwrap();
    assert!(tree.verify(root));
}

#[test]
fn adversarial_verify_tampered() {
    let tree = MerkleTree::from_items(&[1u32, 2, 3, 4, 5]);
    let root = tree.root_hash().unwrap();
    assert!(!tree.verify(root.wrapping_add(1)));
    assert!(!tree.verify(0));
}

#[test]
fn adversarial_empty_tree() {
    let tree = MerkleTree::from_items::<u32>(&[]);
    assert!(tree.is_empty());
    assert_eq!(tree.root_hash(), None);
    assert_eq!(tree.depth(), 0);
    assert_eq!(tree.node_count(), 0);
}

#[test]
fn adversarial_power_of_two_sizes() {
    for size in [1, 2, 4, 8, 16, 32, 64, 128] {
        let items: Vec<u32> = (0..size).collect();
        let tree = MerkleTree::from_items(&items);
        assert_eq!(tree.leaf_count(), size as usize);
        assert!(tree.root_hash().is_some());
    }
}
