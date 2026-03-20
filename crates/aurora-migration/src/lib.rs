//! Data migration framework for schema evolution and transformation.
//!
//! Provides versioned migrations, rollback support, and migration registry
//! for managing system state evolution.

pub mod plan;
pub mod runner;
pub mod version;
