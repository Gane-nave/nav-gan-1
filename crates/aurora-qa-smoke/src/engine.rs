/// aurora-qa-smoke: qa smoke
/// Phase 2520

#[derive(Debug, Clone)]
pub struct QaSmoke {
    pub run_ok: bool,
    pub verify_ok: bool,
    pub report_ok: bool,
    pub gate_ok: bool,
    pub notify_ok: bool,
}

impl Default for QaSmoke {
    fn default() -> Self {
        Self::new()
    }
}

impl QaSmoke {
    pub fn new() -> Self {
        Self {
            run_ok: true,
            verify_ok: true,
            report_ok: true,
            gate_ok: true,
            notify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.run_ok && self.verify_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.gate_ok && self.notify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.run_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.run_ok {
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
        let c = QaSmoke::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaSmoke::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaSmoke::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaSmoke::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaSmoke::new();
        c.run_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaSmoke::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaSmoke::default();
        assert!(c.all_ok());
    }
}
