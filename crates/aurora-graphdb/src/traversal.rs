//! Graph traversal algorithms — BFS, DFS, and connected components.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::{NavGraph, NodeId};

/// Breadth-first search result.
#[derive(Debug)]
pub struct BfsResult {
    /// Visit order.
    pub visited: Vec<NodeId>,
    /// Parent map for path reconstruction.
    pub parents: HashMap<NodeId, NodeId>,
    /// Depth of each visited node.
    pub depths: HashMap<NodeId, usize>,
}

impl BfsResult {
    /// Reconstruct path from start to target using parent map.
    pub fn path_to(&self, target: NodeId) -> Option<Vec<NodeId>> {
        if !self.parents.contains_key(&target) && self.visited.first() != Some(&target) {
            return None;
        }
        let mut path = vec![target];
        let mut current = target;
        while let Some(&parent) = self.parents.get(&current) {
            path.push(parent);
            current = parent;
        }
        path.reverse();
        Some(path)
    }
}

/// Run BFS from a start node.
pub fn bfs(graph: &NavGraph, start: NodeId) -> BfsResult {
    let mut visited = Vec::new();
    let mut parents = HashMap::new();
    let mut depths = HashMap::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    seen.insert(start);
    queue.push_back(start);
    depths.insert(start, 0);

    while let Some(node) = queue.pop_front() {
        visited.push(node);
        let depth = depths[&node];

        for neighbor in graph.neighbors(node) {
            if seen.insert(neighbor) {
                parents.insert(neighbor, node);
                depths.insert(neighbor, depth + 1);
                queue.push_back(neighbor);
            }
        }
    }

    BfsResult {
        visited,
        parents,
        depths,
    }
}

/// Depth-first search result.
#[derive(Debug)]
pub struct DfsResult {
    /// Visit order.
    pub visited: Vec<NodeId>,
    /// Discovery time for each node.
    pub discovery: HashMap<NodeId, usize>,
    /// Finish time for each node.
    pub finish: HashMap<NodeId, usize>,
}

/// Run DFS from a start node.
pub fn dfs(graph: &NavGraph, start: NodeId) -> DfsResult {
    let mut visited = Vec::new();
    let mut discovery = HashMap::new();
    let mut finish = HashMap::new();
    let mut seen = HashSet::new();
    let mut time = 0;

    fn dfs_visit(
        graph: &NavGraph,
        node: NodeId,
        seen: &mut HashSet<NodeId>,
        visited: &mut Vec<NodeId>,
        discovery: &mut HashMap<NodeId, usize>,
        finish: &mut HashMap<NodeId, usize>,
        time: &mut usize,
    ) {
        seen.insert(node);
        *time += 1;
        discovery.insert(node, *time);
        visited.push(node);

        for neighbor in graph.neighbors(node) {
            if !seen.contains(&neighbor) {
                dfs_visit(graph, neighbor, seen, visited, discovery, finish, time);
            }
        }

        *time += 1;
        finish.insert(node, *time);
    }

    dfs_visit(
        graph,
        start,
        &mut seen,
        &mut visited,
        &mut discovery,
        &mut finish,
        &mut time,
    );

    DfsResult {
        visited,
        discovery,
        finish,
    }
}

