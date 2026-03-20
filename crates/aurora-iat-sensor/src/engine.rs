/// Intake air temp sensor: NTC, response, accuracy
/// Phase 590

#[derive(Debug, Clone)]
pub struct IatSensor {
    pub temp_c: f64,
    pub ntc_ok: bool,
    pub response_ok: bool,
    pub accurate: bool,
    pub wiring_ok: bool,
}

impl Default for IatSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl IatSensor {
    pub fn new() -> Self {
        Self {
            temp_c: 25.0,
            ntc_ok: true,
            response_ok: true,
            accurate: true,
            wiring_ok: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.ntc_ok && self.accurate
    }

    pub fn system_ok(&self) -> bool {
        self.reading_ok() && self.response_ok && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.ntc_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ntc_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = IatSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_system() {
        let c = IatSensor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IatSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = IatSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_ntc() {
        let mut c = IatSensor::new();
        c.ntc_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = IatSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
