/// Wiper control: intermittent timing, speed stages, park position
/// Phase 240

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WiperSpeed {
    Off,
    Intermittent,
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct WiperController {
    pub speed: WiperSpeed,
    pub intermittent_delay_s: f64,
    pub parked: bool,
    pub motor_current_a: f64,
    pub motor_ok: bool,
}

impl Default for WiperController {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperController {
    pub fn new() -> Self {
        Self {
            speed: WiperSpeed::Off,
            intermittent_delay_s: 5.0,
            parked: true,
            motor_current_a: 0.0,
            motor_ok: true,
        }
    }

    pub fn is_running(&self) -> bool {
        self.speed != WiperSpeed::Off
    }

    pub fn current_ok(&self) -> bool {
        self.motor_current_a < 15.0
    }

    pub fn motor_stalled(&self) -> bool {
        self.is_running() && self.motor_current_a > 20.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if self.motor_stalled() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_off() {
        let w = WiperController::new();
        assert!(!w.is_running());
    }

    #[test]
    fn test_parked() {
        let w = WiperController::new();
        assert!(w.parked);
    }

    #[test]
    fn test_current_ok() {
        let w = WiperController::new();
        assert!(w.current_ok());
    }

    #[test]
    fn test_not_stalled() {
        let w = WiperController::new();
        assert!(!w.motor_stalled());
    }

    #[test]
    fn test_running() {
        let mut w = WiperController::new();
        w.speed = WiperSpeed::Low;
        assert!(w.is_running());
    }

    #[test]
    fn test_health() {
        let w = WiperController::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
