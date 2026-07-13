//! B-Tree with order-statistic operations.

use std::fmt;

/// Statistics about the B-Tree.
#[derive(Debug, Clone, Copy)]
pub struct BTreeStats {
    pub size: usize,
    pub height: usize,
    pub node_count: usize,
}

/// A node in the B-Tree.
#[derive(Debug, Clone)]
struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<usize>,
    subtree_sizes: Vec<usize>,
    count: usize,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    fn new_leaf() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            subtree_sizes: Vec::new(),
            count: 0,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// B-Tree with order-statistic operations.
///
/// Supports insert, get, rank (position of key),
/// and select (key at position).
pub struct BTree<K, V> {
    nodes: Vec<Node<K, V>>,
    root: Option<usize>,
    order: usize,
    len: usize,
}

impl<K: Ord + Clone + fmt::Debug, V: Clone> BTree<K, V> {
    /// Create a new B-Tree with the given order (minimum degree).
    /// Order must be >= 2.
    pub fn new(order: usize) -> Self {
        let order = order.max(2);
        Self {
            nodes: Vec::new(),
            root: None,
            order,
            len: 0,
        }
    }

    /// Number of key-value pairs.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get statistics about the tree.
    pub fn stats(&self) -> BTreeStats {
        let height = if let Some(root) = self.root {
            self.height(root)
        } else {
            0
        };
        BTreeStats {
            size: self.len,
            height,
            node_count: self.nodes.len(),
        }
    }

