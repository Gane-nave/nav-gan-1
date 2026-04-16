/// Turbocharger: boost pressure, wastegate, shaft speed
/// Phase 503

#[derive(Debug, Clone)]
pub struct TurboCharger {
    pub boost_bar: f64,
    pub target_boost_bar: f64,
    pub shaft_rpm: f64,
    pub wastegate_ok: bool,
    pub oil_supply_ok: bool,
}

impl Default for TurboCharger {
    fn default() -> Self {
        Self::new()
    }
}

impl TurboCharger {
    pub fn new() -> Self {
        Self {
            boost_bar: 1.2,
            target_boost_bar: 1.2,
            shaft_rpm: 80000.0,
            wastegate_ok: true,
            oil_supply_ok: true,
        }
    }

    pub fn boost_ok(&self) -> bool {
        (self.boost_bar - self.target_boost_bar).abs() < 0.2
    }

    pub fn shaft_ok(&self) -> bool {
        self.shaft_rpm < 150000.0
    }

    pub fn all_ok(&self) -> bool {
        self.boost_ok() && self.shaft_ok() && self.wastegate_ok && self.oil_supply_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_supply_ok || !self.wastegate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.oil_supply_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost() {
        let c = TurboCharger::new();
        assert!(c.boost_ok());
    }

    #[test]
    fn test_shaft() {
        let c = TurboCharger::new();
        assert!(c.shaft_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TurboCharger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TurboCharger::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_oil_fail() {
        let mut c = TurboCharger::new();
        c.oil_supply_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TurboCharger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
