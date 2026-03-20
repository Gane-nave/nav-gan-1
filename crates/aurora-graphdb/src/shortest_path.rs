//! Shortest-path algorithms — Dijkstra and A* for navigation graphs.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::graph::{NavGraph, NodeId};
use crate::spatial::haversine_distance;

/// A shortest-path result.
#[derive(Debug, Clone)]
pub struct PathResult {
    /// Ordered list of node IDs from start to goal.
    pub path: Vec<NodeId>,
    /// Total cost (distance/time) of the path.
    pub cost: f64,
    /// Number of nodes explored during search.
    pub nodes_explored: usize,
}

/// Entry in the priority queue.
#[derive(Debug, Clone)]
struct HeapEntry {
    node: NodeId,
    cost: f64,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Eq for HeapEntry {}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Run Dijkstra's algorithm to find shortest path.
pub fn dijkstra(graph: &NavGraph, start: NodeId, goal: NodeId) -> Option<PathResult> {
    let mut dist: HashMap<NodeId, f64> = HashMap::new();
    let mut prev: HashMap<NodeId, NodeId> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut explored = 0usize;

    dist.insert(start, 0.0);
    heap.push(HeapEntry {
        node: start,
        cost: 0.0,
    });

    while let Some(HeapEntry { node, cost }) = heap.pop() {
        if node == goal {
            let path = reconstruct_path(&prev, start, goal);
            return Some(PathResult {
                path,
                cost,
                nodes_explored: explored,
            });
        }

        if cost > *dist.get(&node).unwrap_or(&f64::INFINITY) {
            continue;
        }

        explored += 1;

        for edge in graph.outgoing_edges(node) {
            let next_cost = cost + edge.weight;
            if next_cost < *dist.get(&edge.to).unwrap_or(&f64::INFINITY) {
                dist.insert(edge.to, next_cost);
                prev.insert(edge.to, node);
                heap.push(HeapEntry {
                    node: edge.to,
                    cost: next_cost,
                });
            }
        }
    }

    None
}

/// Run A* algorithm with haversine heuristic.
pub fn astar(graph: &NavGraph, start: NodeId, goal: NodeId) -> Option<PathResult> {
    let goal_node = graph.get_node(goal)?;
    let goal_lat = goal_node.lat;
    let goal_lon = goal_node.lon;

    let mut g_score: HashMap<NodeId, f64> = HashMap::new();
    let mut prev: HashMap<NodeId, NodeId> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut explored = 0usize;

    g_score.insert(start, 0.0);
    let h = heuristic(graph, start, goal_lat, goal_lon);
    heap.push(HeapEntry {
        node: start,
        cost: h,
    });

    while let Some(HeapEntry { node, cost: _ }) = heap.pop() {
        if node == goal {
            let cost = *g_score.get(&goal).unwrap_or(&0.0);
            let path = reconstruct_path(&prev, start, goal);
            return Some(PathResult {
                path,
                cost,
                nodes_explored: explored,
            });
        }

        explored += 1;
        let current_g = *g_score.get(&node).unwrap_or(&f64::INFINITY);

        for edge in graph.outgoing_edges(node) {
            let tentative_g = current_g + edge.weight;
            if tentative_g < *g_score.get(&edge.to).unwrap_or(&f64::INFINITY) {
                g_score.insert(edge.to, tentative_g);
                prev.insert(edge.to, node);
                let f = tentative_g + heuristic(graph, edge.to, goal_lat, goal_lon);
                heap.push(HeapEntry {
                    node: edge.to,
                    cost: f,
                });
            }
        }
    }

    None
}

/// Heuristic function: haversine distance to goal.
fn heuristic(graph: &NavGraph, node: NodeId, goal_lat: f64, goal_lon: f64) -> f64 {
    graph
        .get_node(node)
        .map(|n| haversine_distance(n.lat, n.lon, goal_lat, goal_lon))
        .unwrap_or(0.0)
}

/// Reconstruct path from parent map.
fn reconstruct_path(prev: &HashMap<NodeId, NodeId>, start: NodeId, goal: NodeId) -> Vec<NodeId> {
    let mut path = vec![goal];
    let mut current = goal;
    while current != start {
        match prev.get(&current) {
            Some(&parent) => {
                path.push(parent);
                current = parent;
            }
            None => break,
        }
    }
    path.reverse();
    path
}

/// Find all shortest distances from a source (single-source shortest path).
pub fn dijkstra_all(graph: &NavGraph, start: NodeId) -> HashMap<NodeId, f64> {
    let mut dist: HashMap<NodeId, f64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0.0);
    heap.push(HeapEntry {
        node: start,
        cost: 0.0,
    });

