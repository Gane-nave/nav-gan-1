//! AURORA NAV — Navigation Pipeline Orchestrator
//!
//! Wires infrastructure crates (cache, circuit breaker, rate limiter,
//! retry, bloom filter, feature flags, etc.) into a unified service
//! layer that the navigation pipeline can use.

pub mod services;

pub use services::ServiceRegistry;
