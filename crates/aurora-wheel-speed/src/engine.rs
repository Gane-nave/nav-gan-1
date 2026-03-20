/// Wheel speed sensor: reluctor, gap, signal, wiring
/// Phase 669

#[derive(Debug, Clone)]
pub struct WheelSpeedSensor {
    pub reluctor_ok: bool,
    pub gap_ok: bool,
    pub signal_ok: bool,
    pub wiring_ok: bool,
    pub calibrated: bool,
}

impl Default for WheelSpeedSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelSpeedSensor {
    pub fn new() -> Self {
        Self {
            reluctor_ok: true,
            gap_ok: true,
            signal_ok: true,
            wiring_ok: true,
            calibrated: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.reluctor_ok && self.gap_ok
    }

    pub fn output_ok(&self) -> bool {
        self.signal_ok && self.wiring_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.output_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.reluctor_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = WheelSpeedSensor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_output() {
        let c = WheelSpeedSensor::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelSpeedSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WheelSpeedSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_reluctor() {
        let mut c = WheelSpeedSensor::new();
        c.reluctor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WheelSpeedSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
