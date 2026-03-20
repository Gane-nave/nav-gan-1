/// IMU unit: accelerometer, gyroscope, magnetometer
/// Phase 703

#[derive(Debug, Clone)]
pub struct ImuUnit {
    pub accel_ok: bool,
    pub gyro_ok: bool,
    pub mag_ok: bool,
    pub temp_comp_ok: bool,
    pub calibrated: bool,
}

impl Default for ImuUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl ImuUnit {
    pub fn new() -> Self {
        Self {
            accel_ok: true,
            gyro_ok: true,
            mag_ok: true,
            temp_comp_ok: true,
            calibrated: true,
        }
    }

    pub fn inertial_ok(&self) -> bool {
        self.accel_ok && self.gyro_ok
    }

    pub fn heading_ok(&self) -> bool {
        self.mag_ok && self.temp_comp_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.inertial_ok() && self.heading_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.accel_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.accel_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inertial() {
        let c = ImuUnit::new();
        assert!(c.inertial_ok());
    }

    #[test]
    fn test_heading() {
        let c = ImuUnit::new();
        assert!(c.heading_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ImuUnit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = ImuUnit::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_accel() {
        let mut c = ImuUnit::new();
        c.accel_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = ImuUnit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
