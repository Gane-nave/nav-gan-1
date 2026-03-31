/// aurora-assert-nav: assert nav
/// Phase 2504

#[derive(Debug, Clone)]
pub struct AssertNav {
    pub heading_ok: bool,
    pub speed_ok: bool,
    pub altitude_ok: bool,
    pub accuracy_ok: bool,
    pub fix_ok: bool,
}

impl Default for AssertNav {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertNav {
    pub fn new() -> Self {
        Self {
            heading_ok: true,
            speed_ok: true,
            altitude_ok: true,
            accuracy_ok: true,
            fix_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.heading_ok && self.speed_ok && self.altitude_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.accuracy_ok && self.fix_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.heading_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heading_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AssertNav::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertNav::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertNav::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertNav::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertNav::new();
        c.heading_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertNav::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertNav::default();
        assert!(c.all_ok());
    }
}
