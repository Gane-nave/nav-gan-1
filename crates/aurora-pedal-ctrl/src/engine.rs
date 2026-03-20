/// pedal ctrl: throttle, brake, clutch, force, travel
/// Phase 1310

#[derive(Debug, Clone)]
pub struct PedalCtrl {
    pub throttle_ok: bool,
    pub brake_ok: bool,
    pub clutch_ok: bool,
    pub force_ok: bool,
    pub travel_ok: bool,
}

impl Default for PedalCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl PedalCtrl {
    pub fn new() -> Self {
        Self {
            throttle_ok: true,
            brake_ok: true,
            clutch_ok: true,
            force_ok: true,
            travel_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.throttle_ok && self.brake_ok && self.clutch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.force_ok && self.travel_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.throttle_ok || !self.brake_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.throttle_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = PedalCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PedalCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedalCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PedalCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PedalCtrl::new();
        c.throttle_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PedalCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
