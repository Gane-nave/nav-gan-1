/// Sunroof: motor, track, seal, drain, tilt
/// Phase 537

#[derive(Debug, Clone)]
pub struct Sunroof {
    pub position_pct: f64,
    pub motor_ok: bool,
    pub track_ok: bool,
    pub seal_ok: bool,
    pub drain_ok: bool,
}

impl Default for Sunroof {
    fn default() -> Self {
        Self::new()
    }
}

impl Sunroof {
    pub fn new() -> Self {
        Self {
            position_pct: 0.0,
            motor_ok: true,
            track_ok: true,
            seal_ok: true,
            drain_ok: true,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.position_pct < 1.0
    }

    pub fn mechanical_ok(&self) -> bool {
        self.motor_ok && self.track_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.seal_ok && self.drain_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed() {
        let c = Sunroof::new();
        assert!(c.is_closed());
    }

    #[test]
    fn test_mechanical() {
        let c = Sunroof::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Sunroof::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Sunroof::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = Sunroof::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Sunroof::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
