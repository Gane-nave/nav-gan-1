//! Consistent hashing ring implementation.

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::node::VirtualNode;

/// A consistent hashing ring for distributed load balancing.
#[derive(Debug)]
pub struct HashRing {
    /// Sorted list of virtual nodes by position.
    nodes: Vec<VirtualNode>,
    /// Set of physical node names.
    physical_nodes: HashSet<String>,
    /// Number of virtual nodes per physical node.
    replication_factor: u32,
    /// Total lookups performed.
    lookups: u64,
}

impl HashRing {
    /// Create a new hash ring with the given replication factor.
    pub fn new(replication_factor: u32) -> Self {
        let rf = if replication_factor == 0 { 1 } else { replication_factor };
        Self {
            nodes: Vec::new(),
            physical_nodes: HashSet::new(),
            replication_factor: rf,
            lookups: 0,
        }
    }

    /// Add a physical node to the ring.
    /// Returns true if the node was added, false if it already exists.
    pub fn add_node(&mut self, name: &str) -> bool {
        if !self.physical_nodes.insert(name.to_string()) {
            return false;
        }
        for i in 0..self.replication_factor {
            let vn = VirtualNode::new(name, i);
            let pos = vn.position();
            let idx = self.nodes.partition_point(|n| n.position() < pos);
            self.nodes.insert(idx, vn);
        }
        true
    }

    /// Remove a physical node from the ring.
    /// Returns true if the node was removed.
    pub fn remove_node(&mut self, name: &str) -> bool {
        if !self.physical_nodes.remove(name) {
            return false;
        }
        self.nodes.retain(|n| n.physical() != name);
        true
    }

    /// Look up which physical node a key maps to.
    /// Returns None if the ring is empty.
    pub fn lookup(&mut self, key: &str) -> Option<&str> {
        if self.nodes.is_empty() {
            return None;
        }
        self.lookups = self.lookups.saturating_add(1);
        let hash = Self::hash_key(key);
        let idx = self.nodes.partition_point(|n| n.position() < hash);
        let idx = if idx >= self.nodes.len() { 0 } else { idx };
        Some(self.nodes[idx].physical())
    }

    /// Look up N distinct physical nodes for a key (for replication).
    /// Returns up to `count` distinct physical nodes, starting from the
    /// key's primary node and walking clockwise.
    pub fn lookup_n(&mut self, key: &str, count: usize) -> Vec<&str> {
        if self.nodes.is_empty() || count == 0 {
            return Vec::new();
        }
        self.lookups = self.lookups.saturating_add(1);
        let hash = Self::hash_key(key);
        let start = self.nodes.partition_point(|n| n.position() < hash);
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        let len = self.nodes.len();
        for i in 0..len {
            let idx = (start + i) % len;
            let phys = self.nodes[idx].physical();
            if seen.insert(phys) {
                result.push(phys);
                if result.len() >= count {
                    break;
                }
            }
        }
        result
    }

    /// Get the number of physical nodes.
    pub fn node_count(&self) -> usize {
        self.physical_nodes.len()
    }

    /// Get the number of virtual nodes.
    pub fn virtual_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the replication factor.
    pub fn replication_factor(&self) -> u32 {
        self.replication_factor
    }

    /// Get the total number of lookups performed.
    pub fn lookups(&self) -> u64 {
        self.lookups
    }

