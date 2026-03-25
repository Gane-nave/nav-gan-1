/// Sunroof mechanism: tilt, slide, panoramic, seal, drain
/// Phase 418

#[derive(Debug, Clone)]
pub struct SunroofMech {
    pub motor_ok: bool,
    pub track_ok: bool,
    pub seal_ok: bool,
    pub drain_clear: bool,
    pub position_pct: f64,
}

impl Default for SunroofMech {
    fn default() -> Self {
        Self::new()
    }
}

impl SunroofMech {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            track_ok: true,
            seal_ok: true,
            drain_clear: true,
            position_pct: 0.0,
        }
    }

    pub fn functional(&self) -> bool {
        self.motor_ok && self.track_ok
    }

    pub fn all_ok(&self) -> bool {
        self.functional() && self.seal_ok && self.drain_clear
    }

    pub fn leak_risk(&self) -> bool {
        !self.seal_ok || !self.drain_clear
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.track_ok || !self.drain_clear
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 20.0;
        }
        if !self.seal_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional() {
        let s = SunroofMech::new();
        assert!(s.functional());
    }

    #[test]
    fn test_all_ok() {
        let s = SunroofMech::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_leak() {
        let s = SunroofMech::new();
        assert!(!s.leak_risk());
    }

    #[test]
    fn test_no_service() {
        let s = SunroofMech::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_bad_seal() {
        let mut s = SunroofMech::new();
        s.seal_ok = false;
        assert!(s.leak_risk());
    }

    #[test]
    fn test_health() {
        let s = SunroofMech::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
