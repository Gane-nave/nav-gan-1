/// ABS control: anti-lock braking, wheel slip management, brake modulation
/// Phase 205

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbsState {
    Inactive,
    Monitoring,
    Intervening,
    Fault,
}

#[derive(Debug, Clone)]
pub struct AbsController {
    pub state: AbsState,
    pub wheel_slip_pct: [f64; 4],
    pub intervention_count: u32,
    pub brake_pressure_bar: f64,
    pub enabled: bool,
}

impl Default for AbsController {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsController {
    pub fn new() -> Self {
        Self {
            state: AbsState::Monitoring,
            wheel_slip_pct: [0.0; 4],
            intervention_count: 0,
            brake_pressure_bar: 0.0,
            enabled: true,
        }
    }

    pub fn max_slip(&self) -> f64 {
        self.wheel_slip_pct.iter().cloned().fold(0.0_f64, f64::max)
    }

    pub fn needs_intervention(&self) -> bool {
        self.enabled && self.max_slip() > 15.0
    }

    pub fn is_active(&self) -> bool {
        self.state == AbsState::Intervening
    }

    pub fn has_fault(&self) -> bool {
        self.state == AbsState::Fault
    }

    pub fn health_score(&self) -> f64 {
        if self.has_fault() {
            return 0.0;
        }
        if !self.enabled {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_monitoring() {
        let a = AbsController::new();
        assert_eq!(a.state, AbsState::Monitoring);
    }

    #[test]
    fn test_max_slip() {
        let mut a = AbsController::new();
        a.wheel_slip_pct = [5.0, 10.0, 3.0, 7.0];
        assert!((a.max_slip() - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_no_intervention() {
        let a = AbsController::new();
        assert!(!a.needs_intervention());
    }

    #[test]
    fn test_needs_intervention() {
        let mut a = AbsController::new();
        a.wheel_slip_pct[0] = 20.0;
        assert!(a.needs_intervention());
    }

    #[test]
    fn test_no_fault() {
        let a = AbsController::new();
        assert!(!a.has_fault());
    }

    #[test]
    fn test_health() {
        let a = AbsController::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
