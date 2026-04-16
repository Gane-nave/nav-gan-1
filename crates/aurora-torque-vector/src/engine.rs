/// Torque vectoring: wheel, differential, yaw, cornering
/// Phase 943

#[derive(Debug, Clone)]
pub struct TorqueVector {
    pub wheel_ok: bool,
    pub diff_ok: bool,
    pub yaw_ok: bool,
    pub corner_ok: bool,
    pub sensor_ok: bool,
}

impl Default for TorqueVector {
    fn default() -> Self {
        Self::new()
    }
}

impl TorqueVector {
    pub fn new() -> Self {
        Self {
            wheel_ok: true,
            diff_ok: true,
            yaw_ok: true,
            corner_ok: true,
            sensor_ok: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.wheel_ok && self.yaw_ok && self.sensor_ok
    }

    pub fn distribution_ok(&self) -> bool {
        self.diff_ok && self.corner_ok
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.distribution_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.yaw_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = TorqueVector::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_distribution() {
        let c = TorqueVector::new();
        assert!(c.distribution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TorqueVector::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = TorqueVector::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = TorqueVector::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = TorqueVector::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
