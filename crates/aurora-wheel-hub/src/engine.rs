/// Wheel hub: bearing, ABS ring, stud, seal
/// Phase 649

#[derive(Debug, Clone)]
pub struct WheelHub {
    pub bearing_ok: bool,
    pub abs_ring_ok: bool,
    pub stud_ok: bool,
    pub seal_ok: bool,
    pub play_ok: bool,
}

impl Default for WheelHub {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelHub {
    pub fn new() -> Self {
        Self {
            bearing_ok: true,
            abs_ring_ok: true,
            stud_ok: true,
            seal_ok: true,
            play_ok: true,
        }
    }

    pub fn rotation_ok(&self) -> bool {
        self.bearing_ok && self.play_ok
    }

    pub fn sensors_ok(&self) -> bool {
        self.abs_ring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.rotation_ok() && self.sensors_ok() && self.stud_ok && self.seal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.abs_ring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        let c = WheelHub::new();
        assert!(c.rotation_ok());
    }

    #[test]
    fn test_sensors() {
        let c = WheelHub::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelHub::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WheelHub::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing() {
        let mut c = WheelHub::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WheelHub::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
