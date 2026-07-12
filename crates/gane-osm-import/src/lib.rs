//! G.A.N.E NAV — OSM road-network importer.
//!
//! Pipeline: Overpass JSON → raw nodes/ways → classified, restriction-aware
//! `RoadGraph` ready for `aurora-routing` (native) or `gane-wasm::load_graph`
//! (browser). Vehicle restrictions (`maxheight`, `maxweight`, `hazmat`,
//! tunnels) are carried onto every segment so vehicle-envelope routing works
//! on real map data.

pub mod builder;
pub mod classify;
pub mod model;
pub mod overpass;

pub use builder::build_graph;
pub use overpass::parse_overpass_json;

/// One-call convenience: Overpass JSON → RoadGraph.
pub fn graph_from_overpass_json(
    region: &str,
    json: &str,
) -> Result<aurora_core::map::RoadGraph, serde_json::Error> {
    let (nodes, ways) = parse_overpass_json(json)?;
    Ok(build_graph(region, &nodes, &ways))
}
