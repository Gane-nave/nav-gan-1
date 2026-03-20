/// External temp sensor: ambient, road surface, icing
/// Phase 563

#[derive(Debug, Clone)]
pub struct ExternalTempSensor {
    pub ambient_temp_c: f64,
    pub road_temp_c: f64,
    pub sensor_ok: bool,
    pub icing_risk: bool,
    pub calibrated: bool,
}

impl Default for ExternalTempSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalTempSensor {
    pub fn new() -> Self {
        Self {
            ambient_temp_c: 20.0,
            road_temp_c: 22.0,
            sensor_ok: true,
            icing_risk: false,
            calibrated: true,
        }
    }

    pub fn temp_valid(&self) -> bool {
        self.sensor_ok && self.calibrated
    }

    pub fn icing_detected(&self) -> bool {
        self.road_temp_c < 3.0 || self.icing_risk
    }

    pub fn all_ok(&self) -> bool {
        self.temp_valid() && !self.icing_detected()
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let c = ExternalTempSensor::new();
        assert!(c.temp_valid());
    }

    #[test]
    fn test_no_ice() {
        let c = ExternalTempSensor::new();
        assert!(!c.icing_detected());
    }

    #[test]
    fn test_all_ok() {
        let c = ExternalTempSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ExternalTempSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_sensor() {
        let mut c = ExternalTempSensor::new();
        c.sensor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ExternalTempSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
