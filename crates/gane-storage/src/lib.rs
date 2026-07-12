//! Persistent storage layer for G.A.N.E NAV.
//!
//! Provides time-series data storage, key-value store, schema migrations,
//! and backup/restore capabilities for the navigation system.

pub mod kv;
pub mod migrations;
pub mod snapshots;
pub mod timeseries;
