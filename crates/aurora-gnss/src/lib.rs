//! AURORA NAV — GNSS Acquisition Layer
//!
//! Multi-constellation, multi-frequency GNSS receiver abstraction.
//! Independent acquisition per constellation with quality scoring,
//! selective exclusion, joint solution, and continuous re-entry.

pub mod receiver;
pub mod quality;
pub mod pvt;
pub mod constellation_manager;

pub use receiver::GnssReceiver;
pub use quality::QualityScorer;
pub use pvt::PvtSolver;
pub use constellation_manager::ConstellationManager;
