//! AURORA NAV — Fusion Engine
//!
//! Extended Kalman Filter (EKF) combining GNSS, INS, odometry,
//! map matching, and visual constraints into a unified navigation solution.

pub mod ekf;
pub mod engine;
pub mod measurement;
pub mod state;

pub use engine::FusionEngine;
pub use state::FusionState;
