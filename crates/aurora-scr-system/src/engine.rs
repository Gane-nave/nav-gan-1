/// SCR system: selective catalytic reduction, urea inject
/// Phase 496

#[derive(Debug, Clone)]
pub struct ScrSystem {
    pub nox_reduction_pct: f64,
    pub urea_level_pct: f64,
    pub dosing_ok: bool,
    pub catalyst_ok: bool,
    pub heater_ok: bool,
}

impl Default for ScrSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrSystem {
    pub fn new() -> Self {
        Self {
            nox_reduction_pct: 90.0,
            urea_level_pct: 75.0,
            dosing_ok: true,
            catalyst_ok: true,
            heater_ok: true,
        }
    }

    pub fn reduction_ok(&self) -> bool {
        self.nox_reduction_pct > 70.0
    }

    pub fn urea_ok(&self) -> bool {
        self.urea_level_pct > 10.0
    }

    pub fn all_ok(&self) -> bool {
        self.reduction_ok() && self.urea_ok() && self.dosing_ok && self.catalyst_ok
    }

    pub fn needs_refill(&self) -> bool {
        self.urea_level_pct < 15.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.catalyst_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduction() {
        let c = ScrSystem::new();
        assert!(c.reduction_ok());
    }

    #[test]
    fn test_urea() {
        let c = ScrSystem::new();
        assert!(c.urea_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ScrSystem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refill() {
        let c = ScrSystem::new();
        assert!(!c.needs_refill());
    }

    #[test]
    fn test_low_urea() {
        let mut c = ScrSystem::new();
        c.urea_level_pct = 5.0;
        assert!(c.needs_refill());
    }

    #[test]
    fn test_health() {
        let c = ScrSystem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
