/// starter motor: crank, engage, disengage, protect, check
/// Phase 1224

#[derive(Debug, Clone)]
pub struct StarterMotor {
    pub crank_ok: bool,
    pub engage_ok: bool,
    pub disengage_ok: bool,
    pub protect_ok: bool,
    pub check_ok: bool,
}

impl Default for StarterMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl StarterMotor {
    pub fn new() -> Self {
        Self {
            crank_ok: true,
            engage_ok: true,
            disengage_ok: true,
            protect_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.crank_ok && self.engage_ok && self.disengage_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.crank_ok || !self.engage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.crank_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = StarterMotor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StarterMotor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StarterMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StarterMotor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StarterMotor::new();
        c.crank_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StarterMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
