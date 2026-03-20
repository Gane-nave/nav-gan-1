/// esc ctrl: yaw, brake, throttle, steer, stabilize
/// Phase 1159

#[derive(Debug, Clone)]
pub struct EscCtrl {
    pub yaw_ok: bool,
    pub brake_ok: bool,
    pub throttle_ok: bool,
    pub steer_ok: bool,
    pub stabilize_ok: bool,
}

impl Default for EscCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl EscCtrl {
    pub fn new() -> Self {
        Self {
            yaw_ok: true,
            brake_ok: true,
            throttle_ok: true,
            steer_ok: true,
            stabilize_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.yaw_ok && self.brake_ok && self.throttle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.steer_ok && self.stabilize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.yaw_ok || !self.brake_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.yaw_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = EscCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EscCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EscCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EscCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EscCtrl::new();
        c.yaw_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EscCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
