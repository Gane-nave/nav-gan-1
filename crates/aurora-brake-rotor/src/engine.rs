/// brake rotor: friction, cool, vent, measure, replace
/// Phase 1204

#[derive(Debug, Clone)]
pub struct BrakeRotor {
    pub friction_ok: bool,
    pub cool_ok: bool,
    pub vent_ok: bool,
    pub measure_ok: bool,
    pub replace_ok: bool,
}

impl Default for BrakeRotor {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeRotor {
    pub fn new() -> Self {
        Self {
            friction_ok: true,
            cool_ok: true,
            vent_ok: true,
            measure_ok: true,
            replace_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.friction_ok && self.cool_ok && self.vent_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.measure_ok && self.replace_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.friction_ok || !self.cool_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.friction_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BrakeRotor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BrakeRotor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeRotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BrakeRotor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BrakeRotor::new();
        c.friction_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BrakeRotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
