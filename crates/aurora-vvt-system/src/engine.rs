/// Variable valve timing: cam phasing, valve overlap, VTEC-style switching
/// Phase 211

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VvtMode {
    Economy,
    Normal,
    Performance,
}

#[derive(Debug, Clone)]
pub struct VvtSystem {
    pub mode: VvtMode,
    pub intake_advance_deg: f64,
    pub exhaust_retard_deg: f64,
    pub target_intake_deg: f64,
    pub oil_pressure_ok: bool,
    pub solenoid_duty_pct: f64,
}

impl Default for VvtSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl VvtSystem {
    pub fn new() -> Self {
        Self {
            mode: VvtMode::Normal,
            intake_advance_deg: 15.0,
            exhaust_retard_deg: 5.0,
            target_intake_deg: 15.0,
            oil_pressure_ok: true,
            solenoid_duty_pct: 50.0,
        }
    }

    pub fn position_error_deg(&self) -> f64 {
        (self.intake_advance_deg - self.target_intake_deg).abs()
    }

    pub fn at_target(&self) -> bool {
        self.position_error_deg() < 2.0
    }

    pub fn overlap_deg(&self) -> f64 {
        self.intake_advance_deg + self.exhaust_retard_deg
    }

    pub fn can_operate(&self) -> bool {
        self.oil_pressure_ok
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.oil_pressure_ok {
            score -= 50.0;
        }
        if !self.at_target() {
            score -= 25.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let v = VvtSystem::new();
        assert!(v.at_target());
    }

    #[test]
    fn test_can_operate() {
        let v = VvtSystem::new();
        assert!(v.can_operate());
    }

    #[test]
    fn test_overlap() {
        let v = VvtSystem::new();
        assert!((v.overlap_deg() - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_no_oil_pressure() {
        let mut v = VvtSystem::new();
        v.oil_pressure_ok = false;
        assert!(!v.can_operate());
    }

    #[test]
    fn test_position_error() {
        let v = VvtSystem::new();
        assert!(v.position_error_deg() < 0.1);
    }

    #[test]
    fn test_health() {
        let v = VvtSystem::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