    fn height(&self, node_idx: usize) -> usize {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            1
        } else {
            1 + self.height(node.children[0])
        }
    }

    fn alloc_node(&mut self, node: Node<K, V>) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    /// Insert a key-value pair. Returns the old value if the key existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(root) = self.root {
            if let Some(old) = self.update(root, &key, value.clone()) {
                return Some(old);
            }
            let max_keys = 2 * self.order - 1;
            if self.nodes[root].keys.len() == max_keys {
                let new_root = self.alloc_node(Node::new_leaf());
                self.nodes[new_root].children.push(root);
                let root_count = self.nodes[root].count;
                self.nodes[new_root].subtree_sizes.push(root_count);
                self.split_child(new_root, 0);
                self.root = Some(new_root);
                self.insert_non_full(new_root, key, value);
            } else {
                self.insert_non_full(root, key, value);
            }
        } else {
            let mut leaf = Node::new_leaf();
            leaf.keys.push(key);
            leaf.values.push(value);
            leaf.count = 1;
            let idx = self.alloc_node(leaf);
            self.root = Some(idx);
        }
        self.len = self.len.saturating_add(1);
        None
    }

    fn update(&mut self, node_idx: usize, key: &K, value: V) -> Option<V> {
        let node = &self.nodes[node_idx];
        match node.keys.binary_search(key) {
            Ok(i) => {
                let old = self.nodes[node_idx].values[i].clone();
                self.nodes[node_idx].values[i] = value;
                Some(old)
            }
            Err(i) => {
                if node.is_leaf() {
                    None
                } else {
                    let child = self.nodes[node_idx].children[i];
                    self.update(child, key, value)
                }
            }
        }
    }

    fn split_child(&mut self, parent_idx: usize, child_pos: usize) {
        let child_idx = self.nodes[parent_idx].children[child_pos];
        let mid = self.order - 1;

        let mut right = Node::new_leaf();
        let mid_key = self.nodes[child_idx].keys[mid].clone();
        let mid_val = self.nodes[child_idx].values[mid].clone();

        right.keys = self.nodes[child_idx].keys.split_off(mid + 1);
        self.nodes[child_idx].keys.pop();
        right.values = self.nodes[child_idx].values.split_off(mid + 1);
        self.nodes[child_idx].values.pop();

        if !self.nodes[child_idx].children.is_empty() {
            right.children = self.nodes[child_idx].children.split_off(mid + 1);
            right.subtree_sizes = self.nodes[child_idx].subtree_sizes.split_off(mid + 1);
        }

        right.count = right.keys.len();
        for &sz in &right.subtree_sizes {
            right.count = right.count.saturating_add(sz);
        }
        let left_count = {
            let left = &self.nodes[child_idx];
            let mut c = left.keys.len();
            for &sz in &left.subtree_sizes {
                c = c.saturating_add(sz);
            }
            c
        };
        self.nodes[child_idx].count = left_count;

        let right_idx = self.alloc_node(right);

        self.nodes[parent_idx].keys.insert(child_pos, mid_key);
        self.nodes[parent_idx].values.insert(child_pos, mid_val);
        self.nodes[parent_idx]
            .children
            .insert(child_pos + 1, right_idx);
        let right_count = self.nodes[right_idx].count;
        self.nodes[parent_idx]
            .subtree_sizes
            .insert(child_pos + 1, right_count);
        self.nodes[parent_idx].subtree_sizes[child_pos] = left_count;

        let parent = &self.nodes[parent_idx];
        let mut pc = parent.keys.len();
        for &sz in &parent.subtree_sizes {
            pc = pc.saturating_add(sz);
        }
        self.nodes[parent_idx].count = pc;
    }

    fn insert_non_full(&mut self, node_idx: usize, key: K, value: V) {
        let is_leaf = self.nodes[node_idx].is_leaf();
        if is_leaf {
            let pos = self.nodes[node_idx]
                .keys
                .binary_search(&key)
                .unwrap_or_else(|i| i);
            self.nodes[node_idx].keys.insert(pos, key);
            self.nodes[node_idx].values.insert(pos, value);
            self.nodes[node_idx].count = self.nodes[node_idx].count.saturating_add(1);
        } else {
            let mut pos = self.nodes[node_idx]
                .keys
                .binary_search(&key)
                .unwrap_or_else(|i| i);
            let child = self.nodes[node_idx].children[pos];
            let max_keys = 2 * self.order - 1;
            if self.nodes[child].keys.len() == max_keys {
                self.split_child(node_idx, pos);
                if key > self.nodes[node_idx].keys[pos] {
                    pos += 1;
                }
            }
            let child = self.nodes[node_idx].children[pos];
            self.insert_non_full(child, key, value);
            self.nodes[node_idx].subtree_sizes[pos] = self.nodes[child].count;
            let node = &self.nodes[node_idx];
            let mut c = node.keys.len();
            for &sz in &node.subtree_sizes {
                c = c.saturating_add(sz);
            }
            self.nodes[node_idx].count = c;
        }
    }

    /// Get a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let root = self.root?;
        self.search(root, key)
    }

    fn search(&self, node_idx: usize, key: &K) -> Option<&V> {
        let node = &self.nodes[node_idx];
        match node.keys.binary_search(key) {
            Ok(i) => Some(&node.values[i]),
            Err(i) => {
                if node.is_leaf() {
                    None
                } else {
                    self.search(node.children[i], key)
                }
            }
        }
    }

    /// Get the rank (0-based position) of a key in sorted order.
    pub fn rank(&self, key: &K) -> Option<usize> {
        let root = self.root?;
        self.rank_recursive(root, key)
    }

    fn rank_recursive(&self, node_idx: usize, key: &K) -> Option<usize> {
        let node = &self.nodes[node_idx];
        match node.keys.binary_search(key) {
            Ok(i) => {
                let mut r = i;
                if !node.is_leaf() {
                    for j in 0..=i {
                        r = r.saturating_add(node.subtree_sizes[j]);
                    }
                }
                Some(r)
            }
            Err(i) => {
                if node.is_leaf() {
                    None
                } else {
                    let child = node.children[i];
                    let sub_rank = self.rank_recursive(child, key)?;
                    let mut r = sub_rank + i;
                    for j in 0..i {
                        r = r.saturating_add(node.subtree_sizes[j]);
                    }
                    Some(r)
                }
            }
        }
    }

    /// Select the key at the given 0-based position in sorted order.
    pub fn select(&self, position: usize) -> Option<&K> {
        if position >= self.len {
            return None;
        }
        let root = self.root?;
        self.select_recursive(root, position)
    }

    fn select_recursive(&self, node_idx: usize, mut position: usize) -> Option<&K> {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            if position < node.keys.len() {
                Some(&node.keys[position])
            } else {
                None
            }
        } else {
            for i in 0..node.keys.len() {
                let child_size = node.subtree_sizes[i];
                if position < child_size {
                    return self.select_recursive(node.children[i], position);
                }
                position -= child_size;
                if position == 0 {
                    return Some(&node.keys[i]);
                }
                position -= 1;
            }
            let last = *node.children.last()?;
            self.select_recursive(last, position)
        }
    }

    /// Check if the tree contains the given key.
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Return all keys in sorted order.
    pub fn keys_sorted(&self) -> Vec<K> {
        let mut result = Vec::with_capacity(self.len);
        if let Some(root) = self.root {
            self.collect_keys(root, &mut result);
        }
        result
    }

    fn collect_keys(&self, node_idx: usize, result: &mut Vec<K>) {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            result.extend(node.keys.iter().cloned());
        } else {
            for i in 0..node.keys.len() {
                self.collect_keys(node.children[i], result);
                result.push(node.keys[i].clone());
            }
            if let Some(&last) = node.children.last() {
                self.collect_keys(last, result);
            }
        }
    }

    /// Get the minimum key.
    pub fn min_key(&self) -> Option<&K> {
        let root = self.root?;
        self.min_in_subtree(root)
    }

    fn min_in_subtree(&self, node_idx: usize) -> Option<&K> {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            node.keys.first()
        } else {
            self.min_in_subtree(node.children[0])
        }
    }

    /// Get the maximum key.
    pub fn max_key(&self) -> Option<&K> {
        let root = self.root?;
        self.max_in_subtree(root)
    }

    fn max_in_subtree(&self, node_idx: usize) -> Option<&K> {
        let node = &self.nodes[node_idx];
        if node.is_leaf() {
            node.keys.last()
        } else {
            let last = *node.children.last()?;
            self.max_in_subtree(last)
        }
    }

    /// Range query: get all key-value pairs where low <= key <= high.
    pub fn range(&self, low: &K, high: &K) -> Vec<(K, V)> {
        let mut result = Vec::new();
        if let Some(root) = self.root {
            self.range_collect(root, low, high, &mut result);
        }
        result
    }

    #[allow(clippy::manual_range_contains)]
    fn range_collect(&self, node_idx: usize, low: &K, high: &K, result: &mut Vec<(K, V)>) {
        let node = &self.nodes[node_idx];
        for i in 0..node.keys.len() {
            if !node.is_leaf() && node.keys[i] >= *low {
                self.range_collect(node.children[i], low, high, result);
            }
            if node.keys[i] >= *low && node.keys[i] <= *high {
                result.push((node.keys[i].clone(), node.values[i].clone()));
            }
            if node.keys[i] > *high {
                return;
            }
        }
        if !node.is_leaf() {
            let last_child = *node.children.last().unwrap();
            if node.keys.last().is_none_or(|k| k <= high) {
                self.range_collect(last_child, low, high, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut tree = BTree::new(2);
        tree.insert(10, "ten");
        tree.insert(20, "twenty");
        tree.insert(5, "five");
        assert_eq!(tree.get(&10), Some(&"ten"));
        assert_eq!(tree.get(&20), Some(&"twenty"));
        assert_eq!(tree.get(&5), Some(&"five"));
        assert_eq!(tree.get(&15), None);
    }

    #[test]
    fn test_len_and_empty() {
        let mut tree: BTree<i32, i32> = BTree::new(2);
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        tree.insert(1, 100);
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_update_existing_key() {
        let mut tree = BTree::new(2);
        tree.insert(10, "old");
        let old = tree.insert(10, "new");
        assert_eq!(old, Some("old"));
        assert_eq!(tree.get(&10), Some(&"new"));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_rank() {
        let mut tree = BTree::new(2);
        for i in 0..10 {
            tree.insert(i * 2, i);
        }
        assert_eq!(tree.rank(&0), Some(0));
        assert_eq!(tree.rank(&4), Some(2));
        assert_eq!(tree.rank(&18), Some(9));
        assert_eq!(tree.rank(&3), None);
    }

    #[test]
    fn test_select() {
        let mut tree = BTree::new(2);
        for i in 0..10 {
            tree.insert(i * 2, i);
        }
        assert_eq!(tree.select(0), Some(&0));
        assert_eq!(tree.select(5), Some(&10));
        assert_eq!(tree.select(9), Some(&18));
        assert_eq!(tree.select(10), None);
    }

    #[test]
    fn test_min_max() {
        let mut tree = BTree::new(3);
        tree.insert(50, 0);
        tree.insert(10, 0);
        tree.insert(90, 0);
        tree.insert(30, 0);
        tree.insert(70, 0);
        assert_eq!(tree.min_key(), Some(&10));
        assert_eq!(tree.max_key(), Some(&90));
    }

    #[test]
    fn test_range_query() {
        let mut tree = BTree::new(2);
        for i in 0..20 {
            tree.insert(i, i * 10);
        }
        let range = tree.range(&5, &10);
        let keys: Vec<i32> = range.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_keys_sorted() {
        let mut tree = BTree::new(2);
        tree.insert(5, 0);
        tree.insert(1, 0);
        tree.insert(9, 0);
        tree.insert(3, 0);
        tree.insert(7, 0);
        assert_eq!(tree.keys_sorted(), vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_contains() {
        let mut tree = BTree::new(2);
        tree.insert(42, "answer");
        assert!(tree.contains(&42));
        assert!(!tree.contains(&41));
    }

    #[test]
    fn test_stats() {
        let mut tree = BTree::new(2);
        for i in 0..20 {
            tree.insert(i, i);
        }
        let stats = tree.stats();
        assert_eq!(stats.size, 20);
        assert!(stats.height >= 2);
        assert!(stats.node_count > 0);
    }
}
