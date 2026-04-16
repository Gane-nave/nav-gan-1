/// Oil pressure sensor: transducer, warning, range
/// Phase 591

#[derive(Debug, Clone)]
pub struct OilPressureSensor {
    pub pressure_bar: f64,
    pub min_pressure_bar: f64,
    pub transducer_ok: bool,
    pub warning_ok: bool,
    pub wiring_ok: bool,
}

impl Default for OilPressureSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl OilPressureSensor {
    pub fn new() -> Self {
        Self {
            pressure_bar: 3.5,
            min_pressure_bar: 1.0,
            transducer_ok: true,
            warning_ok: true,
            wiring_ok: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_bar > self.min_pressure_bar
    }

    pub fn sensor_ok(&self) -> bool {
        self.transducer_ok && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.sensor_ok() && self.warning_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.transducer_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transducer_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = OilPressureSensor::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_sensor() {
        let c = OilPressureSensor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilPressureSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = OilPressureSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_transducer() {
        let mut c = OilPressureSensor::new();
        c.transducer_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = OilPressureSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
