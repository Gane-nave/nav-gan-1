//! Jump consistent hashing for load balancing for AURORA NAV.

mod hasher;
pub use hasher::{jump_hash, JumpHasher};
