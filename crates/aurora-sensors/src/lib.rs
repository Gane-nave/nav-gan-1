//! AURORA NAV — Sensor Layer
//!
//! IMU, odometry, vehicle CAN signal ingestion and dead reckoning.

pub mod imu;
pub mod odometry;
pub mod dead_reckoning;
pub mod health;

pub use dead_reckoning::DeadReckoningEngine;
pub use health::SensorHealthMonitor;
pub use imu::ImuProcessor;
pub use odometry::OdometryProcessor;
