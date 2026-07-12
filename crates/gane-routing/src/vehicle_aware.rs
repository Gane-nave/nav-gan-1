//! Vehicle-aware routing — hard-constraint filtering by vehicle envelope.
//!
//! Legal/physical restrictions (height, weight, hazmat, road class) are
//! enforced as *edge filters*: a violating segment costs `f64::INFINITY`,
//! so Dijkstra never commits to it and an illegal route is never returned.
//! Soft preferences (tolls, tunnels, comfort) remain in the base cost.

use gane_core::map::RoadSegment;
use gane_core::vehicle::{RestrictionViolation, VehicleEnvelope};

use crate::dijkstra::CostFn;

/// Wrap a base cost function with hard vehicle-envelope constraints.
///
/// Segments the envelope does not permit become unreachable (infinite cost).
pub fn envelope_filtered(base: CostFn, envelope: VehicleEnvelope) -> CostFn {
    Box::new(move |seg: &RoadSegment| {
        if envelope.permits(seg).is_err() {
            f64::INFINITY
        } else {
            base(seg)
        }
    })
}

/// Convenience: fastest-time cost for a specific vehicle envelope.
pub fn by_time_for_vehicle(envelope: VehicleEnvelope) -> CostFn {
    envelope_filtered(Box::new(crate::dijkstra::cost::by_time), envelope)
}

/// Classify every segment of a path against an envelope.
///
/// Used by the route-explanation layer: an empty result proves the route is
/// legal for the vehicle; entries name each violating segment and why.
pub fn violations_on_path<'a>(
    segments: impl IntoIterator<Item = &'a RoadSegment>,
    envelope: &VehicleEnvelope,
) -> Vec<(gane_core::types::EntityId, RestrictionViolation)> {
    segments
        .into_iter()
        .filter_map(|seg| envelope.permits(seg).err().map(|v| (seg.id, v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dijkstra::{cost, shortest_path};
    use chrono::Utc;
    use gane_core::map::*;
    use gane_core::types::{EntityId, GeoPosition};
    use gane_core::vehicle::VehicleEnvelope;
    use gane_map::graph::RoadGraphIndex;

    fn node(lat: f64, lon: f64) -> RoadNode {
        RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: lat,
                longitude_deg: lon,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        }
    }

    fn segment(
        from: &RoadNode,
        to: &RoadNode,
        time_s: f64,
        height_limit: Option<f64>,
    ) -> RoadSegment {
        RoadSegment {
            id: EntityId::new(),
            from_node: from.id,
            to_node: to.id,
            geometry: vec![from.position, to.position],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: height_limit.is_some(),
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: height_limit,
            hazmat_restricted: false,
            length_m: time_s * 14.0,
            travel_time_s: Some(time_s),
        }
    }

    /// Diamond graph:  A → C direct (fast, 4.0 m height limit)
    ///                 A → B → C detour (slow, unrestricted)
    fn diamond() -> (RoadGraph, EntityId, EntityId, EntityId) {
        let a = node(32.080, 34.780);
        let b = node(32.085, 34.790);
        let c = node(32.090, 34.780);
        let direct = segment(&a, &c, 60.0, Some(4.0)); // low bridge
        let leg1 = segment(&a, &b, 90.0, None);
        let leg2 = segment(&b, &c, 90.0, None);
        let (a_id, b_id, c_id) = (a.id, b.id, c.id);
        let graph = RoadGraph {
            id: EntityId::new(),
            region: "test".into(),
            nodes: vec![a, b, c],
            segments: vec![direct, leg1, leg2],
            version: 1,
            updated_at: Utc::now(),
        };
        (graph, a_id, b_id, c_id)
    }

    #[test]
    fn car_takes_direct_route_under_bridge() {
        let (graph, a, _b, c) = diamond();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn = by_time_for_vehicle(VehicleEnvelope::car());
        let path = shortest_path(&index, a, c, &cost_fn).expect("car route");
        assert_eq!(path.nodes, vec![a, c], "car should go A→C directly");
        assert!((path.total_cost - 60.0).abs() < 1e-9);
    }

    #[test]
    fn truck_detours_around_low_bridge() {
        let (graph, a, b, c) = diamond();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn = by_time_for_vehicle(VehicleEnvelope::heavy_truck());
        let path = shortest_path(&index, a, c, &cost_fn).expect("truck route");
        assert_eq!(path.nodes, vec![a, b, c], "truck must detour via B");
        assert!((path.total_cost - 180.0).abs() < 1e-9);
    }

    #[test]
    fn truck_route_has_no_violations() {
        let (graph, a, _b, c) = diamond();
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn = by_time_for_vehicle(VehicleEnvelope::heavy_truck());
        let path = shortest_path(&index, a, c, &cost_fn).unwrap();
        let segs: Vec<&RoadSegment> = graph
            .segments
            .iter()
            .filter(|s| path.segments.contains(&s.id))
            .collect();
        let violations = violations_on_path(segs, &VehicleEnvelope::heavy_truck());
        assert!(
            violations.is_empty(),
            "returned route must be legal: {violations:?}"
        );
    }

    #[test]
    fn no_legal_route_returns_none() {
        // Only the restricted direct edge exists — truck has no legal path.
        let a = node(32.080, 34.780);
        let c = node(32.090, 34.780);
        let direct = segment(&a, &c, 60.0, Some(4.0));
        let (a_id, c_id) = (a.id, c.id);
        let graph = RoadGraph {
            id: EntityId::new(),
            region: "test".into(),
            nodes: vec![a, c],
            segments: vec![direct],
            version: 1,
            updated_at: Utc::now(),
        };
        let index = RoadGraphIndex::from_graph(&graph);
        let cost_fn = by_time_for_vehicle(VehicleEnvelope::heavy_truck());
        let path = shortest_path(&index, a_id, c_id, &cost_fn);
        assert!(
            path.is_none() || path.unwrap().total_cost.is_infinite(),
            "truck must never receive an illegal route"
        );
    }

    #[test]
    fn base_cost_preserved_for_permitted_segments() {
        let (graph, a, _b, c) = diamond();
        let index = RoadGraphIndex::from_graph(&graph);
        let plain = shortest_path(&index, a, c, &(Box::new(cost::by_time) as CostFn)).unwrap();
        let car =
            shortest_path(&index, a, c, &by_time_for_vehicle(VehicleEnvelope::car())).unwrap();
        assert_eq!(plain.total_cost, car.total_cost);
    }
}
