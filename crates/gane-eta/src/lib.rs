//! Dynamic ETA prediction engine.
//!
//! Combines real-time traffic, historical patterns, and route geometry
//! to produce accurate estimated time of arrival predictions.

pub mod predictor;
pub use predictor::EtaPredictor;
