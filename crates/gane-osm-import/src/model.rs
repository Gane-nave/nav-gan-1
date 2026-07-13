//! Raw OSM primitives as consumed by the graph builder.

use std::collections::HashMap;

/// An OSM node: position only (tags are irrelevant for routing geometry).
#[derive(Debug, Clone)]
pub struct RawNode {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
}

/// An OSM way with its ordered node references and tag map.
#[derive(Debug, Clone)]
pub struct RawWay {
    pub id: i64,
    pub node_refs: Vec<i64>,
    pub tags: HashMap<String, String>,
}
