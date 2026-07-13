//! # gane-debounce
//!
//! Debouncing and throttling for event processing in G.A.N.E NAV.
//! Provides debouncing (fire after quiet period), throttling (rate limiting),
//! and batch collection (accumulate and flush).

pub mod batch;
pub mod debouncer;
pub mod throttle;
