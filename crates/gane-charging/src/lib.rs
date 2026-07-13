#![allow(unknown_lints)]
#![allow(clippy::unnecessary_map_or)]
//! Aurora Charging — vehicle energy model, consumption prediction,
//! charging station integration, and eco/fast/stable routing.
//!
//! Implements section 30 of the G.A.N.E NAV spec.

pub mod energy;
pub mod station;
