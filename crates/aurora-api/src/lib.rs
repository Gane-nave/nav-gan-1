//! AURORA NAV — REST API Server
//!
//! Exposes the navigation pipeline, telemetry, integrity, and system health
//! through a RESTful API built on Axum.

pub mod routes;
pub mod state;
pub mod server;

pub use server::run_server;
pub use state::AppState;
