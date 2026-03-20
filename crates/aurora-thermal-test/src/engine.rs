/// thermal test: heat, cool, cycle, measure, log
/// Phase 1399

#[derive(Debug, Clone)]
pub struct ThermalTest {
    pub heat_ok: bool,
    pub cool_ok: bool,
    pub cycle_ok: bool,
    pub measure_ok: bool,
    pub log_ok: bool,
}

impl Default for ThermalTest {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalTest {
    pub fn new() -> Self {
        Self {
            heat_ok: true,
            cool_ok: true,
            cycle_ok: true,
            measure_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.heat_ok && self.cool_ok && self.cycle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.measure_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.heat_ok || !self.cool_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heat_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ThermalTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ThermalTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThermalTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ThermalTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ThermalTest::new();
        c.heat_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ThermalTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
