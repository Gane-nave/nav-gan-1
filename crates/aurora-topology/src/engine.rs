//! Deep topology awareness — understands complex interchanges,
//! multi-level structures, and graph constraints.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoadLevel {
    Underground,
    Ground,
    Elevated,
    Bridge,
    Tunnel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyNode {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub level: RoadLevel,
    pub connections: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyGraph {
    nodes: Vec<TopologyNode>,
    constraints: Vec<TopologyConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyConstraint {
    NoUturn(u64),
    OneWay(u64, u64),
    LevelSeparation(u64, u64),
    MaxHeight { node: u64, max_m: f64 },
    MaxWeight { node: u64, max_kg: f64 },
}

impl TopologyGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: TopologyNode) {
        self.nodes.push(node);
    }

    pub fn add_constraint(&mut self, c: TopologyConstraint) {
        self.constraints.push(c);
    }

    pub fn get_node(&self, id: u64) -> Option<&TopologyNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Check if two nodes are truly connected (same level or explicit connection).
    pub fn are_connected(&self, from: u64, to: u64) -> bool {
        if let Some(node) = self.get_node(from) {
            if !node.connections.contains(&to) {
                return false;
            }
            // Check level separation constraint
            for c in &self.constraints {
                if let TopologyConstraint::LevelSeparation(a, b) = c {
                    if (*a == from && *b == to) || (*a == to && *b == from) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// Check if a transition violates any constraint.
    pub fn check_constraints(
        &self,
        from: u64,
        to: u64,
        vehicle_height_m: f64,
        vehicle_weight_kg: f64,
    ) -> Vec<String> {
        let mut violations = Vec::new();
        for c in &self.constraints {
            match c {
                TopologyConstraint::NoUturn(n) => {
                    if from == *n && to == *n {
                        violations.push("U-turn not allowed".into());
                    }
                }
                TopologyConstraint::OneWay(a, b) => {
                    if from == *b && to == *a {
                        violations.push("Wrong way on one-way".into());
                    }
                }
                TopologyConstraint::MaxHeight { node, max_m } => {
                    if (from == *node || to == *node) && vehicle_height_m > *max_m {
                        violations.push(format!("Height {vehicle_height_m}m exceeds max {max_m}m"));
                    }
                }
                TopologyConstraint::MaxWeight { node, max_kg } => {
                    if (from == *node || to == *node) && vehicle_weight_kg > *max_kg {
                        violations.push(format!(
                            "Weight {vehicle_weight_kg}kg exceeds max {max_kg}kg"
                        ));
                    }
                }
                TopologyConstraint::LevelSeparation(_, _) => {}
            }
        }
        violations
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}

impl Default for TopologyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let g = TopologyGraph::new();
        assert_eq!(g.node_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut g = TopologyGraph::new();
        g.add_node(TopologyNode {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            level: RoadLevel::Ground,
            connections: vec![2],
        });
        assert_eq!(g.node_count(), 1);
    }

    #[test]
    fn test_connected() {
        let mut g = TopologyGraph::new();
        g.add_node(TopologyNode {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            level: RoadLevel::Ground,
            connections: vec![2],
        });
        g.add_node(TopologyNode {
            id: 2,
            lat: 32.1,
            lon: 34.1,
            level: RoadLevel::Ground,
            connections: vec![1],
        });
        assert!(g.are_connected(1, 2));
    }

    #[test]
    fn test_level_separation() {
        let mut g = TopologyGraph::new();
        g.add_node(TopologyNode {
            id: 1,
            lat: 32.0,
            lon: 34.0,
            level: RoadLevel::Ground,
            connections: vec![2],
        });
        g.add_node(TopologyNode {
            id: 2,
            lat: 32.0,
            lon: 34.0,
            level: RoadLevel::Bridge,
            connections: vec![1],
        });
        g.add_constraint(TopologyConstraint::LevelSeparation(1, 2));
        assert!(!g.are_connected(1, 2));
    }

    #[test]
    fn test_height_constraint() {
        let mut g = TopologyGraph::new();
        g.add_constraint(TopologyConstraint::MaxHeight {
            node: 1,
            max_m: 3.5,
        });
        let v = g.check_constraints(1, 2, 4.0, 1000.0);
        assert!(!v.is_empty());
    }

    #[test]
    fn test_one_way() {
        let mut g = TopologyGraph::new();
        g.add_constraint(TopologyConstraint::OneWay(1, 2));
        let v = g.check_constraints(2, 1, 2.0, 1000.0);
        assert!(v.iter().any(|s| s.contains("Wrong way")));
    }

    #[test]
    fn test_no_uturn() {
        let mut g = TopologyGraph::new();
        g.add_constraint(TopologyConstraint::NoUturn(5));
        let v = g.check_constraints(5, 5, 2.0, 1000.0);
        assert!(!v.is_empty());
    }

    #[test]
    fn test_default() {
        let g = TopologyGraph::default();
        assert_eq!(g.constraint_count(), 0);
    }
}
