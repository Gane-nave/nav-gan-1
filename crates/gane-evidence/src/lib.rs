//! AURORA Evidence System — incident reporting, evidence vault, signed metadata, timeline.
//!
//! Provides instant incident reporting, evidence capture (photo, video, voice,
//! sensor-triggered), signed metadata, encrypted evidence vault, incident
//! timelines, and evidence-based map updates.

pub mod reporter;
pub mod vault;

pub use reporter::IncidentReporter;
pub use vault::EvidenceVault;
