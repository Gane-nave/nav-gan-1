/// TPMS: valve sensor, battery, RF, threshold
/// Phase 734

#[derive(Debug, Clone)]
pub struct TirePressure {
    pub sensor_ok: bool,
    pub battery_ok: bool,
    pub rf_ok: bool,
    pub threshold_ok: bool,
    pub calibrated: bool,
}

impl Default for TirePressure {
    fn default() -> Self {
        Self::new()
    }
}

impl TirePressure {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            battery_ok: true,
            rf_ok: true,
            threshold_ok: true,
            calibrated: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.sensor_ok && self.threshold_ok && self.calibrated
    }

    pub fn communication_ok(&self) -> bool {
        self.rf_ok && self.battery_ok
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.communication_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sensor_ok || !self.battery_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = TirePressure::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_communication() {
        let c = TirePressure::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TirePressure::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TirePressure::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sensor() {
        let mut c = TirePressure::new();
        c.sensor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TirePressure::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
