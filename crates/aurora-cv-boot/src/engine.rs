/// CV boot: constant velocity joint boot, grease retention, split detection
/// Phase 461

#[derive(Debug, Clone)]
pub struct CvBoot {
    pub intact: bool,
    pub grease_ok: bool,
    pub age_years: f64,
    pub cracked: bool,
    pub contaminated: bool,
}

impl Default for CvBoot {
    fn default() -> Self {
        Self::new()
    }
}

impl CvBoot {
    pub fn new() -> Self {
        Self {
            intact: true,
            grease_ok: true,
            age_years: 3.0,
            cracked: false,
            contaminated: false,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.intact && self.grease_ok && !self.cracked && !self.contaminated
    }

    pub fn needs_replacement(&self) -> bool {
        !self.intact || self.cracked
    }

    pub fn joint_at_risk(&self) -> bool {
        !self.grease_ok || self.contaminated
    }

    pub fn remaining_life_pct(&self) -> f64 {
        let max_age = 10.0;
        ((1.0 - self.age_years / max_age) * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if self.cracked {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let c = CvBoot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CvBoot::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_no_risk() {
        let c = CvBoot::new();
        assert!(!c.joint_at_risk());
    }

    #[test]
    fn test_life() {
        let c = CvBoot::new();
        assert!(c.remaining_life_pct() > 60.0);
    }

    #[test]
    fn test_split() {
        let mut c = CvBoot::new();
        c.intact = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CvBoot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
