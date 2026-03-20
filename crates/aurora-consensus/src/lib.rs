//! Distributed consensus and coordination for navigation services.
//!
//! Provides leader election, distributed locking, state replication,
//! and cluster membership management.

pub mod election;
pub mod lock;
pub mod membership;
pub mod replication;
