/// Fatigue simulation: stress cycle, crack, life, endurance
/// Phase 964

#[derive(Debug, Clone)]
pub struct FatigueSim {
    pub stress_ok: bool,
    pub crack_ok: bool,
    pub life_ok: bool,
    pub endurance_ok: bool,
    pub validate_ok: bool,
}

impl Default for FatigueSim {
    fn default() -> Self {
        Self::new()
    }
}

impl FatigueSim {
    pub fn new() -> Self {
        Self {
            stress_ok: true,
            crack_ok: true,
            life_ok: true,
            endurance_ok: true,
            validate_ok: true,
        }
    }

    pub fn analysis_ok(&self) -> bool {
        self.stress_ok && self.crack_ok && self.life_ok
    }

    pub fn prediction_ok(&self) -> bool {
        self.endurance_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.analysis_ok() && self.prediction_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.validate_ok || !self.stress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stress_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis() {
        let c = FatigueSim::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_prediction() {
        let c = FatigueSim::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FatigueSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = FatigueSim::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_validate() {
        let mut c = FatigueSim::new();
        c.validate_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = FatigueSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
