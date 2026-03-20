/// Charge controller: charging schedule, rate control, battery protection
/// Phase 289

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChargeState {
    Idle,
    Charging,
    Complete,
    Error,
    Scheduled,
}

#[derive(Debug, Clone)]
pub struct ChargeController {
    pub state: ChargeState,
    pub charge_rate_kw: f64,
    pub max_rate_kw: f64,
    pub target_soc_pct: f64,
    pub current_soc_pct: f64,
    pub battery_temp_c: f64,
}

impl Default for ChargeController {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeController {
    pub fn new() -> Self {
        Self {
            state: ChargeState::Idle,
            charge_rate_kw: 0.0,
            max_rate_kw: 11.0,
            target_soc_pct: 80.0,
            current_soc_pct: 50.0,
            battery_temp_c: 25.0,
        }
    }

    pub fn is_charging(&self) -> bool {
        self.state == ChargeState::Charging
    }

    pub fn at_target(&self) -> bool {
        self.current_soc_pct >= self.target_soc_pct
    }

    pub fn temp_ok(&self) -> bool {
        self.battery_temp_c > 0.0 && self.battery_temp_c < 45.0
    }

    pub fn time_remaining_h(&self) -> f64 {
        if self.charge_rate_kw <= 0.0 {
            return 0.0;
        }
        let remaining_pct = (self.target_soc_pct - self.current_soc_pct).max(0.0);
        remaining_pct / 100.0 * 75.0 / self.charge_rate_kw
    }

    pub fn health_score(&self) -> f64 {
        if self.state == ChargeState::Error {
            return 0.0;
        }
        if !self.temp_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_charging() {
        let c = ChargeController::new();
        assert!(!c.is_charging());
    }

    #[test]
    fn test_not_at_target() {
        let c = ChargeController::new();
        assert!(!c.at_target());
    }

    #[test]
    fn test_temp_ok() {
        let c = ChargeController::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_zero_time() {
        let c = ChargeController::new();
        assert!(c.time_remaining_h() < 0.1);
    }

    #[test]
    fn test_at_target() {
        let mut c = ChargeController::new();
        c.current_soc_pct = 85.0;
        assert!(c.at_target());
    }

    #[test]
    fn test_health() {
        let c = ChargeController::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
