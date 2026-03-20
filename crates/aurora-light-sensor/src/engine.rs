/// Ambient light sensor: photodiode, range, auto headlights
/// Phase 698

#[derive(Debug, Clone)]
pub struct LightSensor {
    pub photodiode_ok: bool,
    pub range_ok: bool,
    pub auto_ok: bool,
    pub connector_ok: bool,
    pub calibrated: bool,
}

impl Default for LightSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl LightSensor {
    pub fn new() -> Self {
        Self {
            photodiode_ok: true,
            range_ok: true,
            auto_ok: true,
            connector_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.photodiode_ok && self.range_ok
    }

    pub fn automation_ok(&self) -> bool {
        self.auto_ok && self.calibrated && self.connector_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.automation_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.photodiode_ok || !self.range_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.photodiode_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = LightSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_automation() {
        let c = LightSensor::new();
        assert!(c.automation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LightSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = LightSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_photodiode() {
        let mut c = LightSensor::new();
        c.photodiode_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = LightSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
