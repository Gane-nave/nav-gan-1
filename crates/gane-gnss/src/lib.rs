//! G.A.N.E NAV — GNSS Acquisition Layer
//!
//! Multi-constellation, multi-frequency GNSS receiver abstraction.
//! Independent acquisition per constellation with quality scoring,
//! selective exclusion, joint solution, and continuous re-entry.

pub mod constellation_manager;
pub mod pvt;
pub mod quality;
pub mod receiver;

pub use constellation_manager::ConstellationManager;
pub use pvt::PvtSolver;
pub use quality::QualityScorer;
pub use receiver::GnssReceiver;