/// Find connected components (treating edges as undirected).
///
/// Uses undirected BFS that follows both outgoing and incoming edges,
/// producing weakly-connected components for directed graphs.
pub fn connected_components(graph: &NavGraph) -> Vec<Vec<NodeId>> {
    let mut seen = HashSet::new();
    let mut components = Vec::new();

    for &node_id in &graph.node_ids() {
        if seen.contains(&node_id) {
            continue;
        }
        // Undirected BFS: follow both outgoing and incoming edges
        let mut queue = VecDeque::new();
        let mut component = Vec::new();
        seen.insert(node_id);
        queue.push_back(node_id);

        while let Some(current) = queue.pop_front() {
            component.push(current);
            // Follow outgoing edges
            for neighbor in graph.neighbors(current) {
                if seen.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
            // Follow incoming edges (treat as undirected)
            for neighbor in graph.incoming_neighbors(current) {
                if seen.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        component.sort();
        components.push(component);
    }

    components
}

/// Check if the graph is connected (treating edges as directed).
pub fn is_reachable(graph: &NavGraph, from: NodeId, to: NodeId) -> bool {
    let result = bfs(graph, from);
    result.visited.contains(&to)
}

/// Topological sort (returns None if cycle detected).
pub fn topological_sort(graph: &NavGraph) -> Option<Vec<NodeId>> {
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    for &nid in &graph.node_ids() {
        in_degree.entry(nid).or_insert(0);
        for edge in graph.outgoing_edges(nid) {
            *in_degree.entry(edge.to).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<NodeId> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        for edge in graph.outgoing_edges(node) {
            if let Some(deg) = in_degree.get_mut(&edge.to) {
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(edge.to);
                }
            }
        }
    }

    if sorted.len() == graph.node_count() {
        Some(sorted)
    } else {
        None // Cycle detected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{GraphEdge, GraphNode, NavGraph};

    fn build_graph() -> NavGraph {
        let mut g = NavGraph::new();
        // 1 -> 2 -> 3 -> 4
        // 1 -> 3
        for i in 1..=4 {
            g.add_node(GraphNode::new(i, i as f64, i as f64));
        }
        g.add_edge(GraphEdge::new(0, 1, 2, 1.0));
        g.add_edge(GraphEdge::new(0, 2, 3, 1.0));
        g.add_edge(GraphEdge::new(0, 3, 4, 1.0));
        g.add_edge(GraphEdge::new(0, 1, 3, 2.0));
        g
    }

    #[test]
    fn test_bfs_visit_order() {
        let g = build_graph();
        let result = bfs(&g, 1);
        assert_eq!(result.visited[0], 1);
        assert!(result.visited.contains(&2));
        assert!(result.visited.contains(&3));
        assert!(result.visited.contains(&4));
    }

    #[test]
    fn test_bfs_path_reconstruction() {
        let g = build_graph();
        let result = bfs(&g, 1);
        let path = result.path_to(4).unwrap();
        assert_eq!(path[0], 1);
        assert_eq!(*path.last().unwrap(), 4);
    }

    #[test]
    fn test_bfs_depths() {
        let g = build_graph();
        let result = bfs(&g, 1);
        assert_eq!(result.depths[&1], 0);
        assert_eq!(result.depths[&2], 1);
        // Node 3 can be reached at depth 1 (direct) or depth 2 (via 2)
        assert!(result.depths[&3] <= 2);
    }

    #[test]
    fn test_dfs() {
        let g = build_graph();
        let result = dfs(&g, 1);
        assert_eq!(result.visited[0], 1);
        assert_eq!(result.visited.len(), 4);
        // Discovery time should be earlier than finish time for each node
        for &nid in &result.visited {
            assert!(result.discovery[&nid] < result.finish[&nid]);
        }
    }

    #[test]
    fn test_reachability() {
        let g = build_graph();
        assert!(is_reachable(&g, 1, 4));
        assert!(!is_reachable(&g, 4, 1));
    }

    #[test]
    fn test_topological_sort() {
        let g = build_graph();
        let sorted = topological_sort(&g).unwrap();
        assert_eq!(sorted.len(), 4);
        // 1 must come before 2, 3; 2 before 3; 3 before 4
        let pos = |id: NodeId| sorted.iter().position(|&x| x == id).unwrap();
        assert!(pos(1) < pos(2));
        assert!(pos(1) < pos(3));
        assert!(pos(3) < pos(4));
    }

    #[test]
    fn test_connected_components() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_node(GraphNode::new(3, 2.0, 2.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 1.0));
        // Node 3 is isolated
        let comps = connected_components(&g);
        assert!(comps.len() >= 2);
    }

    #[test]
    fn test_connected_components_directed_edge_undirected() {
        // Regression: A->B as directed edge should still put A and B
        // in the same weakly-connected component.
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_node(GraphNode::new(3, 2.0, 2.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 1.0)); // directed 1->2
                                                  // Node 3 isolated
        let comps = connected_components(&g);
        // Nodes 1 and 2 must be in the SAME component
        let comp_with_1 = comps.iter().find(|c| c.contains(&1)).unwrap();
        assert!(
            comp_with_1.contains(&2),
            "directed edge 1->2 should form one undirected component"
        );
        // Node 3 must be in its own component
        let comp_with_3 = comps.iter().find(|c| c.contains(&3)).unwrap();
        assert_eq!(comp_with_3.len(), 1);
        assert_eq!(comps.len(), 2);
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 1.0));
        g.add_edge(GraphEdge::new(0, 2, 1, 1.0)); // cycle
        assert!(topological_sort(&g).is_none());
    }

    #[test]
    fn test_bfs_unreachable_target() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        let result = bfs(&g, 1);
        assert!(result.path_to(2).is_none());
    }
}
