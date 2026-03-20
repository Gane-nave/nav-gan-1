/// PTC heater: element, control, power, safety
/// Phase 634

#[derive(Debug, Clone)]
pub struct PtcHeater {
    pub element_ok: bool,
    pub control_ok: bool,
    pub power_ok: bool,
    pub safety_ok: bool,
    pub temp_limit_ok: bool,
}

impl Default for PtcHeater {
    fn default() -> Self {
        Self::new()
    }
}

impl PtcHeater {
    pub fn new() -> Self {
        Self {
            element_ok: true,
            control_ok: true,
            power_ok: true,
            safety_ok: true,
            temp_limit_ok: true,
        }
    }

    pub fn heating_ok(&self) -> bool {
        self.element_ok && self.power_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.safety_ok && self.temp_limit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.heating_ok() && self.protection_ok() && self.control_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.element_ok || !self.safety_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.element_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heating() {
        let c = PtcHeater::new();
        assert!(c.heating_ok());
    }

    #[test]
    fn test_protection() {
        let c = PtcHeater::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PtcHeater::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PtcHeater::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_element() {
        let mut c = PtcHeater::new();
        c.element_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PtcHeater::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
