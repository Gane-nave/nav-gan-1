//! Topological sort — linearize a DAG into a valid execution order.

use crate::graph::Dag;
use std::collections::{HashMap, VecDeque};

/// Result of a topological sort.
#[derive(Debug, PartialEq)]
pub enum TopoResult {
    /// Valid topological order.
    Order(Vec<String>),
    /// Graph has a cycle (should not happen with Dag's cycle prevention).
    Cycle,
}

/// Perform Kahn's algorithm for topological sorting.
pub fn topological_sort(dag: &Dag) -> TopoResult {
    let mut in_degrees: HashMap<String, usize> = HashMap::new();

    // Initialize in-degrees
    for node in dag.nodes_set() {
        in_degrees.insert(node.clone(), dag.in_degree(node));
    }

    // Start with nodes that have no incoming edges
    let mut queue: VecDeque<String> = in_degrees
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(n, _)| n.clone())
        .collect();

    // Sort the queue for deterministic output
    let mut sorted_start: Vec<String> = queue.drain(..).collect();
    sorted_start.sort();
    for n in sorted_start {
        queue.push_back(n);
    }

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.clone());

        // Get successors sorted for deterministic output
        let mut succs: Vec<String> = dag
            .edges_map()
            .get(&node)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        succs.sort();

        for succ in succs {
            if let Some(deg) = in_degrees.get_mut(&succ) {
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(succ);
                }
            }
        }
    }

    if result.len() == dag.node_count() {
        TopoResult::Order(result)
    } else {
        TopoResult::Cycle
    }
}

/// Get the depth of each node from the roots.
pub fn node_depths(dag: &Dag) -> HashMap<String, usize> {
    let mut depths: HashMap<String, usize> = HashMap::new();

    if let TopoResult::Order(order) = topological_sort(dag) {
        for node in &order {
            let max_pred_depth = dag
                .predecessors(node)
                .iter()
                .filter_map(|p| depths.get(p))
                .max()
                .copied()
                .unwrap_or(0);

            let depth = if dag.in_degree(node) == 0 {
                0
            } else {
                max_pred_depth + 1
            };
            depths.insert(node.clone(), depth);
        }
    }

    depths
}

/// Get the critical path length (longest path from any root to any leaf).
pub fn critical_path_length(dag: &Dag) -> usize {
    let depths = node_depths(dag);
    depths.values().max().copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort_linear() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("b", "c");
        assert_eq!(
            topological_sort(&dag),
            TopoResult::Order(vec!["a".into(), "b".into(), "c".into()])
        );
    }

    #[test]
    fn test_topological_sort_diamond() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("a", "c");
        dag.add_edge("b", "d");
        dag.add_edge("c", "d");
        let result = topological_sort(&dag);
        if let TopoResult::Order(order) = result {
            // a must come before b, c; b and c before d
            let pos = |n: &str| order.iter().position(|x| x == n).unwrap();
            assert!(pos("a") < pos("b"));
            assert!(pos("a") < pos("c"));
            assert!(pos("b") < pos("d"));
            assert!(pos("c") < pos("d"));
        } else {
            panic!("Expected order, got cycle");
        }
    }

    #[test]
    fn test_topological_sort_disconnected() {
        let mut dag = Dag::new();
        dag.add_node("x");
        dag.add_node("y");
        dag.add_edge("a", "b");
        let result = topological_sort(&dag);
        if let TopoResult::Order(order) = result {
            assert_eq!(order.len(), 4);
        } else {
            panic!("Expected order");
        }
    }

    #[test]
    fn test_node_depths() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("a", "c");
        dag.add_edge("b", "d");
        dag.add_edge("c", "d");
        let depths = node_depths(&dag);
        assert_eq!(depths.get("a"), Some(&0));
        assert_eq!(depths.get("b"), Some(&1));
        assert_eq!(depths.get("c"), Some(&1));
        assert_eq!(depths.get("d"), Some(&2));
    }

    #[test]
    fn test_critical_path_length() {
        let mut dag = Dag::new();
        dag.add_edge("a", "b");
        dag.add_edge("b", "c");
        dag.add_edge("c", "d");
        assert_eq!(critical_path_length(&dag), 3);
    }

    #[test]
    fn test_empty_dag_sort() {
        let dag = Dag::new();
        assert_eq!(topological_sort(&dag), TopoResult::Order(vec![]));
    }

    #[test]
    fn test_single_node_sort() {
        let mut dag = Dag::new();
        dag.add_node("solo");
        assert_eq!(
            topological_sort(&dag),
            TopoResult::Order(vec!["solo".into()])
        );
    }

    #[test]
    fn test_node_depths_linear() {
        let mut dag = Dag::new();
        dag.add_edge("start", "middle");
        dag.add_edge("middle", "end");
        let depths = node_depths(&dag);
        assert_eq!(depths.get("start"), Some(&0));
        assert_eq!(depths.get("middle"), Some(&1));
        assert_eq!(depths.get("end"), Some(&2));
    }
}
