/// ux onboard: welcome, configure, teach, complete, log
/// Phase 1508

#[derive(Debug, Clone)]
pub struct UxOnboard {
    pub welcome_ok: bool,
    pub configure_ok: bool,
    pub teach_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for UxOnboard {
    fn default() -> Self {
        Self::new()
    }
}

impl UxOnboard {
    pub fn new() -> Self {
        Self {
            welcome_ok: true,
            configure_ok: true,
            teach_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.welcome_ok && self.configure_ok && self.teach_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.welcome_ok || !self.configure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.welcome_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxOnboard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxOnboard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxOnboard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxOnboard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxOnboard::new();
        c.welcome_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxOnboard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
