#![allow(unknown_lints)]
#![allow(clippy::unnecessary_map_or)]
//! AURORA NAV — Emergency Satellite Layer
//!
//! Provides emergency location sharing, SOS broadcasting, satellite-based
//! navigation, message verification, and multi-channel failover during
//! communication disruption (no cellular, no internet, infrastructure failure).

pub mod beacon;
pub mod channel;
pub mod location;
pub mod protocol;
pub mod verification;
