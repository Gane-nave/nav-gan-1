//! AURORA NAV — Correction Layer
//!
//! SBAS, PPP, RTK, NRTK correction management with fallback logic.
//! Operates in five modes (A through E) as defined in the spec.

pub mod manager;
pub mod source;

pub use manager::CorrectionManager;
pub use source::CorrectionProvider;
