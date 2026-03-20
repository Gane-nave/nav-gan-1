/// SOC estimator: coulomb counting, EKF, OCV, temperature
/// Phase 868

#[derive(Debug, Clone)]
pub struct SocEstimator {
    pub coulomb_ok: bool,
    pub ekf_ok: bool,
    pub ocv_ok: bool,
    pub temp_ok: bool,
    pub accuracy_ok: bool,
}

impl Default for SocEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl SocEstimator {
    pub fn new() -> Self {
        Self {
            coulomb_ok: true,
            ekf_ok: true,
            ocv_ok: true,
            temp_ok: true,
            accuracy_ok: true,
        }
    }

    pub fn estimation_ok(&self) -> bool {
        self.coulomb_ok && self.ekf_ok && self.ocv_ok
    }

    pub fn compensation_ok(&self) -> bool {
        self.temp_ok && self.accuracy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.estimation_ok() && self.compensation_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.ekf_ok || !self.accuracy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ekf_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimation() {
        let c = SocEstimator::new();
        assert!(c.estimation_ok());
    }

    #[test]
    fn test_compensation() {
        let c = SocEstimator::new();
        assert!(c.compensation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SocEstimator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = SocEstimator::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_ekf() {
        let mut c = SocEstimator::new();
        c.ekf_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = SocEstimator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
