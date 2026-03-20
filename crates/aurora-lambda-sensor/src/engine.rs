/// Lambda sensor: wideband, heater, response time
/// Phase 588

#[derive(Debug, Clone)]
pub struct LambdaSensor {
    pub wideband_ok: bool,
    pub heater_ok: bool,
    pub response_ms: f64,
    pub max_response_ms: f64,
    pub signal_ok: bool,
}

impl Default for LambdaSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaSensor {
    pub fn new() -> Self {
        Self {
            wideband_ok: true,
            heater_ok: true,
            response_ms: 50.0,
            max_response_ms: 200.0,
            signal_ok: true,
        }
    }

    pub fn response_ok(&self) -> bool {
        self.response_ms < self.max_response_ms
    }

    pub fn heater_good(&self) -> bool {
        self.heater_ok && self.wideband_ok
    }

    pub fn all_ok(&self) -> bool {
        self.response_ok() && self.heater_good() && self.signal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.wideband_ok || !self.heater_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wideband_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let c = LambdaSensor::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_heater() {
        let c = LambdaSensor::new();
        assert!(c.heater_good());
    }

    #[test]
    fn test_all_ok() {
        let c = LambdaSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = LambdaSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_wideband() {
        let mut c = LambdaSensor::new();
        c.wideband_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = LambdaSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
