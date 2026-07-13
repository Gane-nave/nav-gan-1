//! Geographic routing on an imported graph: coordinate snap + vehicle-aware
//! shortest path. This is the same call path the product uses — CLI and WASM
//! are thin shells over it.

use gane_core::map::RoadGraph;
use gane_core::types::EntityId;
use gane_core::vehicle::VehicleEnvelope;
use gane_map::graph::RoadGraphIndex;
use gane_routing::dijkstra::{cost, shortest_path, CostFn};
use gane_routing::vehicle_aware::by_time_for_vehicle;
use serde::Serialize;

use crate::builder::haversine_m;

/// A computed route with product-facing fields.
#[derive(Debug, Serialize)]
pub struct GeoRoute {
    pub from_node: String,
    pub to_node: String,
    pub node_count: usize,
    pub segment_ids: Vec<String>,
    pub total_time_s: f64,
    pub total_length_m: f64,
    pub constrained: bool,
    /// Route geometry as (lat, lon) pairs for rendering.
    pub polyline: Vec<(f64, f64)>,
}

/// Snap a coordinate to the nearest graph node.
pub fn nearest_node(graph: &RoadGraph, lat: f64, lon: f64) -> Option<EntityId> {
    let probe = gane_core::types::GeoPosition {
        latitude_deg: lat,
        longitude_deg: lon,
        altitude_m: None,
    };
    graph
        .nodes
        .iter()
        .min_by(|a, b| {
            haversine_m(&a.position, &probe)
                .partial_cmp(&haversine_m(&b.position, &probe))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|n| n.id)
}

/// Snap a coordinate to the nearest node the vehicle can actually use — one
/// with at least one incident segment the envelope permits. Plain nearest-node
/// snapping strands motor vehicles on footway/cycleway nodes: on real OSM data
/// nearly half the network can be non-drivable, and a car queried from a park
/// bench would get "no legal route" with a road 30 m away.
pub fn nearest_usable_node(
    graph: &RoadGraph,
    index: &RoadGraphIndex,
    lat: f64,
    lon: f64,
    envelope: &VehicleEnvelope,
) -> Option<EntityId> {
    usable_candidates(graph, index, lat, lon, envelope, 1)
        .into_iter()
        .next()
        .map(|(id, _)| id)
}

/// The `k` nearest envelope-usable nodes with their snap distance in meters,
/// nearest first. Multiple candidates matter because the single nearest
/// usable node can sit in a car-disconnected island (a cul-de-sac cluster
/// linked to the network only by a cycleway); the router falls back through
/// candidates until a pair actually connects.
pub fn usable_candidates(
    graph: &RoadGraph,
    index: &RoadGraphIndex,
    lat: f64,
    lon: f64,
    envelope: &VehicleEnvelope,
    k: usize,
) -> Vec<(EntityId, f64)> {
    let probe = gane_core::types::GeoPosition {
        latitude_deg: lat,
        longitude_deg: lon,
        altitude_m: None,
    };
    let mut candidates: Vec<(EntityId, f64)> = graph
        .nodes
        .iter()
        .filter(|n| {
            index
                .outgoing_segments(&n.id)
                .into_iter()
                .chain(index.incoming_segments(&n.id))
                .any(|seg| envelope.permits(seg).is_ok())
        })
        .map(|n| (n.id, haversine_m(&n.position, &probe)))
        .collect();
    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    candidates.truncate(k);
    candidates
}

/// Route between two coordinates, optionally constrained by a vehicle envelope.
///
/// Returns `None` when no legal route exists — never an illegal one.
pub fn route_geo(
    graph: &RoadGraph,
    index: &RoadGraphIndex,
    from: (f64, f64),
    to: (f64, f64),
    envelope: Option<VehicleEnvelope>,
) -> Option<GeoRoute> {
    /// Snap candidates per endpoint. Must be generous: near an extract
    /// boundary the request point's whole neighborhood can be clipped into
    /// disconnected islands, and permissive classes see even deeper ranks
    /// because footway nodes crowd the shortlist (observed: main network at
    /// rank 27 for car, rank 57 for emergency, on the same real extract).
    const SNAP_CANDIDATES: usize = 64;
    /// A snap farther than this is a wrong answer, not a fallback.
    const MAX_SNAP_DISTANCE_M: f64 = 1_500.0;
    /// Bound on pair attempts — the full candidate product. Failed searches
    /// only explore the stranded island they start in, so even exhausting
    /// every pair stays fast; a tighter cap was observed to cut off the
    /// first connectable pair on real clipped extracts.
    const MAX_PAIR_ATTEMPTS: usize = SNAP_CANDIDATES * SNAP_CANDIDATES;

    type Candidates = Vec<(EntityId, f64)>;
    let constrained = envelope.is_some();
    let (cost_fn, from_cands, to_cands): (CostFn, Candidates, Candidates) = match envelope {
        Some(env) => (
            by_time_for_vehicle(env.clone()),
            usable_candidates(graph, index, from.0, from.1, &env, SNAP_CANDIDATES),
            usable_candidates(graph, index, to.0, to.1, &env, SNAP_CANDIDATES),
        ),
        None => (
            Box::new(cost::by_time),
            nearest_node(graph, from.0, from.1)
                .map(|id| vec![(id, 0.0)])
                .unwrap_or_default(),
            nearest_node(graph, to.0, to.1)
                .map(|id| vec![(id, 0.0)])
                .unwrap_or_default(),
        ),
    };

    // Try candidate pairs in order of combined snap distance so the route
    // still starts as close to the request as the network allows.
    let mut pairs: Vec<(EntityId, EntityId, f64)> = from_cands
        .iter()
        .filter(|(_, d)| *d <= MAX_SNAP_DISTANCE_M)
        .flat_map(|(f, df)| {
            to_cands
                .iter()
                .filter(|(_, d)| *d <= MAX_SNAP_DISTANCE_M)
                .map(move |(t, dt)| (*f, *t, df + dt))
        })
        .collect();
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    pairs.truncate(MAX_PAIR_ATTEMPTS);

    let (from_id, to_id, path) = pairs.into_iter().find_map(|(f, t, _)| {
        shortest_path(index, f, t, &cost_fn)
            .filter(|p| p.total_cost.is_finite())
            .map(|p| (f, t, p))
    })?;

    let mut total_length_m = 0.0;
    let mut polyline: Vec<(f64, f64)> = Vec::new();
    for seg_id in &path.segments {
        if let Some(seg) = graph.segments.iter().find(|s| s.id == *seg_id) {
            total_length_m += seg.length_m;
            for g in &seg.geometry {
                polyline.push((g.latitude_deg, g.longitude_deg));
            }
        }
    }

    Some(GeoRoute {
        from_node: from_id.to_string(),
        to_node: to_id.to_string(),
        node_count: path.nodes.len(),
        segment_ids: path.segments.iter().map(|s| s.to_string()).collect(),
        total_time_s: path.total_cost,
        total_length_m,
        constrained,
        polyline,
    })
}

/// Named vehicle presets for the CLI (`--vehicle` values).
pub fn envelope_by_name(name: &str) -> Option<VehicleEnvelope> {
    use gane_core::vehicle::VehicleClass;
    let mut env = match name {
        "car" => VehicleEnvelope::car(),
        "truck" | "heavy_truck" => VehicleEnvelope::heavy_truck(),
        "van" => {
            let mut e = VehicleEnvelope::car();
            e.class = VehicleClass::Van;
            e.height_m = 2.5;
            e.weight_kg = 3_200.0;
            e
        }
        "bus" => {
            let mut e = VehicleEnvelope::heavy_truck();
            e.class = VehicleClass::Bus;
            e.height_m = 3.4;
            e.weight_kg = 18_000.0;
            e.axle_count = 3;
            e
        }
        "emergency" => {
            let mut e = VehicleEnvelope::car();
            e.class = VehicleClass::Emergency;
            e.height_m = 2.8;
            e.weight_kg = 4_500.0;
            e
        }
        _ => return None,
    };
    env.hazmat = false;
    Some(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_from_overpass_json;

    const FIXTURE: &str = r#"{"elements":[
      {"type":"node","id":1,"lat":32.0800,"lon":34.7800},
      {"type":"node","id":2,"lat":32.0900,"lon":34.7800},
      {"type":"node","id":3,"lat":32.0850,"lon":34.7900},
      {"type":"way","id":100,"nodes":[1,2],"tags":{"highway":"primary","maxspeed":"80","maxheight":"4"}},
      {"type":"way","id":101,"nodes":[1,3],"tags":{"highway":"secondary","maxspeed":"50"}},
      {"type":"way","id":102,"nodes":[3,2],"tags":{"highway":"secondary","maxspeed":"50"}}
    ]}"#;

    #[test]
    fn routes_between_coordinates_with_vehicle_constraints() {
        let graph = graph_from_overpass_json("t", FIXTURE).unwrap();
        let index = RoadGraphIndex::from_graph(&graph);

        let car = route_geo(&graph, &index, (32.0801, 34.7801), (32.0899, 34.7799), None)
            .expect("car route");
        assert_eq!(car.node_count, 2, "car direct");

        let truck = route_geo(
            &graph,
            &index,
            (32.0801, 34.7801),
            (32.0899, 34.7799),
            envelope_by_name("truck"),
        )
        .expect("truck route");
        assert_eq!(truck.node_count, 3, "truck detours");
        assert!(truck.total_time_s > car.total_time_s);
        assert!(truck.total_length_m > car.total_length_m);
        assert!(!truck.polyline.is_empty());
    }

    #[test]
    fn snaps_to_nearest_node() {
        let graph = graph_from_overpass_json("t", FIXTURE).unwrap();
        let id = nearest_node(&graph, 32.0801, 34.7801).unwrap();
        let n = graph.nodes.iter().find(|n| n.id == id).unwrap();
        assert!((n.position.latitude_deg - 32.08).abs() < 1e-9);
    }

    /// A footway spur whose tip sits right next to the query point: raw
    /// nearest-node snapping lands on it and strands the car; usable-node
    /// snapping must skip to the road network and route successfully.
    const FOOTWAY_SPUR_FIXTURE: &str = r#"{"elements":[
      {"type":"node","id":1,"lat":32.0800,"lon":34.7800},
      {"type":"node","id":2,"lat":32.0900,"lon":34.7800},
      {"type":"node","id":4,"lat":32.0803,"lon":34.7802},
      {"type":"node","id":5,"lat":32.0806,"lon":34.7804},
      {"type":"way","id":100,"nodes":[1,2],"tags":{"highway":"primary","maxspeed":"80"}},
      {"type":"way","id":103,"nodes":[4,5],"tags":{"highway":"footway"}}
    ]}"#;

    #[test]
    fn car_snap_skips_footway_only_nodes() {
        let graph = graph_from_overpass_json("t", FOOTWAY_SPUR_FIXTURE).unwrap();
        let index = RoadGraphIndex::from_graph(&graph);
        let probe = (32.0804, 34.7803); // nearest raw node is the footway tip

        let raw = nearest_node(&graph, probe.0, probe.1).unwrap();
        let raw_node = graph.nodes.iter().find(|n| n.id == raw).unwrap();
        assert!(
            (raw_node.position.latitude_deg - 32.0803).abs() < 1e-6
                || (raw_node.position.latitude_deg - 32.0806).abs() < 1e-6,
            "precondition: raw snap lands on the footway spur"
        );

        let car = envelope_by_name("car").unwrap();
        let usable = nearest_usable_node(&graph, &index, probe.0, probe.1, &car).unwrap();
        let usable_node = graph.nodes.iter().find(|n| n.id == usable).unwrap();
        assert!(
            (usable_node.position.latitude_deg - 32.0800).abs() < 1e-6,
            "car snap must land on the road, not the footway"
        );

        let route = route_geo(
            &graph,
            &index,
            probe,
            (32.0899, 34.7799),
            envelope_by_name("car"),
        )
        .expect("car routes from beside the footway");
        assert_eq!(route.node_count, 2);
    }

    #[test]
    fn vehicle_presets_exist() {
        for v in ["car", "truck", "van", "bus", "emergency"] {
            assert!(envelope_by_name(v).is_some(), "{v}");
        }
        assert!(envelope_by_name("spaceship").is_none());
    }
}
