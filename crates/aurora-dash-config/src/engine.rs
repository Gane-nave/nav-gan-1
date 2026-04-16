/// aurora-dash-config: dash config
/// Phase 2445

#[derive(Debug, Clone)]
pub struct DashConfig {
    pub load_ok: bool,
    pub save_ok: bool,
    pub validate_ok: bool,
    pub reset_ok: bool,
    pub export_ok: bool,
}

impl Default for DashConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DashConfig {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            save_ok: true,
            validate_ok: true,
            reset_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.save_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.save_ok
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
        let c = DashConfig::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashConfig::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashConfig::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashConfig::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashConfig::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashConfig::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashConfig::default();
        assert!(c.all_ok());
    }
}
