#![allow(unknown_lints)]
#![allow(clippy::unnecessary_map_or)]
//! G.A.N.E NAV — Fleet Operations Engine
//!
//! Task management, assignment, dispatch, SLA monitoring, proof of visit/delivery,
//! and multi-driver coordination for fleet operations.

pub mod assignment;
pub mod dispatch;
pub mod sla;
pub mod task;
