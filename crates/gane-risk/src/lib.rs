//! AURORA Risk Engine — segment, route, and systemic risk scoring.
//!
//! Computes risk scores for road segments and routes based on multiple
//! weighted factors: accident history, weather, visibility, infrastructure
//! condition, curvature, and more.

pub mod engine;
pub mod hazard;

pub use engine::RiskEngine;
pub use hazard::HazardForecaster;
