/// Cost optimization: analyze, recommend, implement, track, report
/// Phase 1079

#[derive(Debug, Clone)]
pub struct CostOptim {
    pub analyze_ok: bool,
    pub recommend_ok: bool,
    pub implement_ok: bool,
    pub track_ok: bool,
    pub report_ok: bool,
}

impl Default for CostOptim {
    fn default() -> Self {
        Self::new()
    }
}

impl CostOptim {
    pub fn new() -> Self {
        Self {
            analyze_ok: true,
            recommend_ok: true,
            implement_ok: true,
            track_ok: true,
            report_ok: true,
        }
    }

    pub fn optimization_ok(&self) -> bool {
        self.analyze_ok && self.recommend_ok && self.implement_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.track_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.optimization_ok() && self.monitoring_ok()
    }

    pub fn needs_analysis(&self) -> bool {
        !self.analyze_ok || !self.recommend_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.analyze_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization() {
        let c = CostOptim::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = CostOptim::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CostOptim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_analysis() {
        let c = CostOptim::new();
        assert!(!c.needs_analysis());
    }

    #[test]
    fn test_analyze() {
        let mut c = CostOptim::new();
        c.analyze_ok = false;
        assert!(c.needs_analysis());
    }

    #[test]
    fn test_health() {
        let c = CostOptim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
