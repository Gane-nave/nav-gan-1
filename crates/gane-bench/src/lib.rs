//! G.A.N.E NAV — Performance Benchmarks
//!
//! Criterion-based benchmarks for critical navigation paths:
//! - GNSS satellite tracking and PVT solving
//! - EKF fusion prediction and update cycles
//! - Dijkstra routing on road graphs
//! - Full pipeline instantiation and health checks
//! - Configuration loading and validation
//! - Event bus publish/subscribe throughput

/// Re-export for benchmark helpers.
pub use gane_config::AuroraConfig;
