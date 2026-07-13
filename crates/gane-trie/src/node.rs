//! Trie node — represents a single character in the prefix trie.

use std::collections::HashMap;

/// A node in the prefix trie.
#[derive(Debug, Clone)]
pub struct TrieNode {
    /// Children keyed by character.
    children: HashMap<char, TrieNode>,
    /// Whether this node marks the end of a complete key.
    is_terminal: bool,
    /// Value stored at this node (if terminal).
    value: Option<String>,
    /// Depth of this node in the trie.
    depth: usize,
}

impl TrieNode {
    /// Create a new empty trie node.
    pub fn new(depth: usize) -> Self {
        Self {
            children: HashMap::new(),
            is_terminal: false,
            value: None,
            depth,
        }
    }

    /// Check if this node is a terminal (end of a key).
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    /// Set this node as terminal with a value.
    pub fn set_terminal(&mut self, value: String) {
        self.is_terminal = true;
        self.value = Some(value);
    }

    /// Clear terminal status.
    pub fn clear_terminal(&mut self) {
        self.is_terminal = false;
        self.value = None;
    }

    /// Get the stored value.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Get the depth of this node.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Get a child node by character.
    pub fn child(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }

    /// Get a mutable child node by character.
    pub fn child_mut(&mut self, c: char) -> Option<&mut TrieNode> {
        self.children.get_mut(&c)
    }

    /// Insert or get a child node for a character.
    pub fn get_or_insert_child(&mut self, c: char) -> &mut TrieNode {
        self.children
            .entry(c)
            .or_insert_with(|| TrieNode::new(self.depth + 1))
    }

    /// Remove a child node.
    pub fn remove_child(&mut self, c: char) -> Option<TrieNode> {
        self.children.remove(&c)
    }

    /// Number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Check if this node has any children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Get all child characters.
    pub fn child_chars(&self) -> Vec<char> {
        let mut chars: Vec<char> = self.children.keys().copied().collect();
        chars.sort();
        chars
    }

    /// Get children reference.
    pub fn children(&self) -> &HashMap<char, TrieNode> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = TrieNode::new(0);
        assert_eq!(node.depth(), 0);
        assert!(!node.is_terminal());
        assert!(node.value().is_none());
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn test_node_terminal() {
        let mut node = TrieNode::new(1);
        node.set_terminal("hello".to_string());
        assert!(node.is_terminal());
        assert_eq!(node.value(), Some("hello"));
        node.clear_terminal();
        assert!(!node.is_terminal());
        assert!(node.value().is_none());
    }

    #[test]
    fn test_node_children() {
        let mut node = TrieNode::new(0);
        node.get_or_insert_child('a');
        node.get_or_insert_child('b');
        assert_eq!(node.child_count(), 2);
        assert!(node.has_children());
        assert!(node.child('a').is_some());
        assert!(node.child('c').is_none());
    }

    #[test]
    fn test_node_child_depth() {
        let mut node = TrieNode::new(0);
        let child = node.get_or_insert_child('x');
        assert_eq!(child.depth(), 1);
    }

    #[test]
    fn test_node_remove_child() {
        let mut node = TrieNode::new(0);
        node.get_or_insert_child('a');
        assert_eq!(node.child_count(), 1);
        let removed = node.remove_child('a');
        assert!(removed.is_some());
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn test_node_child_chars_sorted() {
        let mut node = TrieNode::new(0);
        node.get_or_insert_child('c');
        node.get_or_insert_child('a');
        node.get_or_insert_child('b');
        assert_eq!(node.child_chars(), vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_node_child_mut() {
        let mut node = TrieNode::new(0);
        node.get_or_insert_child('a');
        let child = node.child_mut('a').unwrap();
        child.set_terminal("val".to_string());
        assert_eq!(node.child('a').unwrap().value(), Some("val"));
    }

    #[test]
    fn test_node_clone() {
        let mut node = TrieNode::new(0);
        node.set_terminal("test".to_string());
        node.get_or_insert_child('x');
        let cloned = node.clone();
        assert_eq!(cloned.value(), Some("test"));
        assert_eq!(cloned.child_count(), 1);
    }
}
