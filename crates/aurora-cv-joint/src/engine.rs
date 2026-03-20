/// CV joint monitoring: boot condition, grease, clicking detection
/// Phase 197

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BootCondition {
    Good,
    Worn,
    Cracked,
    Torn,
}

#[derive(Debug, Clone)]
pub struct CvJoint {
    pub side: String,
    pub boot_condition: BootCondition,
    pub clicking_detected: bool,
    pub vibration_g: f64,
    pub grease_level_pct: f64,
    pub mileage_km: f64,
}

impl Default for CvJoint {
    fn default() -> Self {
        Self::new()
    }
}

impl CvJoint {
    pub fn new() -> Self {
        Self {
            side: "left".into(),
            boot_condition: BootCondition::Good,
            clicking_detected: false,
            vibration_g: 0.02,
            grease_level_pct: 90.0,
            mileage_km: 50000.0,
        }
    }

    pub fn boot_ok(&self) -> bool {
        self.boot_condition == BootCondition::Good
    }

    pub fn needs_replacement(&self) -> bool {
        self.boot_condition == BootCondition::Torn || self.clicking_detected
    }

    pub fn grease_ok(&self) -> bool {
        self.grease_level_pct > 30.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        match self.boot_condition {
            BootCondition::Good => {}
            BootCondition::Worn => score -= 15.0,
            BootCondition::Cracked => score -= 35.0,
            BootCondition::Torn => score -= 60.0,
        }
        if self.clicking_detected {
            score -= 30.0;
        }
        if !self.grease_ok() {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_ok() {
        let c = CvJoint::new();
        assert!(c.boot_ok());
    }

    #[test]
    fn test_no_replacement() {
        let c = CvJoint::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_grease_ok() {
        let c = CvJoint::new();
        assert!(c.grease_ok());
    }

    #[test]
    fn test_clicking() {
        let mut c = CvJoint::new();
        c.clicking_detected = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_torn_boot() {
        let mut c = CvJoint::new();
        c.boot_condition = BootCondition::Torn;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CvJoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
