/// devops config3: load, validate, apply, rollback, log
/// Phase 2179

#[derive(Debug, Clone)]
pub struct DevopsConfig3 {
    pub load_ok: bool,
    pub validate_ok: bool,
    pub apply_ok: bool,
    pub rollback_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsConfig3 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsConfig3 {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            validate_ok: true,
            apply_ok: true,
            rollback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.validate_ok && self.apply_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = DevopsConfig3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsConfig3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsConfig3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsConfig3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsConfig3::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsConfig3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
