/// HVAC duct: distribution, damper, insulation
/// Phase 633

#[derive(Debug, Clone)]
pub struct HvacDuct {
    pub distribution_ok: bool,
    pub damper_ok: bool,
    pub insulation_ok: bool,
    pub sealed: bool,
    pub clean: bool,
}

impl Default for HvacDuct {
    fn default() -> Self {
        Self::new()
    }
}

impl HvacDuct {
    pub fn new() -> Self {
        Self {
            distribution_ok: true,
            damper_ok: true,
            insulation_ok: true,
            sealed: true,
            clean: true,
        }
    }

    pub fn airflow_ok(&self) -> bool {
        self.distribution_ok && self.damper_ok
    }

    pub fn integrity_ok(&self) -> bool {
        self.insulation_ok && self.sealed
    }

    pub fn all_ok(&self) -> bool {
        self.airflow_ok() && self.integrity_ok() && self.clean
    }

    pub fn needs_service(&self) -> bool {
        !self.damper_ok || !self.sealed
    }

    pub fn health_score(&self) -> f64 {
        if !self.sealed {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airflow() {
        let c = HvacDuct::new();
        assert!(c.airflow_ok());
    }

    #[test]
    fn test_integrity() {
        let c = HvacDuct::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HvacDuct::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HvacDuct::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_seal() {
        let mut c = HvacDuct::new();
        c.sealed = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HvacDuct::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
