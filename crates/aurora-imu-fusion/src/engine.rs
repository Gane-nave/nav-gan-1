/// IMU fusion: accel, gyro, mag, filter, calibrate
/// Phase 1108

#[derive(Debug, Clone)]
pub struct ImuFusion {
    pub accel_ok: bool,
    pub gyro_ok: bool,
    pub mag_ok: bool,
    pub filter_ok: bool,
    pub calibrate_ok: bool,
}

impl Default for ImuFusion {
    fn default() -> Self {
        Self::new()
    }
}

impl ImuFusion {
    pub fn new() -> Self {
        Self {
            accel_ok: true,
            gyro_ok: true,
            mag_ok: true,
            filter_ok: true,
            calibrate_ok: true,
        }
    }

    pub fn sensors_ok(&self) -> bool {
        self.accel_ok && self.gyro_ok && self.mag_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.filter_ok && self.calibrate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensors_ok() && self.processing_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.calibrate_ok || !self.accel_ok
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
    fn test_sensors() {
        let c = ImuFusion::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_processing() {
        let c = ImuFusion::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ImuFusion::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = ImuFusion::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_calibrate() {
        let mut c = ImuFusion::new();
        c.calibrate_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = ImuFusion::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
