/// Turn signal: blinker control, hazard lights, lane change assist
/// Phase 245

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignalState {
    Off,
    Left,
    Right,
    Hazard,
}

#[derive(Debug, Clone)]
pub struct TurnSignal {
    pub state: SignalState,
    pub flash_rate_hz: f64,
    pub left_front_ok: bool,
    pub left_rear_ok: bool,
    pub right_front_ok: bool,
    pub right_rear_ok: bool,
}

impl Default for TurnSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnSignal {
    pub fn new() -> Self {
        Self {
            state: SignalState::Off,
            flash_rate_hz: 1.5,
            left_front_ok: true,
            left_rear_ok: true,
            right_front_ok: true,
            right_rear_ok: true,
        }
    }

    pub fn is_signaling(&self) -> bool {
        self.state != SignalState::Off
    }

    pub fn all_bulbs_ok(&self) -> bool {
        self.left_front_ok && self.left_rear_ok && self.right_front_ok && self.right_rear_ok
    }

    pub fn hyper_flash(&self) -> bool {
        self.flash_rate_hz > 2.5
    }

    pub fn bulb_out_detected(&self) -> bool {
        !self.all_bulbs_ok() || self.hyper_flash()
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_bulbs_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_off() {
        let t = TurnSignal::new();
        assert!(!t.is_signaling());
    }

    #[test]
    fn test_all_bulbs() {
        let t = TurnSignal::new();
        assert!(t.all_bulbs_ok());
    }

    #[test]
    fn test_no_hyper() {
        let t = TurnSignal::new();
        assert!(!t.hyper_flash());
    }

    #[test]
    fn test_no_bulb_out() {
        let t = TurnSignal::new();
        assert!(!t.bulb_out_detected());
    }

    #[test]
    fn test_left() {
        let mut t = TurnSignal::new();
        t.state = SignalState::Left;
        assert!(t.is_signaling());
    }

    #[test]
    fn test_health() {
        let t = TurnSignal::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
