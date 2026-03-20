//! IMU processing — bias estimation, integration, and calibration.

use aurora_core::sensor::ImuSample;
use nalgebra::Vector3;
use tracing::debug;

/// IMU processor with bias estimation and gravity removal.
pub struct ImuProcessor {
    /// Estimated accelerometer bias (m/s²).
    accel_bias: Vector3<f64>,
    /// Estimated gyroscope bias (rad/s).
    gyro_bias: Vector3<f64>,
    /// Gravity magnitude (m/s²).
    gravity: f64,
    /// Number of samples used for calibration.
    calibration_samples: u64,
    /// Whether initial calibration is complete.
    calibrated: bool,
    /// Running sum for calibration.
    accel_sum: Vector3<f64>,
    gyro_sum: Vector3<f64>,
    /// Minimum samples for calibration.
    min_calibration_samples: u64,
}

impl ImuProcessor {
    pub fn new() -> Self {
        Self {
            accel_bias: Vector3::zeros(),
            gyro_bias: Vector3::zeros(),
            gravity: 9.80665,
            calibration_samples: 0,
            calibrated: false,
            accel_sum: Vector3::zeros(),
            gyro_sum: Vector3::zeros(),
            min_calibration_samples: 100,
        }
    }

    /// Feed a static calibration sample (vehicle must be stationary).
    pub fn calibration_sample(&mut self, sample: &ImuSample) {
        self.accel_sum += Vector3::new(sample.accel_x, sample.accel_y, sample.accel_z);
        self.gyro_sum += Vector3::new(sample.gyro_x, sample.gyro_y, sample.gyro_z);
        self.calibration_samples += 1;

        if self.calibration_samples >= self.min_calibration_samples && !self.calibrated {
            let n = self.calibration_samples as f64;
            let avg_accel = self.accel_sum / n;
            let avg_gyro = self.gyro_sum / n;

            // Accelerometer bias: mean reading minus gravity (assuming Z-up).
            self.accel_bias = avg_accel - Vector3::new(0.0, 0.0, self.gravity);
            self.gyro_bias = avg_gyro;
            self.calibrated = true;

            debug!(
                accel_bias = ?self.accel_bias.as_slice(),
                gyro_bias = ?self.gyro_bias.as_slice(),
                samples = self.calibration_samples,
                "IMU calibration complete"
            );
        }
    }

    /// Process a raw IMU sample: remove bias and return corrected values.
    pub fn process(&self, sample: &ImuSample) -> CorrectedImu {
        let raw_accel = Vector3::new(sample.accel_x, sample.accel_y, sample.accel_z);
        let raw_gyro = Vector3::new(sample.gyro_x, sample.gyro_y, sample.gyro_z);

        let corrected_accel = raw_accel - self.accel_bias;
        let corrected_gyro = raw_gyro - self.gyro_bias;

        // Remove gravity component (assuming body-frame Z is roughly up).
        let specific_force = corrected_accel - Vector3::new(0.0, 0.0, self.gravity);

        CorrectedImu {
            timestamp: sample.timestamp,
            specific_force,
            angular_rate: corrected_gyro,
        }
    }

    pub fn is_calibrated(&self) -> bool {
        self.calibrated
    }

    pub fn accel_bias(&self) -> &Vector3<f64> {
        &self.accel_bias
    }

    pub fn gyro_bias(&self) -> &Vector3<f64> {
        &self.gyro_bias
    }
}

impl Default for ImuProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Bias-corrected IMU output.
#[derive(Debug, Clone)]
pub struct CorrectedImu {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Specific force (gravity removed) in body frame (m/s²).
    pub specific_force: Vector3<f64>,
    /// Angular rate (bias removed) in body frame (rad/s).
    pub angular_rate: Vector3<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn static_sample() -> ImuSample {
        ImuSample {
            timestamp: Utc::now(),
            accel_x: 0.01,
            accel_y: -0.02,
            accel_z: 9.81,
            gyro_x: 0.001,
            gyro_y: -0.001,
            gyro_z: 0.0005,
        }
    }

    #[test]
    fn calibration_completes_after_enough_samples() {
        let mut proc = ImuProcessor::new();
        assert!(!proc.is_calibrated());

        for _ in 0..100 {
            proc.calibration_sample(&static_sample());
        }

        assert!(proc.is_calibrated());
    }

    #[test]
    fn corrected_output_has_gravity_removed() {
        let mut proc = ImuProcessor::new();
        let sample = static_sample();

        for _ in 0..100 {
            proc.calibration_sample(&sample);
        }

        let corrected = proc.process(&sample);
        // After removing bias and gravity, specific force should be near zero for static.
        assert!(corrected.specific_force.norm() < 0.1);
    }
}
