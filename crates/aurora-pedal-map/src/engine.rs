/// Pedal map: throttle curve, regen curve, creep, sport
/// Phase 873

#[derive(Debug, Clone)]
pub struct PedalMap {
    pub throttle_ok: bool,
    pub regen_ok: bool,
    pub creep_ok: bool,
    pub sport_ok: bool,
    pub eco_ok: bool,
}

impl Default for PedalMap {
    fn default() -> Self {
        Self::new()
    }
}

impl PedalMap {
    pub fn new() -> Self {
        Self {
            throttle_ok: true,
            regen_ok: true,
            creep_ok: true,
            sport_ok: true,
            eco_ok: true,
        }
    }

    pub fn acceleration_ok(&self) -> bool {
        self.throttle_ok && self.sport_ok && self.eco_ok
    }

    pub fn deceleration_ok(&self) -> bool {
        self.regen_ok && self.creep_ok
    }

    pub fn all_ok(&self) -> bool {
        self.acceleration_ok() && self.deceleration_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.throttle_ok || !self.regen_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.throttle_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration() {
        let c = PedalMap::new();
        assert!(c.acceleration_ok());
    }

    #[test]
    fn test_deceleration() {
        let c = PedalMap::new();
        assert!(c.deceleration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedalMap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = PedalMap::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_throttle() {
        let mut c = PedalMap::new();
        c.throttle_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = PedalMap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
