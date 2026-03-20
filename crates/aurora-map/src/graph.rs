//! Road graph spatial index — fast nearest-segment and nearest-node lookups.

use aurora_core::map::{RoadGraph, RoadNode, RoadSegment};
use aurora_core::types::{EntityId, GeoPosition};
use std::collections::HashMap;
use tracing::debug;

/// Spatial index over a road graph for fast position-based lookups.
pub struct RoadGraphIndex {
    /// All nodes keyed by ID.
    nodes: HashMap<EntityId, RoadNode>,
    /// All segments keyed by ID.
    segments: HashMap<EntityId, RoadSegment>,
    /// Adjacency list: node → outgoing segments.
    adjacency: HashMap<EntityId, Vec<EntityId>>,
    /// Reverse adjacency: node → incoming segments.
    reverse_adjacency: HashMap<EntityId, Vec<EntityId>>,
    /// Grid cell size in degrees for spatial binning.
    cell_size_deg: f64,
    /// Spatial grid: (cell_x, cell_y) → segment IDs whose geometry intersects the cell.
    grid: HashMap<(i32, i32), Vec<EntityId>>,
}

impl RoadGraphIndex {
    /// Build a spatial index from a `RoadGraph`.
    pub fn from_graph(graph: &RoadGraph) -> Self {
        let cell_size_deg = 0.001; // ~111m at equator

        let mut nodes = HashMap::new();
        let mut segments = HashMap::new();
        let mut adjacency: HashMap<EntityId, Vec<EntityId>> = HashMap::new();
        let mut reverse_adjacency: HashMap<EntityId, Vec<EntityId>> = HashMap::new();
        let mut grid: HashMap<(i32, i32), Vec<EntityId>> = HashMap::new();

        for node in &graph.nodes {
            nodes.insert(node.id, node.clone());
        }

        for seg in &graph.segments {
            segments.insert(seg.id, seg.clone());
            adjacency.entry(seg.from_node).or_default().push(seg.id);
            reverse_adjacency
                .entry(seg.to_node)
                .or_default()
                .push(seg.id);

            // If not one-way, add reverse adjacency.
            if !seg.one_way {
                adjacency.entry(seg.to_node).or_default().push(seg.id);
                reverse_adjacency
                    .entry(seg.from_node)
                    .or_default()
                    .push(seg.id);
            }

            // Insert into spatial grid.
            for point in &seg.geometry {
                let cell = Self::position_to_cell(point, cell_size_deg);
                grid.entry(cell).or_default().push(seg.id);
            }
        }

        debug!(
            nodes = nodes.len(),
            segments = segments.len(),
            grid_cells = grid.len(),
            "road graph index built"
        );

        Self {
            nodes,
            segments,
            adjacency,
            reverse_adjacency,
            cell_size_deg,
            grid,
        }
    }

