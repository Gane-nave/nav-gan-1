/// crash sim: model, impact, analyze, report, log
/// Phase 1401

#[derive(Debug, Clone)]
pub struct CrashSim {
    pub model_ok: bool,
    pub impact_ok: bool,
    pub analyze_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for CrashSim {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashSim {
    pub fn new() -> Self {
        Self {
            model_ok: true,
            impact_ok: true,
            analyze_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.model_ok && self.impact_ok && self.analyze_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.model_ok || !self.impact_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.model_ok {
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
        let c = CrashSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CrashSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrashSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CrashSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CrashSim::new();
        c.model_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CrashSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
