/// Seat adjustment: power tracks, lumbar, memory
/// Phase 539

#[derive(Debug, Clone)]
pub struct SeatAdjust {
    pub track_ok: bool,
    pub lumbar_ok: bool,
    pub memory_ok: bool,
    pub motor_count: u32,
    pub failed_motors: u32,
}

impl Default for SeatAdjust {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatAdjust {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            lumbar_ok: true,
            memory_ok: true,
            motor_count: 8,
            failed_motors: 0,
        }
    }

    pub fn motors_ok(&self) -> bool {
        self.failed_motors == 0
    }

    pub fn features_ok(&self) -> bool {
        self.lumbar_ok && self.memory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.motors_ok() && self.features_ok() && self.track_ok
    }

    pub fn needs_service(&self) -> bool {
        self.failed_motors > 0 || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.failed_motors > 0 { return 25.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motors() {
        let c = SeatAdjust::new();
        assert!(c.motors_ok());
    }

    #[test]
    fn test_features() {
        let c = SeatAdjust::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatAdjust::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SeatAdjust::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor_fail() {
        let mut c = SeatAdjust::new();
        c.failed_motors = 1;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SeatAdjust::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
