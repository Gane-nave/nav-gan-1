/// Intercooler: charge air cooling, pressure drop, efficiency
/// Phase 504

#[derive(Debug, Clone)]
pub struct Intercooler {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub pressure_drop_kpa: f64,
    pub max_drop_kpa: f64,
    pub leak_free: bool,
}

impl Default for Intercooler {
    fn default() -> Self {
        Self::new()
    }
}

impl Intercooler {
    pub fn new() -> Self {
        Self {
            inlet_temp_c: 150.0,
            outlet_temp_c: 45.0,
            pressure_drop_kpa: 3.0,
            max_drop_kpa: 10.0,
            leak_free: true,
        }
    }

    pub fn cooling_efficiency(&self) -> f64 {
        ((self.inlet_temp_c - self.outlet_temp_c) / self.inlet_temp_c) * 100.0
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_drop_kpa < self.max_drop_kpa
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.leak_free || self.pressure_drop_kpa > self.max_drop_kpa
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling() {
        let c = Intercooler::new();
        assert!(c.cooling_efficiency() > 60.0);
    }

    #[test]
    fn test_pressure() {
        let c = Intercooler::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Intercooler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Intercooler::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = Intercooler::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Intercooler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
