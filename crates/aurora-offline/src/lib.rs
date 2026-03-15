//! AURORA NAV — Offline Maps & Sync
//!
//! Tile caching, differential map updates, local routing fallback,
//! offline evidence storage, and deterministic synchronization per Section 33.

pub mod cache;
pub mod sync_engine;

pub use cache::OfflineCache;
pub use sync_engine::SyncEngine;
