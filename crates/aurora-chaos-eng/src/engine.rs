/// Chaos engineering: inject, observe, recover, report, schedule
/// Phase 1075

#[derive(Debug, Clone)]
pub struct ChaosEng {
    pub inject_ok: bool,
    pub observe_ok: bool,
    pub recover_ok: bool,
    pub report_ok: bool,
    pub schedule_ok: bool,
}

impl Default for ChaosEng {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosEng {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            observe_ok: true,
            recover_ok: true,
            report_ok: true,
            schedule_ok: true,
        }
    }

    pub fn experiment_ok(&self) -> bool {
        self.inject_ok && self.observe_ok && self.recover_ok
    }

    pub fn management_ok(&self) -> bool {
        self.report_ok && self.schedule_ok
    }

    pub fn all_ok(&self) -> bool {
        self.experiment_ok() && self.management_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.inject_ok || !self.recover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok {
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
        let c = ChaosEng::new();
        assert!(c.experiment_ok());
    }

    #[test]
    fn test_management() {
        let c = ChaosEng::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChaosEng::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = ChaosEng::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_inject() {
        let mut c = ChaosEng::new();
        c.inject_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = ChaosEng::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
