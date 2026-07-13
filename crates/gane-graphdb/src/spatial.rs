//! Spatial index — grid-based spatial partitioning for fast geo-queries.

use std::collections::HashMap;

use crate::graph::NodeId;

/// Grid cell coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellCoord {
    x: i64,
    y: i64,
}

/// A bounding box in geographic coordinates.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    /// Create a new bounding box.
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }

    /// Check if a point is inside this bounding box.
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        (self.min_lat..=self.max_lat).contains(&lat) && (self.min_lon..=self.max_lon).contains(&lon)
    }

    /// Check if this bounding box intersects another.
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }

    /// Area in approximate square degrees.
    pub fn area(&self) -> f64 {
        (self.max_lat - self.min_lat) * (self.max_lon - self.min_lon)
    }
}

/// A point with associated node ID.
#[derive(Debug, Clone, Copy)]
struct SpatialEntry {
    node_id: NodeId,
    lat: f64,
    lon: f64,
}

/// Grid-based spatial index for fast proximity queries.
pub struct SpatialIndex {
    /// Grid cell size in degrees.
    cell_size: f64,
    /// Grid cells mapping to contained entries.
    cells: HashMap<CellCoord, Vec<SpatialEntry>>,
    /// Total indexed points.
    count: usize,
}

impl SpatialIndex {
    /// Create a new spatial index with a given cell size (in degrees).
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            count: 0,
        }
    }

    /// Insert a point into the index.
    pub fn insert(&mut self, node_id: NodeId, lat: f64, lon: f64) {
        let cell = self.cell_for(lat, lon);
        self.cells
            .entry(cell)
            .or_default()
            .push(SpatialEntry { node_id, lat, lon });
        self.count += 1;
    }

    /// Find all nodes within a bounding box.
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<NodeId> {
        let min_cell = self.cell_for(bbox.min_lat, bbox.min_lon);
        let max_cell = self.cell_for(bbox.max_lat, bbox.max_lon);

        let mut results = Vec::new();
        for x in min_cell.x..=max_cell.x {
            for y in min_cell.y..=max_cell.y {
                let coord = CellCoord { x, y };
                if let Some(entries) = self.cells.get(&coord) {
                    for entry in entries {
                        if bbox.contains(entry.lat, entry.lon) {
                            results.push(entry.node_id);
                        }
                    }
                }
            }
        }
        results
    }

    /// Find the nearest node to a given point.
    pub fn nearest(&self, lat: f64, lon: f64) -> Option<(NodeId, f64)> {
        // Search expanding rings of cells
        let center = self.cell_for(lat, lon);
        let mut best: Option<(NodeId, f64)> = None;

        for radius in 0..=self.max_search_radius() {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    if dx.abs() != radius && dy.abs() != radius {
                        continue; // Only check the ring perimeter
                    }
                    let coord = CellCoord {
                        x: center.x + dx,
                        y: center.y + dy,
                    };
                    if let Some(entries) = self.cells.get(&coord) {
                        for entry in entries {
                            let dist = haversine_distance(lat, lon, entry.lat, entry.lon);
                            if best.is_none() || dist < best.unwrap().1 {
                                best = Some((entry.node_id, dist));
                            }
                        }
                    }
                }
            }
            // Only stop if the best distance found is less than the minimum
            // possible distance from the next ring of cells.
            if let Some((_, best_dist)) = best {
                let min_next_ring_dist = (radius as f64) * self.cell_size * 111_320.0 * 0.5;
                if best_dist <= min_next_ring_dist {
                    break;
                }
            }
        }
        best
    }

    /// Find all nodes within a radius (meters) of a point.
    pub fn query_radius(&self, lat: f64, lon: f64, radius_m: f64) -> Vec<(NodeId, f64)> {
        // Convert radius to approximate degree span
        let degree_span = radius_m / 111_320.0;
        let bbox = BoundingBox::new(
            lat - degree_span,
            lon - degree_span,
            lat + degree_span,
            lon + degree_span,
        );
        let candidates = self.query_bbox(&bbox);
        let mut results = Vec::new();
        for node_id in candidates {
            // Re-lookup the actual position from the grid
            if let Some(dist) = self.distance_to(node_id, lat, lon) {
                if dist <= radius_m {
                    results.push((node_id, dist));
                }
            }
        }
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Number of indexed points.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Number of occupied cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.cells.clear();
        self.count = 0;
    }

    fn cell_for(&self, lat: f64, lon: f64) -> CellCoord {
        CellCoord {
            x: (lat / self.cell_size).floor() as i64,
            y: (lon / self.cell_size).floor() as i64,
        }
    }

    fn max_search_radius(&self) -> i64 {
        // Limit search to a reasonable radius
        let max_cells = (self.cells.len() as f64).sqrt() as i64;
        max_cells.clamp(3, 50)
    }

    fn distance_to(&self, node_id: NodeId, lat: f64, lon: f64) -> Option<f64> {
        for entries in self.cells.values() {
            for entry in entries {
                if entry.node_id == node_id {
                    return Some(haversine_distance(lat, lon, entry.lat, entry.lon));
                }
            }
        }
        None
    }
}

