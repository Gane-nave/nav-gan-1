/// LED headlight: driver, thermal, beam pattern, DRL
/// Phase 672

#[derive(Debug, Clone)]
pub struct HeadlightLed {
    pub driver_ok: bool,
    pub thermal_ok: bool,
    pub beam_ok: bool,
    pub drl_ok: bool,
    pub aligned: bool,
}

impl Default for HeadlightLed {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlightLed {
    pub fn new() -> Self {
        Self {
            driver_ok: true,
            thermal_ok: true,
            beam_ok: true,
            drl_ok: true,
            aligned: true,
        }
    }

    pub fn electronics_ok(&self) -> bool {
        self.driver_ok && self.thermal_ok
    }

    pub fn optics_ok(&self) -> bool {
        self.beam_ok && self.aligned
    }

    pub fn all_ok(&self) -> bool {
        self.electronics_ok() && self.optics_ok() && self.drl_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.driver_ok || !self.aligned
    }

    pub fn health_score(&self) -> f64 {
        if !self.driver_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electronics() {
        let c = HeadlightLed::new();
        assert!(c.electronics_ok());
    }

    #[test]
    fn test_optics() {
        let c = HeadlightLed::new();
        assert!(c.optics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeadlightLed::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HeadlightLed::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_driver() {
        let mut c = HeadlightLed::new();
        c.driver_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HeadlightLed::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
