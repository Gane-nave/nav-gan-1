/// Seat control: position memory, adjustment motors, occupancy sensing
/// Phase 256

#[derive(Debug, Clone)]
pub struct SeatController {
    pub position_forward_mm: f64,
    pub recline_deg: f64,
    pub height_mm: f64,
    pub memory_slots: u8,
    pub occupied: bool,
    pub motors_ok: bool,
}

impl Default for SeatController {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatController {
    pub fn new() -> Self {
        Self {
            position_forward_mm: 200.0,
            recline_deg: 15.0,
            height_mm: 50.0,
            memory_slots: 3,
            occupied: true,
            motors_ok: true,
        }
    }

    pub fn in_range(&self) -> bool {
        self.position_forward_mm >= 0.0
            && self.position_forward_mm <= 300.0
            && self.recline_deg >= 0.0
            && self.recline_deg <= 60.0
    }

    pub fn has_memory(&self) -> bool {
        self.memory_slots > 0
    }

    pub fn can_adjust(&self) -> bool {
        self.motors_ok
    }

    pub fn fully_upright(&self) -> bool {
        self.recline_deg < 5.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.motors_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        let s = SeatController::new();
        assert!(s.in_range());
    }

    #[test]
    fn test_has_memory() {
        let s = SeatController::new();
        assert!(s.has_memory());
    }

    #[test]
    fn test_can_adjust() {
        let s = SeatController::new();
        assert!(s.can_adjust());
    }

    #[test]
    fn test_not_upright() {
        let s = SeatController::new();
        assert!(!s.fully_upright());
    }

    #[test]
    fn test_occupied() {
        let s = SeatController::new();
        assert!(s.occupied);
    }

    #[test]
    fn test_health() {
        let s = SeatController::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
