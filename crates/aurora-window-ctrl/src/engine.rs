/// Window control: power windows, auto up/down, pinch protection
/// Phase 251

#[derive(Debug, Clone)]
pub struct WindowController {
    pub position_pct: [f64; 4],
    pub pinch_protection: bool,
    pub auto_up_down: bool,
    pub child_lock: bool,
    pub motor_ok: [bool; 4],
}

impl Default for WindowController {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowController {
    pub fn new() -> Self {
        Self {
            position_pct: [100.0; 4],
            pinch_protection: true,
            auto_up_down: true,
            child_lock: false,
            motor_ok: [true; 4],
        }
    }

    pub fn all_closed(&self) -> bool {
        self.position_pct.iter().all(|&p| p >= 99.0)
    }

    pub fn all_open(&self) -> bool {
        self.position_pct.iter().all(|&p| p <= 1.0)
    }

    pub fn any_open(&self) -> bool {
        self.position_pct.iter().any(|&p| p < 99.0)
    }

    pub fn all_motors_ok(&self) -> bool {
        self.motor_ok.iter().all(|&ok| ok)
    }

    pub fn health_score(&self) -> f64 {
        let working = self.motor_ok.iter().filter(|&&ok| ok).count();
        working as f64 / 4.0 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_closed() {
        let w = WindowController::new();
        assert!(w.all_closed());
    }

    #[test]
    fn test_not_all_open() {
        let w = WindowController::new();
        assert!(!w.all_open());
    }

    #[test]
    fn test_none_open() {
        let w = WindowController::new();
        assert!(!w.any_open());
    }

    #[test]
    fn test_all_motors() {
        let w = WindowController::new();
        assert!(w.all_motors_ok());
    }

    #[test]
    fn test_open_window() {
        let mut w = WindowController::new();
        w.position_pct[0] = 50.0;
        assert!(w.any_open());
    }

    #[test]
    fn test_health() {
        let w = WindowController::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
