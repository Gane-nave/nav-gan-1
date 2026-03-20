//! Directed Acyclic Graph — nodes and edges with cycle detection.

use std::collections::{HashMap, HashSet, VecDeque};

/// A directed acyclic graph with string-labeled nodes.
pub struct Dag {
    /// Adjacency list: node -> set of successor nodes.
    edges: HashMap<String, HashSet<String>>,
    /// All known nodes.
    nodes: HashSet<String>,
    /// Total edges added (lifetime).
    total_edges_added: u64,
    /// Total edges removed (lifetime).
    total_edges_removed: u64,
    /// Total cycle checks (lifetime).
    total_cycle_checks: u64,
}

impl Dag {
    /// Create a new empty DAG.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
            total_edges_added: 0,
            total_edges_removed: 0,
            total_cycle_checks: 0,
        }
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, name: &str) {
        self.nodes.insert(name.to_string());
    }

    /// Add a directed edge from `from` to `to`.
    /// Returns false if adding this edge would create a cycle.
    pub fn add_edge(&mut self, from: &str, to: &str) -> bool {
        // Check if adding this edge would create a cycle
        // A cycle would exist if `to` can already reach `from`
        if from == to {
            return false;
        }
        self.nodes.insert(from.to_string());
        self.nodes.insert(to.to_string());

        if self.can_reach(to, from) {
            return false;
        }

        self.edges
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
        self.total_edges_added += 1;
        true
    }

    /// Remove a directed edge.
    pub fn remove_edge(&mut self, from: &str, to: &str) -> bool {
        if let Some(succs) = self.edges.get_mut(from) {
            if succs.remove(to) {
                self.total_edges_removed += 1;
                if succs.is_empty() {
                    self.edges.remove(from);
                }
                return true;
            }
        }
        false
    }

    /// Remove a node and all its edges.
    pub fn remove_node(&mut self, name: &str) -> bool {
        if !self.nodes.remove(name) {
            return false;
        }
        self.edges.remove(name);
        // Remove incoming edges
        for succs in self.edges.values_mut() {
            succs.remove(name);
        }
        // Clean up empty entries
        self.edges.retain(|_, v| !v.is_empty());
        true
    }

    /// Check if node `from` can reach node `to` via directed edges.
    pub fn can_reach(&mut self, from: &str, to: &str) -> bool {
        self.total_cycle_checks += 1;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(from.to_string());

        while let Some(current) = queue.pop_front() {
            if current == to {
                return true;
            }
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if let Some(succs) = self.edges.get(&current) {
                for succ in succs {
                    if !visited.contains(succ) {
                        queue.push_back(succ.clone());
                    }
                }
            }
        }
        false
    }

    /// Get the successors of a node.
    pub fn successors(&self, name: &str) -> Vec<String> {
        match self.edges.get(name) {
            Some(succs) => {
                let mut v: Vec<String> = succs.iter().cloned().collect();
                v.sort();
                v
            }
            None => Vec::new(),
        }
    }

    /// Get the predecessors of a node.
    pub fn predecessors(&self, name: &str) -> Vec<String> {
        let mut preds = Vec::new();
        for (from, succs) in &self.edges {
            if succs.contains(name) {
                preds.push(from.clone());
            }
        }
        preds.sort();
        preds
    }

    /// Get the in-degree of a node (number of incoming edges).
    pub fn in_degree(&self, name: &str) -> usize {
        self.edges
            .values()
            .filter(|succs| succs.contains(name))
            .count()
    }

    /// Get the out-degree of a node (number of outgoing edges).
    pub fn out_degree(&self, name: &str) -> usize {
        self.edges.get(name).map_or(0, |s| s.len())
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|s| s.len()).sum()
    }

    /// Check if the graph contains a node.
    pub fn has_node(&self, name: &str) -> bool {
        self.nodes.contains(name)
    }

    /// Check if the graph contains an edge.
    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        self.edges.get(from).is_some_and(|s| s.contains(to))
    }

    /// Get all nodes.
    pub fn nodes(&self) -> Vec<String> {
        let mut v: Vec<String> = self.nodes.iter().cloned().collect();
        v.sort();
        v
    }

    /// Get root nodes (nodes with no incoming edges).
    pub fn roots(&self) -> Vec<String> {
        let mut roots: Vec<String> = self
            .nodes
            .iter()
            .filter(|n| self.in_degree(n) == 0)
            .cloned()
            .collect();
        roots.sort();
        roots
    }

    /// Get leaf nodes (nodes with no outgoing edges).
    pub fn leaves(&self) -> Vec<String> {
        let mut leaves: Vec<String> = self
            .nodes
            .iter()
            .filter(|n| self.out_degree(n) == 0)
            .cloned()
            .collect();
        leaves.sort();
        leaves
    }

    /// Clear the graph.
    pub fn clear(&mut self) {
        self.edges.clear();
        self.nodes.clear();
    }

    /// Get a reference to the edges map (for topological sort).
    pub fn edges_map(&self) -> &HashMap<String, HashSet<String>> {
        &self.edges
    }

    /// Get a reference to the nodes set.
    pub fn nodes_set(&self) -> &HashSet<String> {
        &self.nodes
    }

    /// Total edges added (lifetime).
    pub fn total_edges_added(&self) -> u64 {
        self.total_edges_added
    }

    /// Total edges removed (lifetime).
    pub fn total_edges_removed(&self) -> u64 {
        self.total_edges_removed
    }
}

