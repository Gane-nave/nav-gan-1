//! Backpressure management and flow control for G.A.N.E NAV.
//!
//! Provides token bucket rate limiting, sliding window counters,
//! and adaptive flow control for managing data ingestion rates.

pub mod bucket;
pub mod controller;
pub mod window;
