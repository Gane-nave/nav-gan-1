//! AURORA NAV / GMIN — Canonical Data Model
//!
//! Single Source of Truth for the Global Mobility Intelligence Network.
//! All domain entities, value objects, and enumerations are defined here.

pub mod communication;
pub mod discovery;
pub mod energy;
pub mod error;
pub mod fleet;
pub mod gnss;
pub mod incident;
pub mod infrastructure;
pub mod map;
pub mod policy;
pub mod route;
pub mod scoring;
pub mod sensor;
pub mod sync;
pub mod types;
pub mod vehicle;

pub use error::AuroraError;
pub use types::*;