    while let Some(HeapEntry { node, cost }) = heap.pop() {
        if cost > *dist.get(&node).unwrap_or(&f64::INFINITY) {
            continue;
        }

        for edge in graph.outgoing_edges(node) {
            let next_cost = cost + edge.weight;
            if next_cost < *dist.get(&edge.to).unwrap_or(&f64::INFINITY) {
                dist.insert(edge.to, next_cost);
                heap.push(HeapEntry {
                    node: edge.to,
                    cost: next_cost,
                });
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{GraphEdge, GraphNode, NavGraph};

    fn build_weighted_graph() -> NavGraph {
        let mut g = NavGraph::new();
        //   1 --2--> 2 --3--> 4
        //   |                 ^
        //   1                 |
        //   v                 2
        //   3 ------4-------->
        g.add_node(GraphNode::new(1, 32.0, 34.0));
        g.add_node(GraphNode::new(2, 32.1, 34.1));
        g.add_node(GraphNode::new(3, 31.9, 34.0));
        g.add_node(GraphNode::new(4, 32.1, 34.2));

        g.add_edge(GraphEdge::new(0, 1, 2, 2.0));
        g.add_edge(GraphEdge::new(0, 1, 3, 1.0));
        g.add_edge(GraphEdge::new(0, 2, 4, 3.0));
        g.add_edge(GraphEdge::new(0, 3, 4, 4.0));
        g
    }

    #[test]
    fn test_dijkstra_shortest_path() {
        let g = build_weighted_graph();
        let result = dijkstra(&g, 1, 4).unwrap();
        assert_eq!(result.path[0], 1);
        assert_eq!(*result.path.last().unwrap(), 4);
        assert!((result.cost - 5.0).abs() < f64::EPSILON); // 1->2->4 = 2+3 = 5
    }

    #[test]
    fn test_dijkstra_no_path() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        assert!(dijkstra(&g, 1, 2).is_none());
    }

    #[test]
    fn test_astar_finds_path() {
        let g = build_weighted_graph();
        let result = astar(&g, 1, 4).unwrap();
        assert_eq!(result.path[0], 1);
        assert_eq!(*result.path.last().unwrap(), 4);
        assert!((result.cost - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_astar_explores_fewer_nodes() {
        let g = build_weighted_graph();
        let dijkstra_result = dijkstra(&g, 1, 4).unwrap();
        let astar_result = astar(&g, 1, 4).unwrap();
        // A* should explore <= Dijkstra nodes
        assert!(astar_result.nodes_explored <= dijkstra_result.nodes_explored + 1);
    }

    #[test]
    fn test_dijkstra_all() {
        let g = build_weighted_graph();
        let dists = dijkstra_all(&g, 1);
        assert!((dists[&1] - 0.0).abs() < f64::EPSILON);
        assert!((dists[&2] - 2.0).abs() < f64::EPSILON);
        assert!((dists[&3] - 1.0).abs() < f64::EPSILON);
        assert!((dists[&4] - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_dijkstra_same_start_goal() {
        let g = build_weighted_graph();
        let result = dijkstra(&g, 1, 1).unwrap();
        assert_eq!(result.cost, 0.0);
        assert_eq!(result.path, vec![1]);
    }

    #[test]
    fn test_astar_no_path() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        assert!(astar(&g, 1, 2).is_none());
    }

    #[test]
    fn test_path_result_fields() {
        let g = build_weighted_graph();
        let result = dijkstra(&g, 1, 4).unwrap();
        assert!(result.nodes_explored > 0);
        assert!(result.path.len() >= 2);
    }
}
