//! End-to-end acceptance: Overpass JSON → RoadGraph → vehicle-aware routing.
//!
//! A miniature road network with a height-limited direct road and an
//! unrestricted detour — imported from real Overpass-shaped JSON — must send
//! a heavy truck around the bridge while a car drives straight through.

use aurora_core::vehicle::VehicleEnvelope;
use aurora_map::graph::RoadGraphIndex;
use aurora_routing::dijkstra::shortest_path;
use aurora_routing::vehicle_aware::by_time_for_vehicle;
use gane_osm_import::graph_from_overpass_json;

/// Direct road 1→2 (fast, maxheight 4.0 m) vs detour 1→3→2 (slower, free).
const OVERPASS_FIXTURE: &str = r#"{
  "version": 0.6,
  "elements": [
    {"type":"node","id":1,"lat":32.0800,"lon":34.7800},
    {"type":"node","id":2,"lat":32.0900,"lon":34.7800},
    {"type":"node","id":3,"lat":32.0850,"lon":34.7900},
    {"type":"way","id":100,"nodes":[1,2],
     "tags":{"highway":"primary","maxspeed":"80","maxheight":"4","bridge":"yes"}},
    {"type":"way","id":101,"nodes":[1,3],
     "tags":{"highway":"secondary","maxspeed":"50"}},
    {"type":"way","id":102,"nodes":[3,2],
     "tags":{"highway":"secondary","maxspeed":"50"}}
  ]
}"#;

fn endpoints(
    graph: &aurora_core::map::RoadGraph,
) -> (aurora_core::types::EntityId, aurora_core::types::EntityId) {
    // Nodes 1 and 2 are the only ones at lon 34.78.
    let a = graph
        .nodes
        .iter()
        .find(|n| (n.position.latitude_deg - 32.08).abs() < 1e-9)
        .unwrap()
        .id;
    let b = graph
        .nodes
        .iter()
        .find(|n| (n.position.latitude_deg - 32.09).abs() < 1e-9)
        .unwrap()
        .id;
    (a, b)
}

#[test]
fn car_uses_the_direct_bridge() {
    let graph = graph_from_overpass_json("fixture", OVERPASS_FIXTURE).unwrap();
    assert_eq!(graph.segments.len(), 3);
    let index = RoadGraphIndex::from_graph(&graph);
    let (a, b) = endpoints(&graph);
    let path = shortest_path(&index, a, b, &by_time_for_vehicle(VehicleEnvelope::car()))
        .expect("car route exists");
    assert_eq!(path.nodes.len(), 2, "car goes direct: {:?}", path.nodes);
}

#[test]
fn truck_detours_on_imported_osm_data() {
    let graph = graph_from_overpass_json("fixture", OVERPASS_FIXTURE).unwrap();
    let index = RoadGraphIndex::from_graph(&graph);
    let (a, b) = endpoints(&graph);
    let path = shortest_path(
        &index,
        a,
        b,
        &by_time_for_vehicle(VehicleEnvelope::heavy_truck()),
    )
    .expect("truck route exists");
    assert_eq!(
        path.nodes.len(),
        3,
        "truck must take the detour via node 3: {:?}",
        path.nodes
    );
    // And the chosen route must not include the height-limited bridge.
    let bridge_seg = graph
        .segments
        .iter()
        .find(|s| s.height_limit_m == Some(4.0))
        .unwrap()
        .id;
    assert!(!path.segments.contains(&bridge_seg));
}
