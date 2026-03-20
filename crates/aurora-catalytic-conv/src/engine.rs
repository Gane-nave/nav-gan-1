/// Catalytic converter: temperature monitoring, efficiency, light-off detection
/// Phase 303

#[derive(Debug, Clone)]
pub struct CatalyticConverter {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub efficiency_pct: f64,
    pub light_off: bool,
    pub substrate_ok: bool,
}

impl Default for CatalyticConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalyticConverter {
    pub fn new() -> Self {
        Self {
            inlet_temp_c: 400.0,
            outlet_temp_c: 450.0,
            efficiency_pct: 95.0,
            light_off: true,
            substrate_ok: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.light_off && self.inlet_temp_c > 250.0
    }

    pub fn efficient(&self) -> bool {
        self.efficiency_pct > 85.0
    }

    pub fn overheating(&self) -> bool {
        self.inlet_temp_c > 900.0 || self.outlet_temp_c > 1000.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.substrate_ok || self.efficiency_pct < 70.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.substrate_ok {
            return 0.0;
        }
        if !self.efficient() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let c = CatalyticConverter::new();
        assert!(c.is_active());
    }

    #[test]
    fn test_efficient() {
        let c = CatalyticConverter::new();
        assert!(c.efficient());
    }

    #[test]
    fn test_not_hot() {
        let c = CatalyticConverter::new();
        assert!(!c.overheating());
    }

    #[test]
    fn test_no_replace() {
        let c = CatalyticConverter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_degraded() {
        let mut c = CatalyticConverter::new();
        c.efficiency_pct = 60.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CatalyticConverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
