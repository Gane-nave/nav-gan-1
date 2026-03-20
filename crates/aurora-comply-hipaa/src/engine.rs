/// comply hipaa: assess, implement, train, audit, log
/// Phase 1493

#[derive(Debug, Clone)]
pub struct ComplyHipaa {
    pub assess_ok: bool,
    pub implement_ok: bool,
    pub train_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyHipaa {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyHipaa {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            implement_ok: true,
            train_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.implement_ok && self.train_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.implement_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assess_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ComplyHipaa::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyHipaa::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyHipaa::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyHipaa::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyHipaa::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyHipaa::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
