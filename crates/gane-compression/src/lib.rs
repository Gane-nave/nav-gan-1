//! Data Compression & Serialization engine for G.A.N.E NAV.
//!
//! Provides run-length encoding, delta encoding, binary serialization,
//! and data deduplication with content-addressable storage.

pub mod binary;
pub mod dedup;
pub mod delta;
pub mod rle;
