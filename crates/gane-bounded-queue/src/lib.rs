//! Bounded MPMC queue with backpressure for G.A.N.E NAV.

mod queue;
pub use queue::{BoundedQueue, PushResult};