    /// Find the nearest road segment to a position.
    /// Returns (segment_id, projected_position, distance_m).
    pub fn nearest_segment(&self, pos: &GeoPosition) -> Option<(EntityId, GeoPosition, f64)> {
        let cell = Self::position_to_cell(pos, self.cell_size_deg);

        // Search in the cell and its 8 neighbours.
        let mut best: Option<(EntityId, GeoPosition, f64)> = None;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let search_cell = (cell.0 + dx, cell.1 + dy);
                if let Some(seg_ids) = self.grid.get(&search_cell) {
                    for seg_id in seg_ids {
                        if let Some(seg) = self.segments.get(seg_id) {
                            let (proj, dist) = Self::project_onto_segment(pos, seg);
                            if best.is_none() || dist < best.as_ref().unwrap().2 {
                                best = Some((*seg_id, proj, dist));
                            }
                        }
                    }
                }
            }
        }

        best
    }

    /// Find the nearest node to a position.
    pub fn nearest_node(&self, pos: &GeoPosition) -> Option<(EntityId, f64)> {
        let mut best: Option<(EntityId, f64)> = None;
        for node in self.nodes.values() {
            let dist = haversine_m(pos, &node.position);
            if best.is_none() || dist < best.as_ref().unwrap().1 {
                best = Some((node.id, dist));
            }
        }
        best
    }

    /// Get outgoing segments from a node.
    pub fn outgoing_segments(&self, node_id: &EntityId) -> Vec<&RoadSegment> {
        self.adjacency
            .get(node_id)
            .map(|ids| ids.iter().filter_map(|id| self.segments.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get incoming segments to a node.
    pub fn incoming_segments(&self, node_id: &EntityId) -> Vec<&RoadSegment> {
        self.reverse_adjacency
            .get(node_id)
            .map(|ids| ids.iter().filter_map(|id| self.segments.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get a segment by ID.
    pub fn segment(&self, id: &EntityId) -> Option<&RoadSegment> {
        self.segments.get(id)
    }

    /// Get a node by ID.
    pub fn node(&self, id: &EntityId) -> Option<&RoadNode> {
        self.nodes.get(id)
    }

    /// Get the opposite node of a segment given one endpoint.
    pub fn opposite_node(&self, seg: &RoadSegment, from: &EntityId) -> EntityId {
        if seg.from_node == *from {
            seg.to_node
        } else {
            seg.from_node
        }
    }

    /// Total number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Total number of segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// All node IDs.
    pub fn node_ids(&self) -> Vec<EntityId> {
        self.nodes.keys().copied().collect()
    }

    // -- internal ---------------------------------------------------------------

    fn position_to_cell(pos: &GeoPosition, cell_size: f64) -> (i32, i32) {
        let cx = (pos.longitude_deg / cell_size).floor() as i32;
        let cy = (pos.latitude_deg / cell_size).floor() as i32;
        (cx, cy)
    }

    /// Project a point onto the nearest point on a segment's geometry polyline.
    fn project_onto_segment(pos: &GeoPosition, seg: &RoadSegment) -> (GeoPosition, f64) {
        if seg.geometry.is_empty() {
            let midpoint = GeoPosition {
                latitude_deg: 0.0,
                longitude_deg: 0.0,
                altitude_m: None,
            };
            return (midpoint, f64::MAX);
        }

        if seg.geometry.len() == 1 {
            let dist = haversine_m(pos, &seg.geometry[0]);
            return (seg.geometry[0], dist);
        }

        let mut best_proj = seg.geometry[0];
        let mut best_dist = haversine_m(pos, &seg.geometry[0]);

        for window in seg.geometry.windows(2) {
            let (proj, dist) = project_onto_line(pos, &window[0], &window[1]);
            if dist < best_dist {
                best_dist = dist;
                best_proj = proj;
            }
        }

        (best_proj, best_dist)
    }
}

/// Project a point onto a line segment defined by two points.
/// Returns (projected_point, distance_m).
fn project_onto_line(p: &GeoPosition, a: &GeoPosition, b: &GeoPosition) -> (GeoPosition, f64) {
    let dx = b.longitude_deg - a.longitude_deg;
    let dy = b.latitude_deg - a.latitude_deg;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-15 {
        return (*a, haversine_m(p, a));
    }

    let t = ((p.longitude_deg - a.longitude_deg) * dx + (p.latitude_deg - a.latitude_deg) * dy)
        / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj = GeoPosition {
        latitude_deg: a.latitude_deg + t * dy,
        longitude_deg: a.longitude_deg + t * dx,
        altitude_m: None,
    };

    (proj, haversine_m(p, &proj))
}

/// Haversine distance in metres between two geo positions.
pub fn haversine_m(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let dlat = (b.latitude_deg - a.latitude_deg).to_radians();
    let dlon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();

    let a_val = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().asin();
    r * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::map::*;
    use chrono::Utc;

    fn make_graph() -> RoadGraph {
        let n1 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.0853,
                longitude_deg: 34.7818,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let n2 = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.0863,
                longitude_deg: 34.7818,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        let seg = RoadSegment {
            id: EntityId::new(),
            from_node: n1.id,
            to_node: n2.id,
            geometry: vec![n1.position, n2.position],
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
            length_m: 111.0,
            travel_time_s: Some(8.0),
        };
        RoadGraph {
            id: EntityId::new(),
            region: "tel-aviv".into(),
            nodes: vec![n1, n2],
            segments: vec![seg],
            version: 1,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn index_builds_from_graph() {
        let graph = make_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        assert_eq!(index.node_count(), 2);
        assert_eq!(index.segment_count(), 1);
    }

    #[test]
    fn nearest_segment_finds_road() {
        let graph = make_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        let pos = GeoPosition {
            latitude_deg: 32.0858,
            longitude_deg: 34.7819,
            altitude_m: None,
        };
        let result = index.nearest_segment(&pos);
        assert!(result.is_some());
        let (_, _, dist) = result.unwrap();
        assert!(dist < 50.0, "expected within 50m, got {dist}m");
    }

    #[test]
    fn haversine_known_distance() {
        let a = GeoPosition {
            latitude_deg: 32.0853,
            longitude_deg: 34.7818,
            altitude_m: None,
        };
        let b = GeoPosition {
            latitude_deg: 32.0943,
            longitude_deg: 34.7818,
            altitude_m: None,
        };
        let dist = haversine_m(&a, &b);
        assert!((dist - 1000.0).abs() < 50.0, "expected ~1000m, got {dist}m");
    }

    #[test]
    fn adjacency_for_bidirectional_road() {
        let graph = make_graph();
        let index = RoadGraphIndex::from_graph(&graph);
        let n1 = graph.nodes[0].id;
        let n2 = graph.nodes[1].id;
        assert!(!index.outgoing_segments(&n1).is_empty());
        assert!(!index.outgoing_segments(&n2).is_empty()); // bidirectional
    }
}
