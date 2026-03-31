/// aurora-qa-soak: qa soak
/// Phase 2519

#[derive(Debug, Clone)]
pub struct QaSoak {
    pub run_ok: bool,
    pub measure_ok: bool,
    pub leak_ok: bool,
    pub drift_ok: bool,
    pub report_ok: bool,
}

impl Default for QaSoak {
    fn default() -> Self {
        Self::new()
    }
}

impl QaSoak {
    pub fn new() -> Self {
        Self {
            run_ok: true,
            measure_ok: true,
            leak_ok: true,
            drift_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.run_ok && self.measure_ok && self.leak_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drift_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.run_ok || !self.measure_ok
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
        let c = QaSoak::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaSoak::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaSoak::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaSoak::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaSoak::new();
        c.run_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaSoak::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaSoak::default();
        assert!(c.all_ok());
    }
}
