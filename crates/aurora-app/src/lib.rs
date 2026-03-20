//! AURORA NAV / GMIN — Application Integration Layer
//!
//! Wires all 39 subsystem crates into a unified navigation application
//! with configuration, health monitoring, CLI, and the full processing
//! pipeline.

pub mod cli;
pub mod health;
pub mod pipeline;
