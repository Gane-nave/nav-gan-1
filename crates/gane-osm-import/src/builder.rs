//! Road-graph builder: raw OSM nodes/ways → `gane_core::map::RoadGraph`.
//!
//! Ways are split at intersection nodes (nodes shared by two or more ways,
//! plus way endpoints) so every produced `RoadSegment` is a routable edge.

use chrono::Utc;
use gane_core::map::{RoadGraph, RoadNode, RoadNodeType, RoadSegment, SurfaceType};
use gane_core::types::{EntityId, GeoPosition};
use std::collections::HashMap;

use crate::classify::{
    default_speed_kmh, parse_length_m, parse_maxspeed_kmh, parse_oneway, parse_weight_kg,
    road_class, OneWay,
};
use crate::model::{RawNode, RawWay};

/// Great-circle distance in metres.
pub fn haversine_m(a: &GeoPosition, b: &GeoPosition) -> f64 {
    const R: f64 = 6_371_000.0;
    let (la1, la2) = (a.latitude_deg.to_radians(), b.latitude_deg.to_radians());
    let dla = (b.latitude_deg - a.latitude_deg).to_radians();
    let dlo = (b.longitude_deg - a.longitude_deg).to_radians();
    let h = (dla / 2.0).sin().powi(2) + la1.cos() * la2.cos() * (dlo / 2.0).sin().powi(2);
    2.0 * R * h.sqrt().asin()
}

