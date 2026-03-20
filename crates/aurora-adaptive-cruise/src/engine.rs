/// Adaptive cruise control: radar following, gap control, stop & go
/// Phase 262

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccState {
    Off,
    Standby,
    Following,
    Coasting,
    Braking,
}

#[derive(Debug, Clone)]
pub struct AdaptiveCruise {
    pub state: AccState,
    pub set_speed_kmh: f64,
    pub current_speed_kmh: f64,
    pub gap_seconds: f64,
    pub target_gap_s: f64,
    pub radar_ok: bool,
}

impl Default for AdaptiveCruise {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveCruise {
    pub fn new() -> Self {
        Self {
            state: AccState::Off,
            set_speed_kmh: 0.0,
            current_speed_kmh: 0.0,
            gap_seconds: 3.0,
            target_gap_s: 2.0,
            radar_ok: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state != AccState::Off && self.state != AccState::Standby
    }

    pub fn gap_ok(&self) -> bool {
        self.gap_seconds >= self.target_gap_s
    }

    pub fn too_close(&self) -> bool {
        self.gap_seconds < self.target_gap_s * 0.5
    }

    pub fn speed_error_kmh(&self) -> f64 {
        (self.current_speed_kmh - self.set_speed_kmh).abs()
    }

    pub fn health_score(&self) -> f64 {
        if !self.radar_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_active() {
        let a = AdaptiveCruise::new();
        assert!(!a.is_active());
    }

    #[test]
    fn test_gap_ok() {
        let a = AdaptiveCruise::new();
        assert!(a.gap_ok());
    }

    #[test]
    fn test_not_close() {
        let a = AdaptiveCruise::new();
        assert!(!a.too_close());
    }

    #[test]
    fn test_no_speed_error() {
        let a = AdaptiveCruise::new();
        assert!(a.speed_error_kmh() < 0.1);
    }

    #[test]
    fn test_following() {
        let mut a = AdaptiveCruise::new();
        a.state = AccState::Following;
        assert!(a.is_active());
    }

    #[test]
    fn test_health() {
        let a = AdaptiveCruise::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
