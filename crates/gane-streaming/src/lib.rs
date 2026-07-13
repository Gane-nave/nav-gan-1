//! Event streaming and message queue for real-time navigation data.
//!
//! Provides topic-based pub/sub, ordered message queues, event sourcing,
//! and back-pressure handling for navigation event pipelines.

pub mod backpressure;
pub mod queue;
pub mod sourcing;
pub mod topic;
