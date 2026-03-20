/// Catalytic converter: catalyst temp, efficiency, light-off
/// Phase 492

#[derive(Debug, Clone)]
pub struct CatalyticConverter {
    pub catalyst_temp_c: f64,
    pub light_off_temp_c: f64,
    pub efficiency_pct: f64,
    pub substrate_ok: bool,
    pub poisoned: bool,
}

impl Default for CatalyticConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalyticConverter {
    pub fn new() -> Self {
        Self {
            catalyst_temp_c: 450.0,
            light_off_temp_c: 300.0,
            efficiency_pct: 95.0,
            substrate_ok: true,
            poisoned: false,
        }
    }

    pub fn is_lit_off(&self) -> bool {
        self.catalyst_temp_c >= self.light_off_temp_c
    }

    pub fn efficient(&self) -> bool {
        self.efficiency_pct > 80.0
    }

    pub fn all_ok(&self) -> bool {
        self.is_lit_off() && self.efficient() && self.substrate_ok && !self.poisoned
    }

    pub fn needs_replacement(&self) -> bool {
        self.poisoned || self.efficiency_pct < 60.0
    }

    pub fn health_score(&self) -> f64 {
        if self.poisoned { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit_off() {
        let c = CatalyticConverter::new();
        assert!(c.is_lit_off());
    }

    #[test]
    fn test_efficient() {
        let c = CatalyticConverter::new();
        assert!(c.efficient());
    }

    #[test]
    fn test_all_ok() {
        let c = CatalyticConverter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CatalyticConverter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_poisoned() {
        let mut c = CatalyticConverter::new();
        c.poisoned = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CatalyticConverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
