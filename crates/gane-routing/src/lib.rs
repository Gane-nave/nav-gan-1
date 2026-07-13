//! G.A.N.E NAV — Routing Engine
//!
//! Turn-by-turn navigation, alternative route generation,
//! corridor-based routing, and multi-objective optimization per Section 18-19.

pub mod dijkstra;
pub mod navigator;
pub mod planner;
pub mod vehicle_aware;

pub use navigator::Navigator;
pub use planner::RoutePlanner;
