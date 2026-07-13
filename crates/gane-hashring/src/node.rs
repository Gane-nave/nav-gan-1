//! Virtual node representation for consistent hashing.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A virtual node on the hash ring.
#[derive(Debug, Clone)]
pub struct VirtualNode {
    /// The physical node this virtual node maps to.
    physical: String,
    /// The replica index (0..replication_factor).
    replica: u32,
    /// The position on the ring (hash value).
    position: u64,
}

impl VirtualNode {
    /// Create a new virtual node.
    pub fn new(physical: &str, replica: u32) -> Self {
        let position = Self::compute_position(physical, replica);
        Self {
            physical: physical.to_string(),
            replica,
            position,
        }
    }

    /// Compute the position on the ring for a physical node and replica index.
    fn compute_position(physical: &str, replica: u32) -> u64 {
        let mut hasher = DefaultHasher::new();
        physical.hash(&mut hasher);
        replica.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the physical node name.
    pub fn physical(&self) -> &str {
        &self.physical
    }

    /// Get the replica index.
    pub fn replica(&self) -> u32 {
        self.replica
    }

    /// Get the position on the ring.
    pub fn position(&self) -> u64 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_node_creation() {
        let vn = VirtualNode::new("server-1", 0);
        assert_eq!(vn.physical(), "server-1");
        assert_eq!(vn.replica(), 0);
        assert!(vn.position() > 0 || vn.position() == 0); // valid u64
    }

    #[test]
    fn test_different_replicas_different_positions() {
        let vn0 = VirtualNode::new("server-1", 0);
        let vn1 = VirtualNode::new("server-1", 1);
        assert_ne!(vn0.position(), vn1.position());
    }

    #[test]
    fn test_different_nodes_different_positions() {
        let a = VirtualNode::new("server-a", 0);
        let b = VirtualNode::new("server-b", 0);
        assert_ne!(a.position(), b.position());
    }

    #[test]
    fn test_same_node_same_replica_same_position() {
        let a = VirtualNode::new("node-x", 5);
        let b = VirtualNode::new("node-x", 5);
        assert_eq!(a.position(), b.position());
    }

    #[test]
    fn test_clone() {
        let vn = VirtualNode::new("node-1", 3);
        let cloned = vn.clone();
        assert_eq!(vn.physical(), cloned.physical());
        assert_eq!(vn.replica(), cloned.replica());
        assert_eq!(vn.position(), cloned.position());
    }

    #[test]
    fn test_debug_format() {
        let vn = VirtualNode::new("s1", 0);
        let debug = format!("{:?}", vn);
        assert!(debug.contains("s1"));
    }
}
