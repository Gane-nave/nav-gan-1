#![allow(unknown_lints)]
#![allow(clippy::unnecessary_map_or)]
//! AURORA NAV — Instrumentation & Environmental Data
//!
//! Compass heading, speedometer, altimeter, weather integration,
//! ambient light/temperature sensing, and environmental condition monitoring.

pub mod altimeter;
pub mod compass;
pub mod speedometer;
pub mod weather;
