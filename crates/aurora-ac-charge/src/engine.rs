/// AC charging: Level 1/2, onboard charger, J1772/Type 2
/// Phase 291

#[derive(Debug, Clone)]
pub struct AcCharge {
    pub power_kw: f64,
    pub max_power_kw: f64,
    pub phase_count: u8,
    pub voltage_v: f64,
    pub pilot_signal_ok: bool,
    pub grounding_ok: bool,
}

impl Default for AcCharge {
    fn default() -> Self {
        Self::new()
    }
}

impl AcCharge {
    pub fn new() -> Self {
        Self {
            power_kw: 0.0,
            max_power_kw: 11.0,
            phase_count: 3,
            voltage_v: 230.0,
            pilot_signal_ok: true,
            grounding_ok: true,
        }
    }

    pub fn is_charging(&self) -> bool {
        self.power_kw > 0.5
    }

    pub fn three_phase(&self) -> bool {
        self.phase_count == 3
    }

    pub fn safe_to_charge(&self) -> bool {
        self.pilot_signal_ok && self.grounding_ok
    }

    pub fn level2(&self) -> bool {
        self.voltage_v >= 200.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.grounding_ok {
            return 0.0;
        }
        if !self.pilot_signal_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_charging() {
        let a = AcCharge::new();
        assert!(!a.is_charging());
    }

    #[test]
    fn test_three_phase() {
        let a = AcCharge::new();
        assert!(a.three_phase());
    }

    #[test]
    fn test_safe() {
        let a = AcCharge::new();
        assert!(a.safe_to_charge());
    }

    #[test]
    fn test_level2() {
        let a = AcCharge::new();
        assert!(a.level2());
    }

    #[test]
    fn test_no_ground() {
        let mut a = AcCharge::new();
        a.grounding_ok = false;
        assert!(!a.safe_to_charge());
    }

    #[test]
    fn test_health() {
        let a = AcCharge::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
