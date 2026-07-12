//! Advanced retry strategies with backoff, jitter, and budget control.
//!
//! Provides configurable retry policies including exponential backoff,
//! linear backoff, and retry budgets for controlling retry storms.

pub mod backoff;
pub mod budget;
pub mod policy;
