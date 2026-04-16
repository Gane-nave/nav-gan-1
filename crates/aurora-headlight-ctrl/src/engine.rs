/// Headlight control: low/high beam, auto leveling, bulb monitoring
/// Phase 243

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeamMode {
    Off,
    DaytimeRunning,
    LowBeam,
    HighBeam,
    Auto,
}

#[derive(Debug, Clone)]
pub struct HeadlightController {
    pub mode: BeamMode,
    pub left_ok: bool,
    pub right_ok: bool,
    pub level_pct: f64,
    pub auto_leveling: bool,
}

impl Default for HeadlightController {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlightController {
    pub fn new() -> Self {
        Self {
            mode: BeamMode::Auto,
            left_ok: true,
            right_ok: true,
            level_pct: 50.0,
            auto_leveling: true,
        }
    }

    pub fn both_ok(&self) -> bool {
        self.left_ok && self.right_ok
    }

    pub fn is_on(&self) -> bool {
        self.mode != BeamMode::Off
    }

    pub fn bulb_out(&self) -> bool {
        !self.left_ok || !self.right_ok
    }

    pub fn needs_service(&self) -> bool {
        self.bulb_out()
    }

    pub fn health_score(&self) -> f64 {
        if !self.left_ok && !self.right_ok {
            return 0.0;
        }
        if self.bulb_out() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_ok() {
        let h = HeadlightController::new();
        assert!(h.both_ok());
    }

    #[test]
    fn test_is_on() {
        let h = HeadlightController::new();
        assert!(h.is_on());
    }

    #[test]
    fn test_no_bulb_out() {
        let h = HeadlightController::new();
        assert!(!h.bulb_out());
    }

    #[test]
    fn test_no_service() {
        let h = HeadlightController::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_bulb_out() {
        let mut h = HeadlightController::new();
        h.left_ok = false;
        assert!(h.bulb_out());
    }

    #[test]
    fn test_health() {
        let h = HeadlightController::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
