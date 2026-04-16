/// aurora-svc-config: svc config
/// Phase 2567

#[derive(Debug, Clone)]
pub struct SvcConfig {
    pub load_ok: bool,
    pub push_ok: bool,
    pub watch_ok: bool,
    pub rollback_ok: bool,
    pub validate_ok: bool,
}

impl Default for SvcConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcConfig {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            push_ok: true,
            watch_ok: true,
            rollback_ok: true,
            validate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.push_ok && self.watch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.push_ok
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
        let c = SvcConfig::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcConfig::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcConfig::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcConfig::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcConfig::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcConfig::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcConfig::default();
        assert!(c.all_ok());
    }
}
