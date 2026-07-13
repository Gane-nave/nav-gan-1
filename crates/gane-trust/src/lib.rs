//! AURORA Trust System — reputation scoring, multi-source validation, sybil/collusion detection.
//!
//! Computes trust scores for users, events, and data sources, with
//! multi-source validation, sybil attack protection, rate limiting,
//! collusion detection, and anomaly scoring.

pub mod reputation;
pub mod sybil;

pub use reputation::ReputationEngine;
pub use sybil::SybilDetector;
