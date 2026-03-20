/// Window regulator: motor, cable, track, switch
/// Phase 535

#[derive(Debug, Clone)]
pub struct WindowRegulator {
    pub position_pct: f64,
    pub motor_ok: bool,
    pub cable_ok: bool,
    pub track_ok: bool,
    pub switch_ok: bool,
}

impl Default for WindowRegulator {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowRegulator {
    pub fn new() -> Self {
        Self {
            position_pct: 100.0,
            motor_ok: true,
            cable_ok: true,
            track_ok: true,
            switch_ok: true,
        }
    }

    pub fn is_open(&self) -> bool {
        self.position_pct < 100.0
    }

    pub fn mechanical_ok(&self) -> bool {
        self.motor_ok && self.cable_ok && self.track_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.switch_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.cable_ok
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
        let c = WindowRegulator::new();
        assert!(!c.is_open());
    }

    #[test]
    fn test_mechanical() {
        let c = WindowRegulator::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WindowRegulator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WindowRegulator::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = WindowRegulator::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WindowRegulator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
