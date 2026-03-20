/// steering ctrl: angle, torque, assist, vibrate, heat
/// Phase 1309

#[derive(Debug, Clone)]
pub struct SteeringCtrl {
    pub angle_ok: bool,
    pub torque_ok: bool,
    pub assist_ok: bool,
    pub vibrate_ok: bool,
    pub heat_ok: bool,
}

impl Default for SteeringCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringCtrl {
    pub fn new() -> Self {
        Self {
            angle_ok: true,
            torque_ok: true,
            assist_ok: true,
            vibrate_ok: true,
            heat_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.angle_ok && self.torque_ok && self.assist_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.vibrate_ok && self.heat_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.angle_ok || !self.torque_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.angle_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SteeringCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SteeringCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SteeringCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SteeringCtrl::new();
        c.angle_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SteeringCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
