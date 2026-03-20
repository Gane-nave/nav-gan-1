//! Merkle tree implementation for data integrity verification.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A Merkle tree node.
#[derive(Debug, Clone)]
struct MerkleNode {
    /// Hash of this node.
    hash: u64,
    /// Left child index (None for leaves).
    left: Option<usize>,
    /// Right child index (None for leaves).
    right: Option<usize>,
}

/// A Merkle tree for data integrity verification.
///
/// Leaf nodes hash individual data items. Internal nodes hash the
/// concatenation of their children's hashes. The root hash summarizes
/// the entire dataset.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// All nodes in the tree.
    nodes: Vec<MerkleNode>,
    /// Number of leaf nodes.
    leaf_count: usize,
    /// The root node index.
    root: Option<usize>,
}

impl MerkleTree {
    /// Build a Merkle tree from a list of data items.
    pub fn from_items<T: Hash>(items: &[T]) -> Self {
        if items.is_empty() {
            return Self {
                nodes: Vec::new(),
                leaf_count: 0,
                root: None,
            };
        }

        let mut tree = Self {
            nodes: Vec::new(),
            leaf_count: items.len(),
            root: None,
        };

        // Create leaf nodes
        let mut level: Vec<usize> = items
            .iter()
            .map(|item| {
                let hash = Self::hash_item(item);
                let idx = tree.nodes.len();
                tree.nodes.push(MerkleNode {
                    hash,
                    left: None,
                    right: None,
                });
                idx
            })
            .collect();

        // Build tree bottom-up
        while level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;
            while i < level.len() {
                let left = level[i];
                if i + 1 < level.len() {
                    let right = level[i + 1];
                    let hash = Self::combine_hashes(
                        tree.nodes[left].hash,
                        tree.nodes[right].hash,
                    );
                    let idx = tree.nodes.len();
                    tree.nodes.push(MerkleNode {
                        hash,
                        left: Some(left),
                        right: Some(right),
                    });
                    next_level.push(idx);
                    i += 2;
                } else {
                    // Odd node — promote directly
                    let hash = Self::combine_hashes(
                        tree.nodes[left].hash,
                        tree.nodes[left].hash,
                    );
                    let idx = tree.nodes.len();
                    tree.nodes.push(MerkleNode {
                        hash,
                        left: Some(left),
                        right: None,
                    });
                    next_level.push(idx);
                    i += 1;
                }
            }
            level = next_level;
        }

