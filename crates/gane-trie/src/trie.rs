//! Prefix trie — string key lookups, prefix matching, and autocomplete.

use crate::node::TrieNode;

/// A prefix trie for string key-value storage and retrieval.
pub struct PrefixTrie {
    /// Root node of the trie.
    root: TrieNode,
    /// Number of keys stored.
    count: usize,
    /// Total inserts (lifetime).
    total_inserts: u64,
    /// Total removes (lifetime).
    total_removes: u64,
    /// Total lookups (lifetime).
    total_lookups: u64,
    /// Total hits (lifetime).
    total_hits: u64,
}

impl PrefixTrie {
    /// Create a new empty prefix trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(0),
            count: 0,
            total_inserts: 0,
            total_removes: 0,
            total_lookups: 0,
            total_hits: 0,
        }
    }

    /// Insert a key-value pair. Returns the old value if the key already existed.
    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.total_inserts += 1;
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.get_or_insert_child(c);
        }
        let old = node.value().map(|v| v.to_string());
        node.set_terminal(value.to_string());
        if old.is_none() {
            self.count += 1;
        }
        old
    }

    /// Get the value for a key.
    pub fn get(&mut self, key: &str) -> Option<&str> {
        self.total_lookups += 1;
        let mut node = &self.root;
        for c in key.chars() {
            {
                let child = node.child(c)?;
                node = child
            }
        }
        if node.is_terminal() {
            self.total_hits += 1;
            node.value()
        } else {
            None
        }
    }

    /// Check if a key exists.
    pub fn contains(&mut self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Remove a key. Returns the removed value.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let removed = Self::remove_recursive(&mut self.root, key, 0);
        if removed.is_some() {
            self.count -= 1;
            self.total_removes += 1;
        }
        removed
    }

    fn remove_recursive(node: &mut TrieNode, key: &str, idx: usize) -> Option<String> {
        let chars: Vec<char> = key.chars().collect();
        if idx == chars.len() {
            if node.is_terminal() {
                let val = node.value().map(|v| v.to_string());
                node.clear_terminal();
                return val;
            }
            return None;
        }
        let c = chars[idx];
        let result = {
            {
                let child = node.child_mut(c)?;
                Self::remove_recursive(child, key, idx + 1)
            }
        };
        if result.is_some() {
            // Clean up empty non-terminal nodes
            if let Some(child) = node.child(c) {
                if !child.is_terminal() && !child.has_children() {
                    node.remove_child(c);
                }
            }
        }
        result
    }

    /// Find all keys with a given prefix.
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.child(c) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }
        let mut results = Vec::new();
        Self::collect_keys(node, &mut prefix.to_string(), &mut results);
        results
    }

    fn collect_keys(node: &TrieNode, prefix: &mut String, results: &mut Vec<String>) {
        if node.is_terminal() {
            results.push(prefix.clone());
        }
        let mut chars: Vec<char> = node.children().keys().copied().collect();
        chars.sort();
        for c in chars {
            prefix.push(c);
            if let Some(child) = node.child(c) {
                Self::collect_keys(child, prefix, results);
            }
            prefix.pop();
        }
    }

    /// Autocomplete: find up to `limit` keys with the given prefix.
    pub fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<String> {
        let all = self.keys_with_prefix(prefix);
        all.into_iter().take(limit).collect()
    }

    /// Check if any key starts with the given prefix.
    pub fn has_prefix(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.child(c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }

    /// Number of keys stored.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the trie is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.root = TrieNode::new(0);
        self.count = 0;
    }

    /// Hit rate (hits / lookups).
    pub fn hit_rate(&self) -> f64 {
        if self.total_lookups == 0 {
            return 0.0;
        }
        self.total_hits as f64 / self.total_lookups as f64
    }

    /// Total inserts (lifetime).
    pub fn total_inserts(&self) -> u64 {
        self.total_inserts
    }

    /// Total removes (lifetime).
    pub fn total_removes(&self) -> u64 {
        self.total_removes
    }

    /// Total lookups (lifetime).
    pub fn total_lookups(&self) -> u64 {
        self.total_lookups
    }
}

impl Default for PrefixTrie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut trie = PrefixTrie::new();
        trie.insert("hello", "world");
        assert_eq!(trie.get("hello"), Some("world"));
        assert_eq!(trie.len(), 1);
    }

    #[test]
    fn test_insert_update() {
        let mut trie = PrefixTrie::new();
        assert!(trie.insert("key", "v1").is_none());
        let old = trie.insert("key", "v2");
        assert_eq!(old, Some("v1".to_string()));
        assert_eq!(trie.get("key"), Some("v2"));
        assert_eq!(trie.len(), 1);
    }

    #[test]
    fn test_contains() {
        let mut trie = PrefixTrie::new();
        trie.insert("abc", "val");
        assert!(trie.contains("abc"));
        assert!(!trie.contains("ab"));
        assert!(!trie.contains("abcd"));
    }

    #[test]
    fn test_remove() {
        let mut trie = PrefixTrie::new();
        trie.insert("hello", "world");
        let removed = trie.remove("hello");
        assert_eq!(removed, Some("world".to_string()));
        assert_eq!(trie.len(), 0);
        assert!(!trie.contains("hello"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut trie = PrefixTrie::new();
        assert!(trie.remove("missing").is_none());
    }

    #[test]
    fn test_prefix_search() {
        let mut trie = PrefixTrie::new();
        trie.insert("car", "1");
        trie.insert("card", "2");
        trie.insert("care", "3");
        trie.insert("cat", "4");
        let results = trie.keys_with_prefix("car");
        assert_eq!(results, vec!["car", "card", "care"]);
    }

    #[test]
    fn test_autocomplete() {
        let mut trie = PrefixTrie::new();
        trie.insert("apple", "1");
        trie.insert("app", "2");
        trie.insert("application", "3");
        trie.insert("banana", "4");
        let results = trie.autocomplete("app", 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "app");
    }

    #[test]
    fn test_has_prefix() {
        let mut trie = PrefixTrie::new();
        trie.insert("hello", "world");
        assert!(trie.has_prefix("hel"));
        assert!(trie.has_prefix("hello"));
        assert!(!trie.has_prefix("help"));
    }

    #[test]
    fn test_clear() {
        let mut trie = PrefixTrie::new();
        trie.insert("a", "1");
        trie.insert("b", "2");
        trie.clear();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_hit_rate() {
        let mut trie = PrefixTrie::new();
        trie.insert("key", "val");
        trie.get("key"); // hit
        trie.get("missing"); // miss
        assert!((trie.hit_rate() - 0.5).abs() < f64::EPSILON);
    }
}
