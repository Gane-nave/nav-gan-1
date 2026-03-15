//! Security middleware, rate limiting, input validation, and TLS configuration
//! for AURORA NAV.
//!
//! Provides defense-in-depth security layers including request rate limiting,
//! input sanitization, security headers, and CORS policy management.

pub mod cors;
pub mod headers;
pub mod rate_limit;
pub mod tls;
pub mod validation;
