/// aurora-qa-stress: qa stress
/// Phase 2518

#[derive(Debug, Clone)]
pub struct QaStress {
    pub ramp_ok: bool,
    pub saturate_ok: bool,
    pub recover_ok: bool,
    pub measure_ok: bool,
    pub report_ok: bool,
}

impl Default for QaStress {
    fn default() -> Self {
        Self::new()
    }
}

impl QaStress {
    pub fn new() -> Self {
        Self {
            ramp_ok: true,
            saturate_ok: true,
            recover_ok: true,
            measure_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ramp_ok && self.saturate_ok && self.recover_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.measure_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ramp_ok || !self.saturate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ramp_ok {
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
        let c = QaStress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaStress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaStress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaStress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaStress::new();
        c.ramp_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaStress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaStress::default();
        assert!(c.all_ok());
    }
}
