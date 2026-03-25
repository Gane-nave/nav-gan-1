/// Haptic steering wheel: vibration feedback, lane departure pulse, navigation tap
/// Phase 275

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HapticPattern {
    None,
    LaneDeparture,
    NavigationTurn,
    CollisionWarning,
    Confirmation,
}

#[derive(Debug, Clone)]
pub struct HapticWheel {
    pub pattern: HapticPattern,
    pub intensity_pct: f64,
    pub motor_ok: bool,
    pub enabled: bool,
    pub duration_ms: u32,
}

impl Default for HapticWheel {
    fn default() -> Self {
        Self::new()
    }
}

impl HapticWheel {
    pub fn new() -> Self {
        Self {
            pattern: HapticPattern::None,
            intensity_pct: 50.0,
            motor_ok: true,
            enabled: true,
            duration_ms: 0,
        }
    }

    pub fn is_vibrating(&self) -> bool {
        self.pattern != HapticPattern::None && self.motor_ok
    }

    pub fn is_warning(&self) -> bool {
        matches!(
            self.pattern,
            HapticPattern::LaneDeparture | HapticPattern::CollisionWarning
        )
    }

    pub fn can_operate(&self) -> bool {
        self.enabled && self.motor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_vibrating() {
        let h = HapticWheel::new();
        assert!(!h.is_vibrating());
    }

    #[test]
    fn test_not_warning() {
        let h = HapticWheel::new();
        assert!(!h.is_warning());
    }

    #[test]
    fn test_can_operate() {
        let h = HapticWheel::new();
        assert!(h.can_operate());
    }

    #[test]
    fn test_motor_ok() {
        let h = HapticWheel::new();
        assert!(h.motor_ok);
    }

    #[test]
    fn test_warning() {
        let mut h = HapticWheel::new();
        h.pattern = HapticPattern::CollisionWarning;
        assert!(h.is_warning());
    }

    #[test]
    fn test_health() {
        let h = HapticWheel::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
