//! AURORA Anti-Manipulation Infrastructure — anomaly detection, adversarial defense,
//! algorithm transparency logging.
//!
//! Protects the navigation system from data manipulation, spoofed reports,
//! coordinated attacks, and ensures full algorithmic transparency through
//! immutable audit trails.

pub mod anomaly;
pub mod transparency;

pub use anomaly::AnomalyDetector;
pub use transparency::TransparencyLogger;
