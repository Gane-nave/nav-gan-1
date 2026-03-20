//! Consistent hashing ring for distributed load balancing.
//!
//! Provides virtual-node based consistent hashing with configurable
//! replication factor, node addition/removal, and key lookup.

mod node;
mod ring;

pub use node::VirtualNode;
pub use ring::HashRing;
