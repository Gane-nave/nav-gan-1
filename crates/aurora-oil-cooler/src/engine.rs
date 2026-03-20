/// Oil cooler: oil temp, flow, thermostat bypass
/// Phase 514

#[derive(Debug, Clone)]
pub struct OilCooler {
    pub oil_temp_c: f64,
    pub max_oil_temp_c: f64,
    pub flow_ok: bool,
    pub thermostat_ok: bool,
    pub leak_free: bool,
}

impl Default for OilCooler {
    fn default() -> Self {
        Self::new()
    }
}

impl OilCooler {
    pub fn new() -> Self {
        Self {
            oil_temp_c: 95.0,
            max_oil_temp_c: 130.0,
            flow_ok: true,
            thermostat_ok: true,
            leak_free: true,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.oil_temp_c < self.max_oil_temp_c
    }

    pub fn cooling_ok(&self) -> bool {
        self.temp_ok() && self.flow_ok && self.thermostat_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cooling_ok() && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.leak_free || !self.thermostat_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let c = OilCooler::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_cooling() {
        let c = OilCooler::new();
        assert!(c.cooling_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilCooler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = OilCooler::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = OilCooler::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = OilCooler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
