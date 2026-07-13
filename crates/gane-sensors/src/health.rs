//! Sensor health monitoring.

use chrono::{DateTime, Utc};
use gane_core::sensor::{SensorHealthReport, SensorStatus};

/// Monitors the health of all sensor subsystems.
pub struct SensorHealthMonitor {
    imu_last_seen: Option<DateTime<Utc>>,
    mag_last_seen: Option<DateTime<Utc>>,
    baro_last_seen: Option<DateTime<Utc>>,
    wheel_last_seen: Option<DateTime<Utc>>,
    camera_last_seen: Option<DateTime<Utc>>,
    can_last_seen: Option<DateTime<Utc>>,
    /// Maximum age in seconds before a sensor is considered degraded.
    degraded_threshold_s: f64,
    /// Maximum age before a sensor is considered failed.
    failed_threshold_s: f64,
}

impl SensorHealthMonitor {
    pub fn new() -> Self {
        Self {
            imu_last_seen: None,
            mag_last_seen: None,
            baro_last_seen: None,
            wheel_last_seen: None,
            camera_last_seen: None,
            can_last_seen: None,
            degraded_threshold_s: 2.0,
            failed_threshold_s: 10.0,
        }
    }

    pub fn report_imu(&mut self, ts: DateTime<Utc>) {
        self.imu_last_seen = Some(ts);
    }

    pub fn report_magnetometer(&mut self, ts: DateTime<Utc>) {
        self.mag_last_seen = Some(ts);
    }

    pub fn report_barometer(&mut self, ts: DateTime<Utc>) {
        self.baro_last_seen = Some(ts);
    }

    pub fn report_wheel_odometry(&mut self, ts: DateTime<Utc>) {
        self.wheel_last_seen = Some(ts);
    }

    pub fn report_camera(&mut self, ts: DateTime<Utc>) {
        self.camera_last_seen = Some(ts);
    }

    pub fn report_can_bus(&mut self, ts: DateTime<Utc>) {
        self.can_last_seen = Some(ts);
    }

    /// Generate the current health report.
    pub fn health_report(&self) -> SensorHealthReport {
        let now = Utc::now();
        SensorHealthReport {
            timestamp: now,
            imu_status: self.status_for(self.imu_last_seen, now),
            magnetometer_status: self.status_for(self.mag_last_seen, now),
            barometer_status: self.status_for(self.baro_last_seen, now),
            wheel_odometry_status: self.status_for(self.wheel_last_seen, now),
            camera_status: self.status_for(self.camera_last_seen, now),
            can_bus_status: self.status_for(self.can_last_seen, now),
        }
    }

    fn status_for(&self, last_seen: Option<DateTime<Utc>>, now: DateTime<Utc>) -> SensorStatus {
        match last_seen {
            None => SensorStatus::NotAvailable,
            Some(ts) => {
                let age = (now - ts).num_milliseconds() as f64 / 1000.0;
                if age <= self.degraded_threshold_s {
                    SensorStatus::Active
                } else if age <= self.failed_threshold_s {
                    SensorStatus::Degraded
                } else {
                    SensorStatus::Failed
                }
            }
        }
    }
}

impl Default for SensorHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}
