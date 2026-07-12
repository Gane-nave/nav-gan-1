//! Read-Copy-Update for lock-free concurrent reads for G.A.N.E NAV.

mod rcu;
pub use rcu::RcuCell;
