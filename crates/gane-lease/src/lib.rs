//! # gane-lease
//!
//! Distributed lease and lock management with TTL for G.A.N.E NAV.
//! Provides time-bounded resource ownership, conflict detection,
//! renewal limits, and automatic expiration.

pub mod grant;
pub mod manager;
