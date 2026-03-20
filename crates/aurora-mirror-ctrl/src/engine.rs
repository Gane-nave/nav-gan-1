/// Mirror control: power adjustment, auto-dimming, folding, blind spot
/// Phase 250

#[derive(Debug, Clone)]
pub struct MirrorController {
    pub left_position: (f64, f64),
    pub right_position: (f64, f64),
    pub auto_dim_active: bool,
    pub folded: bool,
    pub heated: bool,
    pub blind_spot_left: bool,
    pub blind_spot_right: bool,
}

impl Default for MirrorController {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorController {
    pub fn new() -> Self {
        Self {
            left_position: (0.0, 0.0),
            right_position: (0.0, 0.0),
            auto_dim_active: true,
            folded: false,
            heated: false,
            blind_spot_left: false,
            blind_spot_right: false,
        }
    }

    pub fn any_blind_spot(&self) -> bool {
        self.blind_spot_left || self.blind_spot_right
    }

    pub fn is_folded(&self) -> bool {
        self.folded
    }

    pub fn should_heat(&self, ambient_temp_c: f64) -> bool {
        ambient_temp_c < 5.0
    }

    pub fn driving_ready(&self) -> bool {
        !self.folded
    }

    pub fn health_score(&self) -> f64 {
        if self.folded {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_blind_spot() {
        let m = MirrorController::new();
        assert!(!m.any_blind_spot());
    }

    #[test]
    fn test_not_folded() {
        let m = MirrorController::new();
        assert!(!m.is_folded());
    }

    #[test]
    fn test_no_heat() {
        let m = MirrorController::new();
        assert!(!m.should_heat(20.0));
    }

    #[test]
    fn test_driving_ready() {
        let m = MirrorController::new();
        assert!(m.driving_ready());
    }

    #[test]
    fn test_blind_spot() {
        let mut m = MirrorController::new();
        m.blind_spot_left = true;
        assert!(m.any_blind_spot());
    }

    #[test]
    fn test_health() {
        let m = MirrorController::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