/// Haversine distance between two geographic points in meters.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6_371_000.0; // Earth radius in meters
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_query_bbox() {
        let mut idx = SpatialIndex::new(0.01);
        idx.insert(1, 32.0, 34.0);
        idx.insert(2, 32.05, 34.05);
        idx.insert(3, 33.0, 35.0);

        let bbox = BoundingBox::new(31.9, 33.9, 32.1, 34.1);
        let results = idx.query_bbox(&bbox);
        assert!(results.contains(&1));
        assert!(!results.contains(&3));
    }

    #[test]
    fn test_nearest() {
        let mut idx = SpatialIndex::new(0.01);
        idx.insert(1, 32.0, 34.0);
        idx.insert(2, 32.1, 34.1);
        idx.insert(3, 32.5, 34.5);

        let (nearest_id, _dist) = idx.nearest(32.01, 34.01).unwrap();
        assert_eq!(nearest_id, 1);
    }

    #[test]
    fn test_query_radius() {
        let mut idx = SpatialIndex::new(0.01);
        idx.insert(1, 32.0, 34.0);
        idx.insert(2, 32.001, 34.001);
        idx.insert(3, 33.0, 35.0); // far away

        // 500m radius from (32.0, 34.0)
        let results = idx.query_radius(32.0, 34.0, 500.0);
        assert!(results.iter().any(|(id, _)| *id == 1));
        assert!(results.iter().any(|(id, _)| *id == 2));
        assert!(!results.iter().any(|(id, _)| *id == 3));
    }

    #[test]
    fn test_bbox_contains() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert!(bbox.contains(5.0, 5.0));
        assert!(!bbox.contains(11.0, 5.0));
    }

    #[test]
    fn test_bbox_intersects() {
        let a = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let b = BoundingBox::new(5.0, 5.0, 15.0, 15.0);
        let c = BoundingBox::new(20.0, 20.0, 30.0, 30.0);
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_haversine_distance() {
        // Tel Aviv to Jerusalem ~ 54 km
        let dist = haversine_distance(32.0853, 34.7818, 31.7683, 35.2137);
        assert!((dist - 54_000.0).abs() < 5000.0);
    }

    #[test]
    fn test_empty_index() {
        let idx = SpatialIndex::new(0.01);
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
        assert!(idx.nearest(0.0, 0.0).is_none());
    }

    #[test]
    fn test_clear() {
        let mut idx = SpatialIndex::new(0.01);
        idx.insert(1, 32.0, 34.0);
        assert_eq!(idx.len(), 1);
        idx.clear();
        assert!(idx.is_empty());
    }

    #[test]
    fn test_bbox_area() {
        let bbox = BoundingBox::new(0.0, 0.0, 1.0, 2.0);
        assert!((bbox.area() - 2.0).abs() < f64::EPSILON);
    }
}
