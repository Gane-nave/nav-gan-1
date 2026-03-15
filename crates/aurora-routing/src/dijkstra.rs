//! Dijkstra shortest-path with pluggable cost functions.

use aurora_core::types::EntityId;
use aurora_map::graph::RoadGraphIndex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A node in the priority queue.
#[derive(Debug, Clone)]
struct DijkstraNode {
    node_id: EntityId,
    cost: f64,
}

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap.
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

/// Cost function type: takes a segment and returns its cost.
pub type CostFn = Box<dyn Fn(&aurora_core::map::RoadSegment) -> f64>;

/// Result of a shortest-path search.
#[derive(Debug, Clone)]
pub struct ShortestPath {
    /// Ordered list of node IDs from source to destination.
    pub nodes: Vec<EntityId>,
    /// Ordered list of segment IDs traversed.
    pub segments: Vec<EntityId>,
    /// Total cost.
    pub total_cost: f64,
}

/// Run Dijkstra's algorithm on the road graph.
///
/// `cost_fn` determines how each segment is weighted.
/// Returns `None` if no path exists.
pub fn shortest_path(
    index: &RoadGraphIndex,
    from: EntityId,
    to: EntityId,
    cost_fn: &CostFn,
) -> Option<ShortestPath> {
    let mut dist: HashMap<EntityId, f64> = HashMap::new();
    let mut prev: HashMap<EntityId, (EntityId, EntityId)> = HashMap::new(); // node → (prev_node, via_segment)
    let mut heap = BinaryHeap::new();

    dist.insert(from, 0.0);
    heap.push(DijkstraNode {
        node_id: from,
        cost: 0.0,
    });

    while let Some(DijkstraNode { node_id, cost }) = heap.pop() {
        if node_id == to {
            break;
        }

        // Skip if we already found a shorter path.
        if let Some(&best) = dist.get(&node_id) {
            if cost > best {
                continue;
            }
        }

        for seg in index.outgoing_segments(&node_id) {
            let next_node = index.opposite_node(seg, &node_id);
            let edge_cost = cost_fn(seg);
            let new_cost = cost + edge_cost;

            let current_best = dist.get(&next_node).copied().unwrap_or(f64::INFINITY);
            if new_cost < current_best {
                dist.insert(next_node, new_cost);
                prev.insert(next_node, (node_id, seg.id));
                heap.push(DijkstraNode {
                    node_id: next_node,
                    cost: new_cost,
                });
            }
        }
    }

    // Reconstruct path.
    if !dist.contains_key(&to) {
        return None;
    }

    let mut nodes = vec![to];
    let mut segments = Vec::new();
    let mut current = to;

    while current != from {
        if let Some(&(prev_node, seg_id)) = prev.get(&current) {
            nodes.push(prev_node);
            segments.push(seg_id);
            current = prev_node;
        } else {
            return None; // no path
        }
    }

    nodes.reverse();
    segments.reverse();

    Some(ShortestPath {
        nodes,
        segments,
        total_cost: dist[&to],
    })
}

/// Standard cost functions.
pub mod cost {
    use aurora_core::map::RoadSegment;

    /// Cost by distance (metres).
    pub fn by_distance(seg: &RoadSegment) -> f64 {
        seg.length_m
    }

    /// Cost by travel time (seconds). Falls back to distance / default speed.
    pub fn by_time(seg: &RoadSegment) -> f64 {
        seg.travel_time_s.unwrap_or_else(|| {
            let speed_mps = seg.speed_limit_kmh.unwrap_or(50.0) / 3.6;
            seg.length_m / speed_mps
        })
    }

    /// Cost combining time with a penalty for toll roads.
    pub fn by_time_no_tolls(seg: &RoadSegment) -> f64 {
        let base = by_time(seg);
        if seg.toll {
            base * 10.0 // heavy penalty
        } else {
            base
        }
    }

    /// Cost that avoids tunnels (for certain vehicle types).
    pub fn by_time_avoid_tunnels(seg: &RoadSegment) -> f64 {
        let base = by_time(seg);
        if seg.tunnel {
            base * 5.0
        } else {
            base
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::map::*;
    use aurora_core::types::GeoPosition;
    use chrono::Utc;

    fn make_linear_graph() -> (RoadGraph, Vec<EntityId>) {
        // A → B → C linear graph.
        let n_a = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.08,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n_b = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.085,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n_c = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.09,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };

        let seg_ab = RoadSegment {
            id: EntityId::new(),
            from_node: n_a.id,
            to_node: n_b.id,
            geometry: vec![n_a.position, n_b.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m: 555.0,
            travel_time_s: Some(40.0),
        };
        let seg_bc = RoadSegment {
            id: EntityId::new(),
            from_node: n_b.id,
            to_node: n_c.id,
            geometry: vec![n_b.position, n_c.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m: 555.0,
            travel_time_s: Some(40.0),
        };

        let node_ids = vec![n_a.id, n_b.id, n_c.id];
        let graph = RoadGraph {
            id: EntityId::new(),
            region: "test".into(),
            nodes: vec![n_a, n_b, n_c],
            segments: vec![seg_ab, seg_bc],
            version: 1,
            updated_at: Utc::now(),
        };
        (graph, node_ids)
    }

    #[test]
    fn shortest_path_a_to_c() {
        let (graph, node_ids) = make_linear_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn: CostFn = Box::new(cost::by_distance);

        let result = shortest_path(&index, node_ids[0], node_ids[2], &cost_fn);
        assert!(result.is_some());
        let path = result.unwrap();
        assert_eq!(path.nodes.len(), 3);
        assert_eq!(path.segments.len(), 2);
        assert!((path.total_cost - 1110.0).abs() < 1.0);
    }

    #[test]
    fn shortest_path_c_to_a_bidirectional() {
        let (graph, node_ids) = make_linear_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn: CostFn = Box::new(cost::by_time);

        let result = shortest_path(&index, node_ids[2], node_ids[0], &cost_fn);
        assert!(result.is_some());
        let path = result.unwrap();
        assert!((path.total_cost - 80.0).abs() < 1.0);
    }

    #[test]
    fn no_path_to_disconnected_node() {
        let (graph, _) = make_linear_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn: CostFn = Box::new(cost::by_distance);
        let fake = EntityId::new();
        let result = shortest_path(&index, graph.nodes[0].id, fake, &cost_fn);
        assert!(result.is_none());
    }
}
