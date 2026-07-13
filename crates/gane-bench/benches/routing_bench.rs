//! Benchmarks for road graph indexing and Dijkstra shortest-path.

use chrono::Utc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gane_core::map::*;
use gane_core::types::{EntityId, GeoPosition};
use gane_map::graph::RoadGraphIndex;
use gane_routing::dijkstra::{self, CostFn};

fn make_linear_graph(n: usize) -> (RoadGraph, Vec<EntityId>) {
    let mut nodes = Vec::with_capacity(n);
    let mut segments = Vec::with_capacity(n - 1);
    let mut node_ids = Vec::with_capacity(n);

    for i in 0..n {
        let node = RoadNode {
            id: EntityId::new(),
            position: GeoPosition {
                latitude_deg: 32.08 + i as f64 * 0.001,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            node_type: RoadNodeType::Intersection,
        };
        node_ids.push(node.id);
        nodes.push(node);
    }

    for i in 0..n - 1 {
        let seg = RoadSegment {
            id: EntityId::new(),
            from_node: nodes[i].id,
            to_node: nodes[i + 1].id,
            geometry: vec![nodes[i].position, nodes[i + 1].position],
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
        segments.push(seg);
    }

    let graph = RoadGraph {
        id: EntityId::new(),
        region: "bench".into(),
        nodes,
        segments,
        version: 1,
        updated_at: Utc::now(),
    };
    (graph, node_ids)
}

fn bench_graph_index_10_nodes(c: &mut Criterion) {
    let (graph, _) = make_linear_graph(10);
    c.bench_function("RoadGraphIndex::from_graph_10_nodes", |b| {
        b.iter(|| black_box(RoadGraphIndex::from_graph(&graph)));
    });
}

fn bench_graph_index_100_nodes(c: &mut Criterion) {
    let (graph, _) = make_linear_graph(100);
    c.bench_function("RoadGraphIndex::from_graph_100_nodes", |b| {
        b.iter(|| black_box(RoadGraphIndex::from_graph(&graph)));
    });
}

fn bench_dijkstra_10_nodes(c: &mut Criterion) {
    let (graph, node_ids) = make_linear_graph(10);
    let index = RoadGraphIndex::from_graph(&graph);
    let cost_fn: CostFn = Box::new(dijkstra::cost::by_distance);

    c.bench_function("dijkstra_10_nodes_end_to_end", |b| {
        b.iter(|| {
            black_box(dijkstra::shortest_path(
                &index,
                node_ids[0],
                node_ids[9],
                &cost_fn,
            ))
        });
    });
}

fn bench_dijkstra_100_nodes(c: &mut Criterion) {
    let (graph, node_ids) = make_linear_graph(100);
    let index = RoadGraphIndex::from_graph(&graph);
    let cost_fn: CostFn = Box::new(dijkstra::cost::by_distance);

    c.bench_function("dijkstra_100_nodes_end_to_end", |b| {
        b.iter(|| {
            black_box(dijkstra::shortest_path(
                &index,
                node_ids[0],
                node_ids[99],
                &cost_fn,
            ))
        });
    });
}

fn bench_dijkstra_by_time(c: &mut Criterion) {
    let (graph, node_ids) = make_linear_graph(50);
    let index = RoadGraphIndex::from_graph(&graph);
    let cost_fn: CostFn = Box::new(dijkstra::cost::by_time);

    c.bench_function("dijkstra_50_nodes_by_time", |b| {
        b.iter(|| {
            black_box(dijkstra::shortest_path(
                &index,
                node_ids[0],
                node_ids[49],
                &cost_fn,
            ))
        });
    });
}

fn bench_nearest_segment(c: &mut Criterion) {
    let (graph, _) = make_linear_graph(100);
    let index = RoadGraphIndex::from_graph(&graph);
    let pos = GeoPosition {
        latitude_deg: 32.13,
        longitude_deg: 34.7801,
        altitude_m: None,
    };

    c.bench_function("nearest_segment_100_node_graph", |b| {
        b.iter(|| black_box(index.nearest_segment(&pos)));
    });
}

criterion_group!(
    benches,
    bench_graph_index_10_nodes,
    bench_graph_index_100_nodes,
    bench_dijkstra_10_nodes,
    bench_dijkstra_100_nodes,
    bench_dijkstra_by_time,
    bench_nearest_segment,
);
criterion_main!(benches);
