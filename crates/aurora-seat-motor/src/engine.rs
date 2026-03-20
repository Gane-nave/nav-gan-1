/// Seat motor: track, recline, height, lumbar
/// Phase 682

#[derive(Debug, Clone)]
pub struct SeatMotor {
    pub track_ok: bool,
    pub recline_ok: bool,
    pub height_ok: bool,
    pub lumbar_ok: bool,
    pub memory_ok: bool,
}

impl Default for SeatMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatMotor {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            recline_ok: true,
            height_ok: true,
            lumbar_ok: true,
            memory_ok: true,
        }
    }

    pub fn position_ok(&self) -> bool {
        self.track_ok && self.recline_ok && self.height_ok
    }

    pub fn comfort_ok(&self) -> bool {
        self.lumbar_ok && self.memory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.position_ok() && self.comfort_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.track_ok || !self.recline_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let c = SeatMotor::new();
        assert!(c.position_ok());
    }

    #[test]
    fn test_comfort() {
        let c = SeatMotor::new();
        assert!(c.comfort_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SeatMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_track() {
        let mut c = SeatMotor::new();
        c.track_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SeatMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
