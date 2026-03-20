/// analytics cohort: define, segment, analyze, compare, log
/// Phase 1557

#[derive(Debug, Clone)]
pub struct AnalyticsCohort {
    pub define_ok: bool,
    pub segment_ok: bool,
    pub analyze_ok: bool,
    pub compare_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsCohort {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsCohort {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            segment_ok: true,
            analyze_ok: true,
            compare_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.segment_ok && self.analyze_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.segment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = AnalyticsCohort::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsCohort::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsCohort::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsCohort::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsCohort::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsCohort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
