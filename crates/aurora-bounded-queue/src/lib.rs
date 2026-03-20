//! Bounded MPMC queue with backpressure for AURORA NAV.

mod queue;
pub use queue::{BoundedQueue, PushResult};