impl Default for Dag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node_and_edge() {
        let mut dag = Dag::new();
        assert!(dag.add_edge("a", "b"));
        assert!(dag.has_node("a"));
        assert!(dag.has_node("b"));
        assert!(dag.has_edge("a", "b"));
        assert_eq!(dag.node_count(), 2);
        assert_eq!(dag.edge_count(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("b", "c");
        assert!(!dag.add_edge("c", "a")); // would create cycle
        assert!(!dag.add_edge("a", "a")); // self-loop
    }

    #[test]
    fn test_successors_predecessors() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("a", "c");
        dag.add_edge("b", "d");
        assert_eq!(dag.successors("a"), vec!["b", "c"]);
        assert_eq!(dag.predecessors("d"), vec!["b"]);
    }

    #[test]
    fn test_in_out_degree() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("a", "c");
        dag.add_edge("c", "b");
        assert_eq!(dag.out_degree("a"), 2);
        assert_eq!(dag.in_degree("b"), 2);
    }

    #[test]
    fn test_roots_and_leaves() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("a", "c");
        dag.add_edge("b", "d");
        dag.add_edge("c", "d");
        assert_eq!(dag.roots(), vec!["a"]);
        assert_eq!(dag.leaves(), vec!["d"]);
    }

    #[test]
    fn test_remove_edge() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        assert!(dag.remove_edge("a", "b"));
        assert!(!dag.has_edge("a", "b"));
        assert_eq!(dag.edge_count(), 0);
    }

    #[test]
    fn test_remove_node() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("b", "c");
        dag.remove_node("b");
        assert!(!dag.has_node("b"));
        assert!(!dag.has_edge("a", "b"));
        assert_eq!(dag.node_count(), 2);
    }

    #[test]
    fn test_can_reach() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("b", "c");
        dag.add_edge("c", "d");
        assert!(dag.can_reach("a", "d"));
        assert!(!dag.can_reach("d", "a"));
    }

    #[test]
    fn test_clear() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.clear();
        assert_eq!(dag.node_count(), 0);
        assert_eq!(dag.edge_count(), 0);
    }

    #[test]
    fn test_nodes_sorted() {
        let mut dag = Dag::new();
        dag.add_node("c");
        dag.add_node("a");
        dag.add_node("b");
        assert_eq!(dag.nodes(), vec!["a", "b", "c"]);
    }
}
