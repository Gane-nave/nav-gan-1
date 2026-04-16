/// Solar sensor: sun intensity, direction, auto climate compensation
/// Phase 449

#[derive(Debug, Clone)]
pub struct SolarSensor {
    pub intensity_wm2: f64,
    pub direction_deg: f64,
    pub sensor_ok: bool,
    pub dual_zone: bool,
    pub compensating: bool,
}

impl Default for SolarSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl SolarSensor {
    pub fn new() -> Self {
        Self {
            intensity_wm2: 500.0,
            direction_deg: 45.0,
            sensor_ok: true,
            dual_zone: true,
            compensating: true,
        }
    }

    pub fn sunny(&self) -> bool {
        self.intensity_wm2 > 300.0
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok
    }

    pub fn load_pct(&self) -> f64 {
        (self.intensity_wm2 / 1000.0 * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunny() {
        let s = SolarSensor::new();
        assert!(s.sunny());
    }

    #[test]
    fn test_all_ok() {
        let s = SolarSensor::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_service() {
        let s = SolarSensor::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_load() {
        let s = SolarSensor::new();
        assert!(s.load_pct() > 40.0);
    }

    #[test]
    fn test_bad_sensor() {
        let mut s = SolarSensor::new();
        s.sensor_ok = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SolarSensor::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
