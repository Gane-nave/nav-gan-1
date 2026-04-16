/// Battery test: CCA, voltage, load, conductance
/// Phase 822

#[derive(Debug, Clone)]
pub struct BatteryTest {
    pub cca_ok: bool,
    pub voltage_ok: bool,
    pub load_ok: bool,
    pub conductance_ok: bool,
    pub age_ok: bool,
}

impl Default for BatteryTest {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryTest {
    pub fn new() -> Self {
        Self {
            cca_ok: true,
            voltage_ok: true,
            load_ok: true,
            conductance_ok: true,
            age_ok: true,
        }
    }

    pub fn power_ok(&self) -> bool {
        self.cca_ok && self.voltage_ok && self.load_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.conductance_ok && self.age_ok
    }

    pub fn all_ok(&self) -> bool {
        self.power_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.cca_ok || !self.voltage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cca_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let c = BatteryTest::new();
        assert!(c.power_ok());
    }

    #[test]
    fn test_condition() {
        let c = BatteryTest::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BatteryTest::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cca() {
        let mut c = BatteryTest::new();
        c.cca_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BatteryTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
