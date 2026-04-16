/// aurora-svc-retry: svc retry
/// Phase 2571

#[derive(Debug, Clone)]
pub struct SvcRetry {
    pub attempt_ok: bool,
    pub backoff_ok: bool,
    pub jitter_ok: bool,
    pub limit_ok: bool,
    pub report_ok: bool,
}

impl Default for SvcRetry {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcRetry {
    pub fn new() -> Self {
        Self {
            attempt_ok: true,
            backoff_ok: true,
            jitter_ok: true,
            limit_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.attempt_ok && self.backoff_ok && self.jitter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.limit_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.attempt_ok || !self.backoff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.attempt_ok {
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
        let c = SvcRetry::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcRetry::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcRetry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcRetry::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcRetry::new();
        c.attempt_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcRetry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcRetry::default();
        assert!(c.all_ok());
    }
}
