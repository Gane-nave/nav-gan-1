/// fan ctrl: start, speed, direction, pwm, stop
/// Phase 1153

#[derive(Debug, Clone)]
pub struct FanCtrl {
    pub start_ok: bool,
    pub speed_ok: bool,
    pub direction_ok: bool,
    pub pwm_ok: bool,
    pub stop_ok: bool,
}

impl Default for FanCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl FanCtrl {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            speed_ok: true,
            direction_ok: true,
            pwm_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.speed_ok && self.direction_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pwm_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FanCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FanCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FanCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FanCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FanCtrl::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FanCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
