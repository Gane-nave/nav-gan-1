/// feature override: set, clear, list, audit, log
/// Phase 1784

#[derive(Debug, Clone)]
pub struct FeatureOverride {
    pub set_ok: bool,
    pub clear_ok: bool,
    pub list_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureOverride {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureOverride {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            clear_ok: true,
            list_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.clear_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.clear_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = FeatureOverride::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureOverride::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureOverride::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureOverride::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureOverride::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureOverride::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
