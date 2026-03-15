//! AURORA NAV — Telemetry, Audit, and Replay
//!
//! Structured logging, raw GNSS/fusion recording, source trust history,
//! failover events, spoof/jam alerts, and replay session management.

pub mod recorder;
pub mod audit;
pub mod replay;

pub use audit::AuditLogger;
pub use recorder::TelemetryRecorder;
pub use replay::ReplayController;