        tree.root = Some(level[0]);
        tree
    }

    /// Get the root hash. Returns `None` if the tree is empty.
    pub fn root_hash(&self) -> Option<u64> {
        self.root.map(|idx| self.nodes[idx].hash)
    }

    /// Number of leaf nodes.
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }

    /// Total number of nodes (leaves + internal).
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Verify that the tree's root hash matches a given hash.
    pub fn verify(&self, expected_hash: u64) -> bool {
        self.root_hash() == Some(expected_hash)
    }

    /// Get the depth of the tree.
    pub fn depth(&self) -> usize {
        if self.leaf_count == 0 {
            return 0;
        }
        let mut d = 0;
        let mut n = self.leaf_count;
        while n > 1 {
            n = n.div_ceil(2);
            d += 1;
        }
        d
    }

    /// Get the proof path for a leaf at the given index.
    /// Returns a list of (hash, is_right) pairs from leaf to root.
    pub fn proof(&self, leaf_idx: usize) -> Vec<(u64, bool)> {
        if leaf_idx >= self.leaf_count {
            return Vec::new();
        }
        let mut path = Vec::new();
        self.collect_proof(self.root, leaf_idx, 0, self.leaf_count, &mut path);
        path
    }

    fn collect_proof(
        &self,
        node: Option<usize>,
        target: usize,
        range_start: usize,
        range_end: usize,
        path: &mut Vec<(u64, bool)>,
    ) -> bool {
        let node_idx = match node {
            Some(idx) => idx,
            None => return false,
        };
        let node = &self.nodes[node_idx];

        // Leaf node
        if node.left.is_none() && node.right.is_none() {
            return range_start == target;
        }

        let mid = range_start + (range_end - range_start).div_ceil(2);

        if target < mid {
            // Target is in left subtree
            if self.collect_proof(node.left, target, range_start, mid - 1, path) {
                if let Some(right) = node.right {
                    path.push((self.nodes[right].hash, true));
                }
                return true;
            }
        } else {
            // Target is in right subtree
            if self.collect_proof(node.right, target, mid, range_end, path) {
                if let Some(left) = node.left {
                    path.push((self.nodes[left].hash, false));
                }
                return true;
            }
        }
        false
    }

    /// Verify a proof against the root hash.
    pub fn verify_proof(leaf_hash: u64, proof: &[(u64, bool)], root_hash: u64) -> bool {
        let mut hash = leaf_hash;
        for &(sibling_hash, is_right) in proof {
            hash = if is_right {
                Self::combine_hashes(hash, sibling_hash)
            } else {
                Self::combine_hashes(sibling_hash, hash)
            };
        }
        hash == root_hash
    }

    /// Get the hash of a leaf at the given index.
    pub fn leaf_hash(&self, idx: usize) -> Option<u64> {
        if idx < self.leaf_count {
            Some(self.nodes[idx].hash)
        } else {
            None
        }
    }

    fn hash_item<T: Hash>(item: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    fn combine_hashes(left: u64, right: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        left.hash(&mut hasher);
        right.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::from_items::<u32>(&[]);
        assert!(tree.is_empty());
        assert_eq!(tree.root_hash(), None);
        assert_eq!(tree.leaf_count(), 0);
        assert_eq!(tree.depth(), 0);
    }

    #[test]
    fn test_single_item() {
        let tree = MerkleTree::from_items(&[42u32]);
        assert!(!tree.is_empty());
        assert!(tree.root_hash().is_some());
        assert_eq!(tree.leaf_count(), 1);
    }

    #[test]
    fn test_two_items() {
        let tree = MerkleTree::from_items(&[1u32, 2]);
        assert_eq!(tree.leaf_count(), 2);
        assert_eq!(tree.depth(), 1);
        assert!(tree.root_hash().is_some());
    }

    #[test]
    fn test_four_items() {
        let tree = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        assert_eq!(tree.leaf_count(), 4);
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn test_odd_items() {
        let tree = MerkleTree::from_items(&[1u32, 2, 3]);
        assert_eq!(tree.leaf_count(), 3);
        assert!(tree.root_hash().is_some());
    }

    #[test]
    fn test_same_data_same_hash() {
        let tree1 = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        let tree2 = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        assert_eq!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn test_different_data_different_hash() {
        let tree1 = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        let tree2 = MerkleTree::from_items(&[1u32, 2, 3, 5]);
        assert_ne!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn test_verify() {
        let tree = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        let root = tree.root_hash().unwrap();
        assert!(tree.verify(root));
        assert!(!tree.verify(root.wrapping_add(1)));
    }

    #[test]
    fn test_node_count() {
        let tree = MerkleTree::from_items(&[1u32, 2, 3, 4]);
        // 4 leaves + 2 internal + 1 root = 7
        assert_eq!(tree.node_count(), 7);
    }

    #[test]
    fn test_leaf_hash() {
        let tree = MerkleTree::from_items(&[42u32]);
        assert!(tree.leaf_hash(0).is_some());
        assert!(tree.leaf_hash(1).is_none());
    }

    #[test]
    fn test_string_items() {
        let tree = MerkleTree::from_items(&["hello", "world", "foo"]);
        assert_eq!(tree.leaf_count(), 3);
        assert!(tree.root_hash().is_some());
    }

    #[test]
    fn test_proof_empty() {
        let tree = MerkleTree::from_items::<u32>(&[]);
        assert!(tree.proof(0).is_empty());
    }

    #[test]
    fn test_proof_out_of_range() {
        let tree = MerkleTree::from_items(&[1u32, 2]);
        assert!(tree.proof(5).is_empty());
    }

    #[test]
    fn test_depth_power_of_two() {
        let tree = MerkleTree::from_items(&[1u32, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(tree.depth(), 3);
    }

    #[test]
    fn test_large_tree() {
        let items: Vec<u32> = (0..1000).collect();
        let tree = MerkleTree::from_items(&items);
        assert_eq!(tree.leaf_count(), 1000);
        assert!(tree.root_hash().is_some());
    }
}
