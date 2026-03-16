//! # Aurora Gateway
//!
//! API gateway for the AURORA NAV platform — request routing, rate
//! limiting, load balancing, circuit breaking, and request transformation.

pub mod balancer;
pub mod circuit;
pub mod limiter;
pub mod router;
