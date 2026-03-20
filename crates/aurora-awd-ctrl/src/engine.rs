/// All-wheel drive control: torque distribution, drive mode, terrain response
/// Phase 163

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AwdMode {
    Auto,
    FrontBias,
    RearBias,
    Equal,
    Off,
}

impl AwdMode {
    pub fn front_torque_pct(&self) -> f64 {
        match self {
            AwdMode::Auto => 60.0,
            AwdMode::FrontBias => 80.0,
            AwdMode::RearBias => 30.0,
            AwdMode::Equal => 50.0,
            AwdMode::Off => 100.0,
        }
    }

    pub fn rear_torque_pct(&self) -> f64 {
        100.0 - self.front_torque_pct()
    }
}

#[derive(Debug, Clone)]
pub struct AwdSystem {
    pub mode: AwdMode,
    pub enabled: bool,
    pub front_slip: f64,
    pub rear_slip: f64,
    pub transfer_case_temp_c: f64,
}

impl Default for AwdSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AwdSystem {
    pub fn new() -> Self {
        Self {
            mode: AwdMode::Auto,
            enabled: true,
            front_slip: 0.0,
            rear_slip: 0.0,
            transfer_case_temp_c: 60.0,
        }
    }

    pub fn needs_rebalance(&self) -> bool {
        (self.front_slip - self.rear_slip).abs() > 0.1
    }

    pub fn effective_front_pct(&self) -> f64 {
        if !self.enabled {
            return 100.0;
        }
        self.mode.front_torque_pct()
    }

    pub fn transfer_case_ok(&self) -> bool {
        self.transfer_case_temp_c < 120.0
    }

    pub fn traction_score(&self) -> f64 {
        let slip_penalty = (self.front_slip + self.rear_slip) * 100.0;
        (100.0 - slip_penalty).clamp(0.0, 100.0)
    }

    pub fn is_awd_active(&self) -> bool {
        self.enabled && !matches!(self.mode, AwdMode::Off)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torque_split() {
        assert!((AwdMode::Equal.front_torque_pct() - 50.0).abs() < 0.1);
        assert!((AwdMode::Equal.rear_torque_pct() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_front_bias() {
        assert!(AwdMode::FrontBias.front_torque_pct() > 70.0);
    }

    #[test]
    fn test_rear_bias() {
        assert!(AwdMode::RearBias.rear_torque_pct() > 60.0);
    }

    #[test]
    fn test_rebalance() {
        let mut s = AwdSystem::new();
        s.front_slip = 0.3;
        assert!(s.needs_rebalance());
    }

    #[test]
    fn test_transfer_case_ok() {
        let s = AwdSystem::new();
        assert!(s.transfer_case_ok());
    }

    #[test]
    fn test_traction_score() {
        let s = AwdSystem::new();
        assert!((s.traction_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_awd_active() {
        let s = AwdSystem::new();
        assert!(s.is_awd_active());
    }

    #[test]
    fn test_awd_off() {
        let mut s = AwdSystem::new();
        s.mode = AwdMode::Off;
        assert!(!s.is_awd_active());
    }
}
