//! Advanced serialization and schema evolution for G.A.N.E NAV.
//!
//! Provides schema-versioned encoding/decoding, field-level migration,
//! and compact wire formats for navigation data exchange.

pub mod codec;
pub mod migration;
pub mod schema;
