/// Hinge mechanism: door hinge, hood hinge, trunk hinge, pivot wear
/// Phase 409

#[derive(Debug, Clone)]
pub struct HingeMech {
    pub friction_nm: f64,
    pub max_friction_nm: f64,
    pub play_mm: f64,
    pub lubricated: bool,
    pub pin_ok: bool,
}

impl Default for HingeMech {
    fn default() -> Self {
        Self::new()
    }
}

impl HingeMech {
    pub fn new() -> Self {
        Self {
            friction_nm: 2.0,
            max_friction_nm: 8.0,
            play_mm: 0.1,
            lubricated: true,
            pin_ok: true,
        }
    }

    pub fn smooth(&self) -> bool {
        self.friction_nm < self.max_friction_nm && self.lubricated
    }

    pub fn play_ok(&self) -> bool {
        self.play_mm < 1.0
    }

    pub fn all_ok(&self) -> bool {
        self.smooth() && self.play_ok() && self.pin_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.lubricated || !self.pin_ok || self.play_mm > 2.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.pin_ok {
            return 0.0;
        }
        if !self.lubricated {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth() {
        let h = HingeMech::new();
        assert!(h.smooth());
    }

    #[test]
    fn test_play() {
        let h = HingeMech::new();
        assert!(h.play_ok());
    }

    #[test]
    fn test_all_ok() {
        let h = HingeMech::new();
        assert!(h.all_ok());
    }

    #[test]
    fn test_no_service() {
        let h = HingeMech::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_dry() {
        let mut h = HingeMech::new();
        h.lubricated = false;
        assert!(h.needs_service());
    }

    #[test]
    fn test_health() {
        let h = HingeMech::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
