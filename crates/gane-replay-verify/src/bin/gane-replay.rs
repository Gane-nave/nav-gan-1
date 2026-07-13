//! CLI: replay a JSONL sensor log through the canonical ESKF.
//!
//! Usage:
//!   gane-replay golden            # emit the built-in golden trace as JSONL
//!   gane-replay <log.jsonl>       # replay a log, print ReplayResult JSON
//!
//! The printed JSON uses full f64 precision so external runners (e.g. the
//! WASM engine in Node) can be compared bit-for-bit.

use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("golden") => {
            for ev in gane_replay_verify::golden_trace() {
                println!("{}", serde_json::to_string(&ev).expect("serializable"));
            }
            ExitCode::SUCCESS
        }
        Some(path) => {
            let text = match fs::read_to_string(path) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("error: cannot read {path}: {e}");
                    return ExitCode::FAILURE;
                }
            };
            match gane_replay_verify::parse_log(&text) {
                Ok(events) => {
                    let result = gane_replay_verify::replay(&events);
                    println!("{}", serde_json::to_string(&result).expect("serializable"));
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: invalid log: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        None => {
            eprintln!("usage: gane-replay <golden | log.jsonl>");
            ExitCode::from(2)
        }
    }
}
