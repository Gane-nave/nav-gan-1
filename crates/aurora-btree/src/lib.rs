//! B-Tree with order-statistic operations for AURORA NAV.
//!
//! Provides a sorted key-value map backed by a B-Tree with
//! rank and select operations for positional access.

mod btree;
pub use btree::{BTree, BTreeStats};
