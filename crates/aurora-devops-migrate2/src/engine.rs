/// devops migrate2: plan, apply, rollback, verify, log
/// Phase 2181

#[derive(Debug, Clone)]
pub struct DevopsMigrate2 {
    pub plan_ok: bool,
    pub apply_ok: bool,
    pub rollback_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsMigrate2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsMigrate2 {
    pub fn new() -> Self {
        Self {
            plan_ok: true,
            apply_ok: true,
            rollback_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.plan_ok && self.apply_ok && self.rollback_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.plan_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.plan_ok {
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
        let c = DevopsMigrate2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsMigrate2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsMigrate2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsMigrate2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsMigrate2::new();
        c.plan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsMigrate2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
