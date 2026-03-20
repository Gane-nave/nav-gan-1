//! Circuit breaker and resilience patterns for AURORA NAV.
//!
//! Provides circuit breaker state machine, retry policies, bulkhead isolation,
//! and fallback strategies for fault-tolerant service communication.

pub mod breaker;
pub mod bulkhead;
pub mod retry;
