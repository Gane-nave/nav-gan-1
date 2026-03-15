//! AURORA NAV / GMIN — Canonical Data Model
//!
//! Single Source of Truth for the Global Mobility Intelligence Network.
//! All domain entities, value objects, and enumerations are defined here.

pub mod types;
pub mod gnss;
pub mod sensor;
pub mod map;
pub mod route;
pub mod incident;
pub mod scoring;
pub mod policy;
pub mod fleet;
pub mod energy;
pub mod infrastructure;
pub mod communication;
pub mod sync;
pub mod discovery;
pub mod error;

pub use types::*;
pub use error::AuroraError;
