/// aurora-svc-timeout: svc timeout
/// Phase 2572

#[derive(Debug, Clone)]
pub struct SvcTimeout {
    pub set_ok: bool,
    pub cancel_ok: bool,
    pub extend_ok: bool,
    pub report_ok: bool,
    pub alert_ok: bool,
}

impl Default for SvcTimeout {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcTimeout {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            cancel_ok: true,
            extend_ok: true,
            report_ok: true,
            alert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.cancel_ok && self.extend_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.cancel_ok
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
        let c = SvcTimeout::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcTimeout::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcTimeout::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcTimeout::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcTimeout::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcTimeout::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcTimeout::default();
        assert!(c.all_ok());
    }
}
