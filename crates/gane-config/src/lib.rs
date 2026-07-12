//! G.A.N.E NAV — Unified Configuration
//!
//! Provides a single, strongly-typed configuration tree that every subsystem
//! reads from.  Supports loading from TOML files, environment variable
//! overrides (`AURORA_*`), and programmatic builder patterns.

pub mod builder;
pub mod defaults;
pub mod loader;
pub mod sections;
pub mod validate;

pub use builder::ConfigBuilder;
pub use loader::load_config;
pub use sections::AuroraConfig;
pub use validate::ConfigError;
