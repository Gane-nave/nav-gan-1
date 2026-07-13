//! Overpass API JSON parsing.
//!
//! Accepts the standard Overpass `[out:json]` response shape:
//! `{"elements":[{"type":"node",...},{"type":"way",...}]}` — obtainable for
//! any bounding box with a query like:
//! `[out:json]; way[highway](32.05,34.75,32.11,34.82); (._;>;); out body;`

use serde::Deserialize;
use std::collections::HashMap;

use crate::model::{RawNode, RawWay};

#[derive(Deserialize)]
struct OverpassResponse {
    elements: Vec<Element>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Element {
    #[serde(rename = "node")]
    Node { id: i64, lat: f64, lon: f64 },
    #[serde(rename = "way")]
    Way {
        id: i64,
        nodes: Vec<i64>,
        #[serde(default)]
        tags: HashMap<String, String>,
    },
    #[serde(other)]
    Other,
}

/// Parse an Overpass JSON document into raw nodes and ways.
pub fn parse_overpass_json(json: &str) -> Result<(Vec<RawNode>, Vec<RawWay>), serde_json::Error> {
    let resp: OverpassResponse = serde_json::from_str(json)?;
    let mut nodes = Vec::new();
    let mut ways = Vec::new();
    for el in resp.elements {
        match el {
            Element::Node { id, lat, lon } => nodes.push(RawNode { id, lat, lon }),
            Element::Way {
                id,
                nodes: refs,
                tags,
            } => ways.push(RawWay {
                id,
                node_refs: refs,
                tags,
            }),
            Element::Other => {}
        }
    }
    Ok((nodes, ways))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_overpass_shape() {
        let json = r#"{
          "version": 0.6,
          "elements": [
            {"type":"node","id":1,"lat":32.08,"lon":34.78},
            {"type":"node","id":2,"lat":32.081,"lon":34.78},
            {"type":"way","id":10,"nodes":[1,2],"tags":{"highway":"primary","maxheight":"4"}},
            {"type":"relation","id":99,"members":[]}
          ]
        }"#;
        let (nodes, ways) = parse_overpass_json(json).unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(ways.len(), 1);
        assert_eq!(ways[0].tags.get("maxheight").map(String::as_str), Some("4"));
    }
}
