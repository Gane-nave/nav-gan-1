/// Oil pressure sensor: engine lubrication monitoring, low pressure warning
/// Phase 217

#[derive(Debug, Clone)]
pub struct OilPressureSensor {
    pub pressure_bar: f64,
    pub temp_c: f64,
    pub min_pressure_bar: f64,
    pub sensor_ok: bool,
    pub engine_rpm: f64,
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
            temp_c: 90.0,
            min_pressure_bar: 1.0,
            sensor_ok: true,
            engine_rpm: 2000.0,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_bar >= self.min_pressure_bar
    }

    pub fn low_pressure(&self) -> bool {
        self.pressure_bar < self.min_pressure_bar
    }

    pub fn critical_low(&self) -> bool {
        self.pressure_bar < 0.5 && self.engine_rpm > 800.0
    }

    pub fn oil_hot(&self) -> bool {
        self.temp_c > 130.0
    }

    pub fn expected_pressure(&self) -> f64 {
        (self.engine_rpm / 1000.0 * 1.5).clamp(1.0, 6.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if self.critical_low() {
            return 10.0;
        }
        if self.low_pressure() {
            return 40.0;
        }
        if self.oil_hot() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_ok() {
        let o = OilPressureSensor::new();
        assert!(o.pressure_ok());
    }

    #[test]
    fn test_not_low() {
        let o = OilPressureSensor::new();
        assert!(!o.low_pressure());
    }

    #[test]
    fn test_not_critical() {
        let o = OilPressureSensor::new();
        assert!(!o.critical_low());
    }

    #[test]
    fn test_not_hot() {
        let o = OilPressureSensor::new();
        assert!(!o.oil_hot());
    }

    #[test]
    fn test_expected() {
        let o = OilPressureSensor::new();
        assert!(o.expected_pressure() > 2.0);
    }

    #[test]
    fn test_health() {
        let o = OilPressureSensor::new();
        assert!((o.health_score() - 100.0).abs() < 0.1);
    }
}
