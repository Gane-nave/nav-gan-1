//! AURORA NAV — Continuity Manager
//!
//! Automatic mode switching, graceful degradation, recovery logic,
//! and re-entry logic per Section 13 of the spec.
//!
//! Operating modes:
//!   A — Full GNSS + Dual Frequency + Corrections
//!   B — Multi-GNSS without corrections
//!   C — GNSS Degraded + INS fused
//!   D — INS + Odometry + Map Matching (dead reckoning)
//!   E — Emergency bounded localization

pub mod manager;
pub mod health;

pub use manager::ContinuityManager;
pub use health::HealthStateMachine;
