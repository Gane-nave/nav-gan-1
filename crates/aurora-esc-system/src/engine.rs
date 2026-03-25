/// Electronic Stability Control: yaw rate, lateral acceleration, torque vectoring
/// Phase 206

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EscMode {
    Normal,
    Sport,
    Off,
    Fault,
}

#[derive(Debug, Clone)]
pub struct EscSystem {
    pub mode: EscMode,
    pub yaw_rate_dps: f64,
    pub target_yaw_dps: f64,
    pub lateral_accel_g: f64,
    pub intervention_active: bool,
    pub interventions_count: u32,
}

impl Default for EscSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EscSystem {
    pub fn new() -> Self {
        Self {
            mode: EscMode::Normal,
            yaw_rate_dps: 0.0,
            target_yaw_dps: 0.0,
            lateral_accel_g: 0.0,
            intervention_active: false,
            interventions_count: 0,
        }
    }

    pub fn yaw_error_dps(&self) -> f64 {
        (self.yaw_rate_dps - self.target_yaw_dps).abs()
    }

    pub fn oversteer(&self) -> bool {
        self.yaw_rate_dps.abs() > self.target_yaw_dps.abs() * 1.2 && self.target_yaw_dps.abs() > 1.0
    }

    pub fn understeer(&self) -> bool {
        self.yaw_rate_dps.abs() < self.target_yaw_dps.abs() * 0.8 && self.target_yaw_dps.abs() > 1.0
    }

    pub fn needs_correction(&self) -> bool {
        self.mode != EscMode::Off && self.yaw_error_dps() > 5.0
    }

    pub fn has_fault(&self) -> bool {
        self.mode == EscMode::Fault
    }

    pub fn health_score(&self) -> f64 {
        if self.has_fault() {
            return 0.0;
        }
        if self.mode == EscMode::Off {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let e = EscSystem::new();
        assert_eq!(e.mode, EscMode::Normal);
    }

    #[test]
    fn test_no_yaw_error() {
        let e = EscSystem::new();
        assert!(e.yaw_error_dps() < 0.1);
    }

    #[test]
    fn test_oversteer() {
        let mut e = EscSystem::new();
        e.yaw_rate_dps = 20.0;
        e.target_yaw_dps = 10.0;
        assert!(e.oversteer());
    }

    #[test]
    fn test_understeer() {
        let mut e = EscSystem::new();
        e.yaw_rate_dps = 3.0;
        e.target_yaw_dps = 10.0;
        assert!(e.understeer());
    }

    #[test]
    fn test_no_fault() {
        let e = EscSystem::new();
        assert!(!e.has_fault());
    }

    #[test]
    fn test_health() {
        let e = EscSystem::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
