/// A/B testing: split, assign, track, analyze, conclude
/// Phase 1078

#[derive(Debug, Clone)]
pub struct AbTest {
    pub split_ok: bool,
    pub assign_ok: bool,
    pub track_ok: bool,
    pub analyze_ok: bool,
    pub conclude_ok: bool,
}

impl Default for AbTest {
    fn default() -> Self {
        Self::new()
    }
}

impl AbTest {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            assign_ok: true,
            track_ok: true,
            analyze_ok: true,
            conclude_ok: true,
        }
    }

    pub fn experiment_ok(&self) -> bool {
        self.split_ok && self.assign_ok && self.track_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.analyze_ok && self.conclude_ok
    }

    pub fn all_ok(&self) -> bool {
        self.experiment_ok() && self.analysis_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.split_ok || !self.assign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment() {
        let c = AbTest::new();
        assert!(c.experiment_ok());
    }

    #[test]
    fn test_analysis() {
        let c = AbTest::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AbTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = AbTest::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_split() {
        let mut c = AbTest::new();
        c.split_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = AbTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
