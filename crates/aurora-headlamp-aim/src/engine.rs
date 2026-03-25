/// Headlamp aim: beam pattern, leveling, auto-aim, adaptive lighting
/// Phase 420

#[derive(Debug, Clone)]
pub struct HeadlampAim {
    pub vertical_deg: f64,
    pub horizontal_deg: f64,
    pub auto_level: bool,
    pub adaptive: bool,
    pub bulb_ok: bool,
}

impl Default for HeadlampAim {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlampAim {
    pub fn new() -> Self {
        Self {
            vertical_deg: -1.0,
            horizontal_deg: 0.0,
            auto_level: true,
            adaptive: true,
            bulb_ok: true,
        }
    }

    pub fn aim_ok(&self) -> bool {
        self.vertical_deg.abs() < 3.0 && self.horizontal_deg.abs() < 2.0
    }

    pub fn all_ok(&self) -> bool {
        self.aim_ok() && self.bulb_ok
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.aim_ok()
    }

    pub fn glare_risk(&self) -> bool {
        self.vertical_deg > 1.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.bulb_ok {
            return 0.0;
        }
        if !self.aim_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aim() {
        let h = HeadlampAim::new();
        assert!(h.aim_ok());
    }

    #[test]
    fn test_all_ok() {
        let h = HeadlampAim::new();
        assert!(h.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let h = HeadlampAim::new();
        assert!(!h.needs_adjustment());
    }

    #[test]
    fn test_no_glare() {
        let h = HeadlampAim::new();
        assert!(!h.glare_risk());
    }

    #[test]
    fn test_bad_aim() {
        let mut h = HeadlampAim::new();
        h.vertical_deg = 5.0;
        assert!(h.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let h = HeadlampAim::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