/// Build a routable graph from raw OSM data for a named region.
pub fn build_graph(region: &str, nodes: &[RawNode], ways: &[RawWay]) -> RoadGraph {
    let node_pos: HashMap<i64, GeoPosition> = nodes
        .iter()
        .map(|n| {
            (
                n.id,
                GeoPosition {
                    latitude_deg: n.lat,
                    longitude_deg: n.lon,
                    altitude_m: None,
                },
            )
        })
        .collect();

    // Count how many routable ways reference each node.
    let mut usage: HashMap<i64, u32> = HashMap::new();
    let routable: Vec<&RawWay> = ways
        .iter()
        .filter(|w| {
            w.tags.get("highway").and_then(|h| road_class(h)).is_some() && w.node_refs.len() >= 2
        })
        .collect();
    for way in &routable {
        for id in &way.node_refs {
            *usage.entry(*id).or_insert(0) += 1;
        }
    }

    let mut graph_nodes: Vec<RoadNode> = Vec::new();
    let mut id_map: HashMap<i64, EntityId> = HashMap::new();
    let mut segments: Vec<RoadSegment> = Vec::new();

    let graph_node_id = |osm_id: i64,
                         pos: GeoPosition,
                         graph_nodes: &mut Vec<RoadNode>,
                         id_map: &mut HashMap<i64, EntityId>|
     -> EntityId {
        *id_map.entry(osm_id).or_insert_with(|| {
            let node = RoadNode {
                id: EntityId::new(),
                position: pos,
                node_type: RoadNodeType::Intersection,
            };
            let id = node.id;
            graph_nodes.push(node);
            id
        })
    };

    for way in &routable {
        let highway = way.tags.get("highway").expect("filtered");
        let class = road_class(highway).expect("filtered");

        // Normalize direction: a `oneway=-1` way is reversed once here.
        let mut refs = way.node_refs.clone();
        let one_way = match parse_oneway(&way.tags, class) {
            OneWay::No => false,
            OneWay::Forward => true,
            OneWay::Reverse => {
                refs.reverse();
                true
            }
        };

        let speed = way
            .tags
            .get("maxspeed")
            .and_then(|v| parse_maxspeed_kmh(v))
            .unwrap_or_else(|| default_speed_kmh(class));
        let height_limit = way.tags.get("maxheight").and_then(|v| parse_length_m(v));
        let weight_limit = way.tags.get("maxweight").and_then(|v| parse_weight_kg(v));
        let hazmat_restricted = way.tags.get("hazmat").map(String::as_str) == Some("no");
        let tunnel = way.tags.get("tunnel").is_some_and(|v| v != "no");
        let bridge = way.tags.get("bridge").is_some_and(|v| v != "no");
        let toll = way.tags.get("toll").is_some_and(|v| v != "no");
        let lanes = way.tags.get("lanes").and_then(|v| v.parse::<u8>().ok());

        // Split at intersections: nodes used by >1 way, plus way endpoints.
        let last = refs.len() - 1;
        let mut leg_start = 0usize;
        for i in 1..=last {
            let is_cut = i == last || usage.get(&refs[i]).copied().unwrap_or(0) > 1;
            if !is_cut {
                continue;
            }
            let leg = &refs[leg_start..=i];
            let geometry: Vec<GeoPosition> = leg
                .iter()
                .filter_map(|id| node_pos.get(id).copied())
                .collect();
            if geometry.len() >= 2 {
                let length_m: f64 = geometry.windows(2).map(|w| haversine_m(&w[0], &w[1])).sum();
                let from = graph_node_id(leg[0], geometry[0], &mut graph_nodes, &mut id_map);
                let to = graph_node_id(
                    *leg.last().expect("non-empty"),
                    *geometry.last().expect("non-empty"),
                    &mut graph_nodes,
                    &mut id_map,
                );
                segments.push(RoadSegment {
                    id: EntityId::new(),
                    from_node: from,
                    to_node: to,
                    geometry,
                    road_class: class,
                    one_way,
                    speed_limit_kmh: Some(speed),
                    lane_count: lanes,
                    surface_type: SurfaceType::Unknown,
                    bridge,
                    tunnel,
                    toll,
                    weight_limit_kg: weight_limit,
                    height_limit_m: height_limit,
                    hazmat_restricted,
                    length_m,
                    travel_time_s: Some(length_m / (speed / 3.6)),
                });
            }
            leg_start = i;
        }
    }

    RoadGraph {
        id: EntityId::new(),
        region: region.to_string(),
        nodes: graph_nodes,
        segments,
        version: 1,
        updated_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn node(id: i64, lat: f64, lon: f64) -> RawNode {
        RawNode { id, lat, lon }
    }

    fn way(id: i64, refs: &[i64], tags: &[(&str, &str)]) -> RawWay {
        RawWay {
            id,
            node_refs: refs.to_vec(),
            tags: tags
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<_, _>>(),
        }
    }

    #[test]
    fn splits_ways_at_intersections() {
        // Way A: 1-2-3, Way B: 4-2-5 — node 2 is a crossing.
        let nodes = vec![
            node(1, 32.080, 34.780),
            node(2, 32.081, 34.780),
            node(3, 32.082, 34.780),
            node(4, 32.081, 34.779),
            node(5, 32.081, 34.781),
        ];
        let ways = vec![
            way(10, &[1, 2, 3], &[("highway", "residential")]),
            way(11, &[4, 2, 5], &[("highway", "residential")]),
        ];
        let g = build_graph("test", &nodes, &ways);
        assert_eq!(g.segments.len(), 4, "each way splits into two at node 2");
        assert_eq!(g.nodes.len(), 5);
    }

    #[test]
    fn carries_vehicle_restrictions() {
        let nodes = vec![node(1, 32.0, 34.0), node(2, 32.001, 34.0)];
        let ways = vec![way(
            10,
            &[1, 2],
            &[
                ("highway", "primary"),
                ("maxheight", "4"),
                ("maxweight", "10"),
                ("hazmat", "no"),
                ("tunnel", "yes"),
            ],
        )];
        let g = build_graph("test", &nodes, &ways);
        let s = &g.segments[0];
        assert_eq!(s.height_limit_m, Some(4.0));
        assert_eq!(s.weight_limit_kg, Some(10_000.0));
        assert!(s.hazmat_restricted);
        assert!(s.tunnel);
    }

    #[test]
    fn computes_length_and_time() {
        let nodes = vec![node(1, 32.0, 34.0), node(2, 32.009, 34.0)]; // ~1 km
        let ways = vec![way(
            10,
            &[1, 2],
            &[("highway", "primary"), ("maxspeed", "50")],
        )];
        let g = build_graph("test", &nodes, &ways);
        let s = &g.segments[0];
        assert!(
            (s.length_m - 1000.0).abs() < 15.0,
            "≈1 km, got {}",
            s.length_m
        );
        let t = s.travel_time_s.unwrap();
        assert!((t - s.length_m / (50.0 / 3.6)).abs() < 1e-9);
    }

    #[test]
    fn skips_non_routable_ways() {
        let nodes = vec![node(1, 32.0, 34.0), node(2, 32.001, 34.0)];
        let ways = vec![way(10, &[1, 2], &[("highway", "construction")])];
        let g = build_graph("test", &nodes, &ways);
        assert!(g.segments.is_empty());
    }
}
