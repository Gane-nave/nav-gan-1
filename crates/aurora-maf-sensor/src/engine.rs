/// MAF sensor: mass air flow measurement, air density, fuel calculation
/// Phase 215

#[derive(Debug, Clone)]
pub struct MafSensor {
    pub flow_gps: f64,
    pub voltage: f64,
    pub air_temp_c: f64,
    pub sensor_ok: bool,
    pub contamination_pct: f64,
}

impl Default for MafSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MafSensor {
    pub fn new() -> Self {
        Self {
            flow_gps: 15.0,
            voltage: 2.5,
            air_temp_c: 25.0,
            sensor_ok: true,
            contamination_pct: 5.0,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_gps > 2.0 && self.flow_gps < 300.0
    }

    pub fn air_density_factor(&self) -> f64 {
        273.15 / (self.air_temp_c + 273.15)
    }

    pub fn corrected_flow(&self) -> f64 {
        self.flow_gps * self.air_density_factor()
    }

    pub fn needs_cleaning(&self) -> bool {
        self.contamination_pct > 20.0
    }

    pub fn voltage_ok(&self) -> bool {
        self.voltage > 0.2 && self.voltage < 4.9
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.sensor_ok {
            score -= 50.0;
        }
        if self.needs_cleaning() {
            score -= 25.0;
        }
        if !self.voltage_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_ok() {
        let m = MafSensor::new();
        assert!(m.flow_ok());
    }

    #[test]
    fn test_density_factor() {
        let m = MafSensor::new();
        assert!(m.air_density_factor() < 1.0);
    }

    #[test]
    fn test_corrected_flow() {
        let m = MafSensor::new();
        assert!(m.corrected_flow() < m.flow_gps);
    }

    #[test]
    fn test_no_cleaning() {
        let m = MafSensor::new();
        assert!(!m.needs_cleaning());
    }

    #[test]
    fn test_voltage_ok() {
        let m = MafSensor::new();
        assert!(m.voltage_ok());
    }

    #[test]
    fn test_health() {
        let m = MafSensor::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
