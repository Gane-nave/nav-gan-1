//! AURORA NAV — Integrity and Trust Layer
//!
//! Fault detection, source scoring, spoof/jam detection,
//! cross-checks, and alarm generation per Section 12.

pub mod detector;
pub mod engine;
pub mod trust;

pub use engine::IntegrityEngine;
pub use trust::TrustManager;
