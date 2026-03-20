/// Parking brake: electronic park brake, hill hold, auto-apply
/// Phase 155

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParkBrakeState {
    Released,
    Engaged,
    Engaging,
    Releasing,
    Fault,
}

#[derive(Debug, Clone)]
pub struct ParkBrakeSystem {
    pub state: ParkBrakeState,
    pub auto_hold: bool,
    pub hill_hold: bool,
    pub grade_pct: f64,
    pub vehicle_speed_kmh: f64,
}

impl Default for ParkBrakeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkBrakeSystem {
    pub fn new() -> Self {
        Self {
            state: ParkBrakeState::Released,
            auto_hold: true,
            hill_hold: true,
            grade_pct: 0.0,
            vehicle_speed_kmh: 0.0,
        }
    }

    pub fn is_engaged(&self) -> bool {
        matches!(self.state, ParkBrakeState::Engaged)
    }

    pub fn is_safe_to_release(&self) -> bool {
        self.grade_pct.abs() < 15.0 || self.vehicle_speed_kmh > 0.0
    }

    pub fn should_auto_engage(&self) -> bool {
        self.auto_hold && self.vehicle_speed_kmh < 0.1 && !self.is_engaged()
    }

    pub fn needs_hill_hold(&self) -> bool {
        self.hill_hold && self.grade_pct.abs() > 3.0 && self.vehicle_speed_kmh < 1.0
    }

    pub fn holding_force_n(&self) -> f64 {
        if self.is_engaged() {
            let base = 5000.0;
            let grade_factor = 1.0 + self.grade_pct.abs() / 100.0;
            base * grade_factor
        } else {
            0.0
        }
    }

    pub fn has_fault(&self) -> bool {
        matches!(self.state, ParkBrakeState::Fault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engaged() {
        let mut s = ParkBrakeSystem::new();
        s.state = ParkBrakeState::Engaged;
        assert!(s.is_engaged());
    }

    #[test]
    fn test_released() {
        let s = ParkBrakeSystem::new();
        assert!(!s.is_engaged());
    }

    #[test]
    fn test_safe_release_flat() {
        let s = ParkBrakeSystem::new();
        assert!(s.is_safe_to_release());
    }

    #[test]
    fn test_auto_engage() {
        let s = ParkBrakeSystem::new();
        assert!(s.should_auto_engage());
    }

    #[test]
    fn test_hill_hold() {
        let mut s = ParkBrakeSystem::new();
        s.grade_pct = 8.0;
        assert!(s.needs_hill_hold());
    }

    #[test]
    fn test_no_hill_hold_flat() {
        let s = ParkBrakeSystem::new();
        assert!(!s.needs_hill_hold());
    }

    #[test]
    fn test_holding_force() {
        let mut s = ParkBrakeSystem::new();
        s.state = ParkBrakeState::Engaged;
        s.grade_pct = 10.0;
        assert!(s.holding_force_n() > 5000.0);
    }

    #[test]
    fn test_no_force_released() {
        let s = ParkBrakeSystem::new();
        assert!((s.holding_force_n() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_fault() {
        let mut s = ParkBrakeSystem::new();
        s.state = ParkBrakeState::Fault;
        assert!(s.has_fault());
    }
}
