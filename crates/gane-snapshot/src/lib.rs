//! State snapshotting and checkpoint/restore for G.A.N.E NAV.
//!
//! Provides point-in-time state capture, incremental snapshots,
//! and checkpoint management for navigation state persistence.

pub mod capture;
pub mod checkpoint;
pub mod diff;
