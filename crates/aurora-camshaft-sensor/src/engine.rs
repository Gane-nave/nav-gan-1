/// Camshaft sensor: hall effect, phase, timing
/// Phase 583

#[derive(Debug, Clone)]
pub struct CamshaftSensor {
    pub hall_ok: bool,
    pub phase_ok: bool,
    pub timing_ok: bool,
    pub wiring_ok: bool,
    pub calibrated: bool,
}

impl Default for CamshaftSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CamshaftSensor {
    pub fn new() -> Self {
        Self {
            hall_ok: true,
            phase_ok: true,
            timing_ok: true,
            wiring_ok: true,
            calibrated: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.hall_ok && self.phase_ok
    }

    pub fn timing_valid(&self) -> bool {
        self.timing_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.timing_valid() && self.wiring_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.hall_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hall_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = CamshaftSensor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_timing() {
        let c = CamshaftSensor::new();
        assert!(c.timing_valid());
    }

    #[test]
    fn test_all_ok() {
        let c = CamshaftSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CamshaftSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_hall() {
        let mut c = CamshaftSensor::new();
        c.hall_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CamshaftSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
