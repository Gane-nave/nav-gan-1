/// Exhaust hanger: rubber isolator, bracket, rod, bushing
/// Phase 801

#[derive(Debug, Clone)]
pub struct ExhaustHanger {
    pub isolator_ok: bool,
    pub bracket_ok: bool,
    pub rod_ok: bool,
    pub bushing_ok: bool,
    pub clearance_ok: bool,
}

impl Default for ExhaustHanger {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustHanger {
    pub fn new() -> Self {
        Self {
            isolator_ok: true,
            bracket_ok: true,
            rod_ok: true,
            bushing_ok: true,
            clearance_ok: true,
        }
    }

    pub fn suspension_ok(&self) -> bool {
        self.isolator_ok && self.bushing_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.rod_ok && self.clearance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.suspension_ok() && self.mounting_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.isolator_ok || !self.bushing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.isolator_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suspension() {
        let c = ExhaustHanger::new();
        assert!(c.suspension_ok());
    }

    #[test]
    fn test_mounting() {
        let c = ExhaustHanger::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ExhaustHanger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ExhaustHanger::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_isolator() {
        let mut c = ExhaustHanger::new();
        c.isolator_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ExhaustHanger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
