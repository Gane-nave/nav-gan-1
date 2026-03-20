/// Oxygen sensor: lambda, wideband, heater circuit
/// Phase 498

#[derive(Debug, Clone)]
pub struct OxygenSensor {
    pub lambda_value: f64,
    pub heater_ok: bool,
    pub response_ms: f64,
    pub max_response_ms: f64,
    pub aged: bool,
}

impl Default for OxygenSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl OxygenSensor {
    pub fn new() -> Self {
        Self {
            lambda_value: 1.0,
            heater_ok: true,
            response_ms: 50.0,
            max_response_ms: 150.0,
            aged: false,
        }
    }

    pub fn stoichiometric(&self) -> bool {
        (self.lambda_value - 1.0).abs() < 0.05
    }

    pub fn response_ok(&self) -> bool {
        self.response_ms < self.max_response_ms
    }

    pub fn all_ok(&self) -> bool {
        self.stoichiometric() && self.response_ok() && self.heater_ok && !self.aged
    }

    pub fn needs_replacement(&self) -> bool {
        self.aged || !self.heater_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.aged { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stoich() {
        let c = OxygenSensor::new();
        assert!(c.stoichiometric());
    }

    #[test]
    fn test_response() {
        let c = OxygenSensor::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OxygenSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = OxygenSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_aged() {
        let mut c = OxygenSensor::new();
        c.aged = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = OxygenSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
