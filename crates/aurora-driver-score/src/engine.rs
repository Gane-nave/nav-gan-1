/// Driver scoring: collect, analyze, score, feedback, trend
/// Phase 1095

#[derive(Debug, Clone)]
pub struct DriverScore {
    pub collect_ok: bool,
    pub analyze_ok: bool,
    pub score_ok: bool,
    pub feedback_ok: bool,
    pub trend_ok: bool,
}

impl Default for DriverScore {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverScore {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            analyze_ok: true,
            score_ok: true,
            feedback_ok: true,
            trend_ok: true,
        }
    }

    pub fn assessment_ok(&self) -> bool {
        self.collect_ok && self.analyze_ok && self.score_ok
    }

    pub fn improvement_ok(&self) -> bool {
        self.feedback_ok && self.trend_ok
    }

    pub fn all_ok(&self) -> bool {
        self.assessment_ok() && self.improvement_ok()
    }

    pub fn needs_data(&self) -> bool {
        !self.collect_ok || !self.analyze_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessment() {
        let c = DriverScore::new();
        assert!(c.assessment_ok());
    }

    #[test]
    fn test_improvement() {
        let c = DriverScore::new();
        assert!(c.improvement_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DriverScore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_data() {
        let c = DriverScore::new();
        assert!(!c.needs_data());
    }

    #[test]
    fn test_collect() {
        let mut c = DriverScore::new();
        c.collect_ok = false;
        assert!(c.needs_data());
    }

    #[test]
    fn test_health() {
        let c = DriverScore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
