//! Authentication, authorization, and session management for AURORA NAV.
//!
//! Provides JWT token generation/validation, API key management, role-based
//! access control (RBAC), and session lifecycle management.

pub mod api_key;
pub mod jwt;
pub mod rbac;
pub mod session;
