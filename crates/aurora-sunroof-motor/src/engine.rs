/// Sunroof motor: track, tilt, slide, drain
/// Phase 683

#[derive(Debug, Clone)]
pub struct SunroofMotor {
    pub track_ok: bool,
    pub tilt_ok: bool,
    pub slide_ok: bool,
    pub drain_ok: bool,
    pub seal_ok: bool,
}

impl Default for SunroofMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl SunroofMotor {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            tilt_ok: true,
            slide_ok: true,
            drain_ok: true,
            seal_ok: true,
        }
    }

    pub fn movement_ok(&self) -> bool {
        self.track_ok && self.tilt_ok && self.slide_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.drain_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.movement_ok() && self.sealing_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.track_ok || !self.drain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement() {
        let c = SunroofMotor::new();
        assert!(c.movement_ok());
    }

    #[test]
    fn test_sealing() {
        let c = SunroofMotor::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SunroofMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SunroofMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_track() {
        let mut c = SunroofMotor::new();
        c.track_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SunroofMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
