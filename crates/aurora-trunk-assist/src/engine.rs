/// Trunk assistance: power liftgate, cargo management, load sensing
/// Phase 143

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrunkState {
    Closed,
    Opening,
    Open,
    Closing,
    Stuck,
}

impl TrunkState {
    pub fn is_secure(&self) -> bool {
        matches!(self, TrunkState::Closed)
    }
    pub fn is_moving(&self) -> bool {
        matches!(self, TrunkState::Opening | TrunkState::Closing)
    }
}

#[derive(Debug, Clone)]
pub struct TrunkSystem {
    pub state: TrunkState,
    pub max_load_kg: f64,
    pub current_load_kg: f64,
    pub height_limit_mm: f64,
    pub power_liftgate: bool,
    pub hands_free: bool,
}

impl TrunkSystem {
    pub fn new(max_load: f64) -> Self {
        Self {
            state: TrunkState::Closed,
            max_load_kg: max_load,
            current_load_kg: 0.0,
            height_limit_mm: 2000.0,
            power_liftgate: true,
            hands_free: true,
        }
    }

    pub fn is_overloaded(&self) -> bool {
        self.current_load_kg > self.max_load_kg
    }

    pub fn load_pct(&self) -> f64 {
        if self.max_load_kg <= 0.0 {
            return 0.0;
        }
        (self.current_load_kg / self.max_load_kg * 100.0).min(100.0)
    }

    pub fn remaining_capacity_kg(&self) -> f64 {
        (self.max_load_kg - self.current_load_kg).max(0.0)
    }

    pub fn can_open(&self, clearance_mm: f64) -> bool {
        clearance_mm >= self.height_limit_mm && self.state == TrunkState::Closed
    }

    pub fn obstruction_detected(&self) -> bool {
        self.state == TrunkState::Stuck
    }

    pub fn safe_to_drive(&self) -> bool {
        self.state.is_secure() && !self.is_overloaded()
    }

    pub fn gesture_available(&self) -> bool {
        self.hands_free && self.power_liftgate
    }

    pub fn estimated_open_time_sec(&self) -> f64 {
        if self.power_liftgate {
            3.0
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_secure() {
        assert!(TrunkState::Closed.is_secure());
        assert!(!TrunkState::Open.is_secure());
    }

    #[test]
    fn test_state_moving() {
        assert!(TrunkState::Opening.is_moving());
    }

    #[test]
    fn test_overloaded() {
        let mut t = TrunkSystem::new(100.0);
        t.current_load_kg = 150.0;
        assert!(t.is_overloaded());
    }

    #[test]
    fn test_not_overloaded() {
        let t = TrunkSystem::new(100.0);
        assert!(!t.is_overloaded());
    }

    #[test]
    fn test_load_pct() {
        let mut t = TrunkSystem::new(200.0);
        t.current_load_kg = 100.0;
        assert!((t.load_pct() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_remaining() {
        let mut t = TrunkSystem::new(200.0);
        t.current_load_kg = 80.0;
        assert!((t.remaining_capacity_kg() - 120.0).abs() < 0.1);
    }

    #[test]
    fn test_can_open() {
        let t = TrunkSystem::new(100.0);
        assert!(t.can_open(2500.0));
        assert!(!t.can_open(1500.0));
    }

    #[test]
    fn test_safe_to_drive() {
        let t = TrunkSystem::new(100.0);
        assert!(t.safe_to_drive());
    }

    #[test]
    fn test_gesture() {
        let t = TrunkSystem::new(100.0);
        assert!(t.gesture_available());
    }

    #[test]
    fn test_open_time() {
        let t = TrunkSystem::new(100.0);
        assert!((t.estimated_open_time_sec() - 3.0).abs() < 0.1);
    }
}
