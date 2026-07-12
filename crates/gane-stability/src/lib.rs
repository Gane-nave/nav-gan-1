//! Aurora Stability — network stability engine with stability index computation,
//! oscillation suppression, corridor throttling, and fairness constraints.
//!
//! Provides the "Network Stability over Local Optimization" principle from
//! the G.A.N.E NAV spec (principle #7).

pub mod index;
pub mod oscillation;
