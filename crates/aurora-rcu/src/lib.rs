//! Read-Copy-Update for lock-free concurrent reads for AURORA NAV.

mod rcu;
pub use rcu::RcuCell;
