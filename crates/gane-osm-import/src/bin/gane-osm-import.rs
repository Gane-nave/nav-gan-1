//! G.A.N.E NAV road-data CLI.
//!
//! Usage:
//!   gane-osm-import import <region-name> <overpass.json> [out.json]
//!   gane-osm-import route  <graph.json> <from_lat,from_lon> <to_lat,to_lon> [vehicle]
//!
//! `import` converts an Overpass JSON extract into a RoadGraph JSON that loads
//! directly into `gane-wasm::GaneEngine::load_graph` and the native stack.
//! `route` snaps coordinates to the graph and computes a vehicle-aware route
//! (vehicle ∈ car|truck|van|bus|emergency; omit for unconstrained).

use std::env;
use std::fs;
use std::process::ExitCode;

use aurora_map::graph::RoadGraphIndex;
use gane_osm_import::route::{envelope_by_name, route_geo};

fn parse_latlon(s: &str) -> Option<(f64, f64)> {
    let (a, b) = s.split_once(',')?;
    Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
}

fn cmd_import(args: &[String]) -> ExitCode {
    if args.len() < 2 {
        eprintln!("usage: gane-osm-import import <region-name> <overpass.json> [out.json]");
        return ExitCode::from(2);
    }
    let region = &args[0];
    let input = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", args[1]);
            return ExitCode::FAILURE;
        }
    };
    let graph = match gane_osm_import::graph_from_overpass_json(region, &input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("error: invalid Overpass JSON: {e}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!(
        "imported region '{}': {} nodes, {} segments",
        region,
        graph.nodes.len(),
        graph.segments.len()
    );
    let out = serde_json::to_string(&graph).expect("serializable graph");
    match args.get(2) {
        Some(path) => {
            if let Err(e) = fs::write(path, out) {
                eprintln!("error: cannot write {path}: {e}");
                return ExitCode::FAILURE;
            }
        }
        None => println!("{out}"),
    }
    ExitCode::SUCCESS
}

fn cmd_route(args: &[String]) -> ExitCode {
    if args.len() < 3 {
        eprintln!(
            "usage: gane-osm-import route <graph.json> <from_lat,from_lon> <to_lat,to_lon> [vehicle]"
        );
        return ExitCode::from(2);
    }
    let graph: aurora_core::map::RoadGraph = match fs::read_to_string(&args[0])
        .map_err(|e| e.to_string())
        .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
    {
        Ok(g) => g,
        Err(e) => {
            eprintln!("error: cannot load graph {}: {e}", args[0]);
            return ExitCode::FAILURE;
        }
    };
    let (Some(from), Some(to)) = (parse_latlon(&args[1]), parse_latlon(&args[2])) else {
        eprintln!("error: coordinates must be lat,lon");
        return ExitCode::from(2);
    };
    let envelope = match args.get(3) {
        Some(name) => match envelope_by_name(name) {
            Some(e) => Some(e),
            None => {
                eprintln!("error: unknown vehicle '{name}' (car|truck|van|bus|emergency)");
                return ExitCode::from(2);
            }
        },
        None => None,
    };

    let index = RoadGraphIndex::from_graph(&graph);
    let started = std::time::Instant::now();
    match route_geo(&graph, &index, from, to, envelope) {
        Some(route) => {
            eprintln!(
                "route: {} nodes, {:.0} m, {:.0} s drive, computed in {:.1} ms",
                route.node_count,
                route.total_length_m,
                route.total_time_s,
                started.elapsed().as_secs_f64() * 1000.0
            );
            println!("{}", serde_json::to_string(&route).expect("serializable"));
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("no legal route for the requested vehicle");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("import") => cmd_import(&args[2..]),
        Some("route") => cmd_route(&args[2..]),
        // Back-compat: bare `<region> <overpass.json> [out]` behaves as import.
        Some(_) if args.len() >= 3 => cmd_import(&args[1..]),
        _ => {
            eprintln!("usage: gane-osm-import <import|route> ...");
            ExitCode::from(2)
        }
    }
}