    /// Get all physical node names.
    pub fn physical_nodes(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.physical_nodes.iter().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    /// Get the distribution of keys across nodes for analysis.
    /// Returns a map of physical node name to count of virtual nodes.
    pub fn distribution(&self) -> HashMap<&str, usize> {
        let mut dist = HashMap::new();
        for vn in &self.nodes {
            *dist.entry(vn.physical()).or_insert(0) += 1;
        }
        dist
    }

    /// Check if the ring contains a physical node.
    pub fn contains(&self, name: &str) -> bool {
        self.physical_nodes.contains(name)
    }

    /// Check if the ring is empty.
    pub fn is_empty(&self) -> bool {
        self.physical_nodes.is_empty()
    }

    /// Hash a key to a u64 position.
    fn hash_key(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ring() {
        let ring = HashRing::new(3);
        assert_eq!(ring.replication_factor(), 3);
        assert_eq!(ring.node_count(), 0);
        assert!(ring.is_empty());
    }

    #[test]
    fn test_zero_replication_defaults_to_one() {
        let ring = HashRing::new(0);
        assert_eq!(ring.replication_factor(), 1);
    }

    #[test]
    fn test_add_node() {
        let mut ring = HashRing::new(3);
        assert!(ring.add_node("server-1"));
        assert_eq!(ring.node_count(), 1);
        assert_eq!(ring.virtual_node_count(), 3);
        assert!(!ring.is_empty());
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut ring = HashRing::new(3);
        assert!(ring.add_node("server-1"));
        assert!(!ring.add_node("server-1"));
        assert_eq!(ring.node_count(), 1);
    }

    #[test]
    fn test_remove_node() {
        let mut ring = HashRing::new(3);
        ring.add_node("server-1");
        ring.add_node("server-2");
        assert!(ring.remove_node("server-1"));
        assert_eq!(ring.node_count(), 1);
        assert_eq!(ring.virtual_node_count(), 3);
        assert!(!ring.contains("server-1"));
        assert!(ring.contains("server-2"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut ring = HashRing::new(3);
        assert!(!ring.remove_node("ghost"));
    }

    #[test]
    fn test_lookup_empty_ring() {
        let mut ring = HashRing::new(3);
        assert!(ring.lookup("key1").is_none());
    }

    #[test]
    fn test_lookup_single_node() {
        let mut ring = HashRing::new(3);
        ring.add_node("server-1");
        let result = ring.lookup("any-key");
        assert_eq!(result, Some("server-1"));
    }

    #[test]
    fn test_lookup_consistency() {
        let mut ring = HashRing::new(10);
        ring.add_node("a");
        ring.add_node("b");
        ring.add_node("c");
        let first = ring.lookup("test-key").unwrap().to_string();
        let second = ring.lookup("test-key").unwrap().to_string();
        assert_eq!(first, second);
    }

    #[test]
    fn test_lookup_n() {
        let mut ring = HashRing::new(10);
        ring.add_node("a");
        ring.add_node("b");
        ring.add_node("c");
        let nodes = ring.lookup_n("key1", 2);
        assert_eq!(nodes.len(), 2);
        assert_ne!(nodes[0], nodes[1]);
    }

    #[test]
    fn test_lookup_n_more_than_available() {
        let mut ring = HashRing::new(5);
        ring.add_node("a");
        ring.add_node("b");
        let nodes = ring.lookup_n("key1", 10);
        assert_eq!(nodes.len(), 2); // only 2 physical nodes
    }

    #[test]
    fn test_distribution() {
        let mut ring = HashRing::new(5);
        ring.add_node("a");
        ring.add_node("b");
        let dist = ring.distribution();
        assert_eq!(dist[&"a"], 5);
        assert_eq!(dist[&"b"], 5);
    }

    #[test]
    fn test_physical_nodes_sorted() {
        let mut ring = HashRing::new(3);
        ring.add_node("charlie");
        ring.add_node("alpha");
        ring.add_node("bravo");
        let names = ring.physical_nodes();
        assert_eq!(names, vec!["alpha", "bravo", "charlie"]);
    }

    #[test]
    fn test_lookup_counter() {
        let mut ring = HashRing::new(3);
        ring.add_node("s1");
        assert_eq!(ring.lookups(), 0);
        ring.lookup("k1");
        ring.lookup("k2");
        assert_eq!(ring.lookups(), 2);
    }

    #[test]
    fn test_minimal_movement_on_add() {
        let mut ring = HashRing::new(50);
        ring.add_node("a");
        ring.add_node("b");
        // Record assignments for 100 keys
        let mut before = Vec::new();
        for i in 0..100 {
            let key = format!("key-{}", i);
            before.push(ring.lookup(&key).unwrap().to_string());
        }
        // Add a third node
        ring.add_node("c");
        let mut changed = 0;
        for (i, prev) in before.iter().enumerate() {
            let key = format!("key-{}", i);
            let after = ring.lookup(&key).unwrap();
            if after != prev {
                changed += 1;
            }
        }
        // With consistent hashing, roughly 1/3 of keys should move
        // Allow generous bounds
        assert!(changed < 70, "Too many keys moved: {}", changed);
    }

    #[test]
    fn test_contains() {
        let mut ring = HashRing::new(3);
        ring.add_node("exists");
        assert!(ring.contains("exists"));
        assert!(!ring.contains("nope"));
    }
}
