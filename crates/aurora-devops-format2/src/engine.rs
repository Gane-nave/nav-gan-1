/// devops format2: check, fix, configure, enforce, log
/// Phase 2170

#[derive(Debug, Clone)]
pub struct DevopsFormat2 {
    pub check_ok: bool,
    pub fix_ok: bool,
    pub configure_ok: bool,
    pub enforce_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsFormat2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsFormat2 {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            fix_ok: true,
            configure_ok: true,
            enforce_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.fix_ok && self.configure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.enforce_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.fix_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = DevopsFormat2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsFormat2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsFormat2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsFormat2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsFormat2::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsFormat2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
