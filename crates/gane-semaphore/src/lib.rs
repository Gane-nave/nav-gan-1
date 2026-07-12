//! # gane-semaphore
//!
//! Counting semaphore for resource access control in G.A.N.E NAV.
//! Provides permit-based concurrency limiting with TTL, fair queuing,
//! and priority scheduling.

pub mod counter;
pub mod fair;
pub mod permit;
