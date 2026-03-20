/// A node in the skip list that stores a key, value, and level.
#[derive(Debug, Clone)]
pub struct SkipNode<V: Clone> {
    key: u64,
    value: V,
    level: usize,
}

impl<V: Clone> SkipNode<V> {
    /// Create a new skip node.
    pub fn new(key: u64, value: V, level: usize) -> Self {
        Self { key, value, level }
    }

    /// Get the key.
    pub fn key(&self) -> u64 {
        self.key
    }

    /// Get a reference to the value.
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Set the value.
    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }

    /// Get the level of this node.
    pub fn level(&self) -> usize {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = SkipNode::new(42, "hello", 3);
        assert_eq!(node.key(), 42);
        assert_eq!(*node.value(), "hello");
        assert_eq!(node.level(), 3);
    }

    #[test]
    fn test_node_set_value() {
        let mut node = SkipNode::new(1, 100u32, 1);
        assert_eq!(*node.value(), 100);
        node.set_value(200);
        assert_eq!(*node.value(), 200);
    }

    #[test]
    fn test_node_clone() {
        let node = SkipNode::new(10, "world", 2);
        let cloned = node.clone();
        assert_eq!(cloned.key(), 10);
        assert_eq!(*cloned.value(), "world");
        assert_eq!(cloned.level(), 2);
    }

    #[test]
    fn test_node_zero_level() {
        let node = SkipNode::new(0, 0i32, 0);
        assert_eq!(node.key(), 0);
        assert_eq!(node.level(), 0);
    }

    #[test]
    fn test_node_max_key() {
        let node = SkipNode::new(u64::MAX, "max", 1);
        assert_eq!(node.key(), u64::MAX);
    }

    #[test]
    fn test_node_complex_value() {
        let node = SkipNode::new(5, vec![1, 2, 3], 2);
        assert_eq!(node.value().len(), 3);
        assert_eq!(node.value()[0], 1);
    }

    #[test]
    fn test_node_debug() {
        let node = SkipNode::new(1, 42u32, 1);
        let debug = format!("{:?}", node);
        assert!(debug.contains("SkipNode"));
    }

    #[test]
    fn test_node_string_value() {
        let node = SkipNode::new(99, String::from("aurora"), 4);
        assert_eq!(node.value(), "aurora");
        assert_eq!(node.key(), 99);
    }
}
