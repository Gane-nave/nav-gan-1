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
    let from_id = nearest_node(graph, from.0, from.1)?;
    let to_id = nearest_node(graph, to.0, to.1)?;
    let constrained = envelope.is_some();
    let cost_fn: CostFn = match envelope {
        Some(env) => by_time_for_vehicle(env),
        None => Box::new(cost::by_time),
    };
    let path =
        shortest_path(index, from_id, to_id, &cost_fn).filter(|p| p.total_cost.is_finite())?;

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

    #[test]
    fn vehicle_presets_exist() {
        for v in ["car", "truck", "van", "bus", "emergency"] {
            assert!(envelope_by_name(v).is_some(), "{v}");
        }
        assert!(envelope_by_name("spaceship").is_none());
    }
}
