/// ml pipeline: orchestrate, schedule, retry, report, log
/// Phase 1471

#[derive(Debug, Clone)]
pub struct MlPipeline2 {
    pub orchestrate_ok: bool,
    pub schedule_ok: bool,
    pub retry_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for MlPipeline2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MlPipeline2 {
    pub fn new() -> Self {
        Self {
            orchestrate_ok: true,
            schedule_ok: true,
            retry_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.orchestrate_ok && self.schedule_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.orchestrate_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.orchestrate_ok {
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
        let c = MlPipeline2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlPipeline2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlPipeline2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlPipeline2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlPipeline2::new();
        c.orchestrate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlPipeline2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
