/// IMU sensor: accelerometer, gyroscope, orientation, motion detection
/// Phase 185

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct ImuSensor {
    pub accel: Vector3,
    pub gyro: Vector3,
    pub sample_rate_hz: u32,
    pub calibrated: bool,
    pub temp_c: f64,
}

impl Default for ImuSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl ImuSensor {
    pub fn new() -> Self {
        Self {
            accel: Vector3::new(0.0, 0.0, -9.81),
            gyro: Vector3::zero(),
            sample_rate_hz: 200,
            calibrated: true,
            temp_c: 25.0,
        }
    }

    pub fn total_accel_g(&self) -> f64 {
        self.accel.magnitude() / 9.81
    }

    pub fn total_rotation_dps(&self) -> f64 {
        self.gyro.magnitude()
    }

    pub fn is_stationary(&self) -> bool {
        (self.total_accel_g() - 1.0).abs() < 0.05 && self.total_rotation_dps() < 1.0
    }

    pub fn impact_detected(&self) -> bool {
        self.total_accel_g() > 4.0
    }

    pub fn tilt_deg(&self) -> f64 {
        if self.accel.magnitude() < 0.01 {
            return 0.0;
        }
        (self.accel.z.abs() / self.accel.magnitude()).acos().to_degrees()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || self.temp_c > 60.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_accel_gravity() {
        let s = ImuSensor::new();
        assert!((s.total_accel_g() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_stationary() {
        let s = ImuSensor::new();
        assert!(s.is_stationary());
    }

    #[test]
    fn test_not_stationary() {
        let mut s = ImuSensor::new();
        s.accel = Vector3::new(5.0, 0.0, -9.81);
        assert!(!s.is_stationary());
    }

    #[test]
    fn test_no_impact() {
        let s = ImuSensor::new();
        assert!(!s.impact_detected());
    }

    #[test]
    fn test_impact() {
        let mut s = ImuSensor::new();
        s.accel = Vector3::new(40.0, 20.0, -9.81);
        assert!(s.impact_detected());
    }

    #[test]
    fn test_tilt() {
        let s = ImuSensor::new();
        assert!(s.tilt_deg() < 5.0);
    }

    #[test]
    fn test_calibrated() {
        let s = ImuSensor::new();
        assert!(!s.needs_calibration());
    }
}
