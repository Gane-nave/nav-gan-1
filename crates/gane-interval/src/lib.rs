//! Interval tree for range queries.
//!
//! Provides an augmented interval tree supporting insertion, deletion,
//! point queries, overlap queries, and intersection detection.

mod interval;
mod tree;

pub use interval::Interval;
pub use tree::IntervalTree;
