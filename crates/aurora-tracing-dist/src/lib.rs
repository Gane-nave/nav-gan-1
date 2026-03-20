//! Distributed Tracing engine for AURORA NAV.
//!
//! Provides trace/span management, context propagation,
//! and trace collection for distributed request tracking.

pub mod collector;
pub mod context;
pub mod span;
