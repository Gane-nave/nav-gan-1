/// feature toggle: check, enable, disable, list, log
/// Phase 1780

#[derive(Debug, Clone)]
pub struct FeatureToggle {
    pub check_ok: bool,
    pub enable_ok: bool,
    pub disable_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureToggle {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureToggle {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            enable_ok: true,
            disable_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.enable_ok && self.disable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.enable_ok
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
        let c = FeatureToggle::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureToggle::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureToggle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureToggle::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureToggle::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureToggle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
