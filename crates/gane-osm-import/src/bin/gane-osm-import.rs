//! CLI: convert an Overpass JSON extract into a G.A.N.E RoadGraph JSON.
//!
//! Usage:
//!   gane-osm-import <region-name> <overpass.json> [out.json]
//!
//! The output loads directly into `gane-wasm::GaneEngine::load_graph` and the
//! native routing stack.

use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: gane-osm-import <region-name> <overpass.json> [out.json]");
        return ExitCode::from(2);
    }
    let region = &args[1];
    let input = match fs::read_to_string(&args[2]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", args[2]);
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
    match args.get(3) {
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
