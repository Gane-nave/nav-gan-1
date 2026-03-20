/// NOx sensor: electrochemical, heater, calibration
/// Phase 596

#[derive(Debug, Clone)]
pub struct NoxSensor {
    pub nox_ppm: f64,
    pub max_nox_ppm: f64,
    pub heater_ok: bool,
    pub cell_ok: bool,
    pub calibrated: bool,
}

impl Default for NoxSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl NoxSensor {
    pub fn new() -> Self {
        Self {
            nox_ppm: 50.0,
            max_nox_ppm: 200.0,
            heater_ok: true,
            cell_ok: true,
            calibrated: true,
        }
    }

    pub fn emission_ok(&self) -> bool {
        self.nox_ppm < self.max_nox_ppm
    }

    pub fn sensor_ok(&self) -> bool {
        self.heater_ok && self.cell_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.emission_ok() && self.sensor_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.cell_ok || !self.heater_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cell_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emission() {
        let c = NoxSensor::new();
        assert!(c.emission_ok());
    }

    #[test]
    fn test_sensor() {
        let c = NoxSensor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NoxSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = NoxSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cell() {
        let mut c = NoxSensor::new();
        c.cell_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = NoxSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
