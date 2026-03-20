/// Fuel injector: spray pattern, flow rate, resistance
/// Phase 606

#[derive(Debug, Clone)]
pub struct Injector {
    pub spray_ok: bool,
    pub flow_ok: bool,
    pub resistance_ok: bool,
    pub seal_ok: bool,
    pub clean: bool,
}

impl Default for Injector {
    fn default() -> Self {
        Self::new()
    }
}

impl Injector {
    pub fn new() -> Self {
        Self {
            spray_ok: true,
            flow_ok: true,
            resistance_ok: true,
            seal_ok: true,
            clean: true,
        }
    }

    pub fn injection_ok(&self) -> bool {
        self.spray_ok && self.flow_ok
    }

    pub fn electrical_ok(&self) -> bool {
        self.resistance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.injection_ok() && self.electrical_ok() && self.seal_ok && self.clean
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.clean || !self.spray_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spray_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injection() {
        let c = Injector::new();
        assert!(c.injection_ok());
    }

    #[test]
    fn test_electrical() {
        let c = Injector::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Injector::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = Injector::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_dirty() {
        let mut c = Injector::new();
        c.clean = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = Injector::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
