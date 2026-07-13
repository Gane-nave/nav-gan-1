//! Health check aggregation and readiness probes for G.A.N.E NAV.
//!
//! Provides component health tracking, dependency checking,
//! readiness/liveness probes, and aggregated system health status.

pub mod aggregator;
pub mod checker;
pub mod probe;
