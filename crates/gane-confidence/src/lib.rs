//! AURORA Confidence Metrics — ETA distribution, confidence and volatility indices.
//!
//! Computes probabilistic ETA distributions and confidence/volatility metrics
//! for routes and navigation decisions.

pub mod calculator;

pub use calculator::ConfidenceCalculator;
