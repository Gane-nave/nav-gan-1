#![allow(unknown_lints)]
#![allow(clippy::unnecessary_map_or)]
//! Aurora Traffic — traffic flow control, demand forecasting, congestion prediction,
//! flow balancing, and herd behaviour suppression.
//!
//! Implements the closed-loop traffic control cycle from the AURORA NAV spec:
//! Measure → Predict → Allocate → Monitor → Correct.

pub mod flow;
pub mod forecast;
pub mod herd;
