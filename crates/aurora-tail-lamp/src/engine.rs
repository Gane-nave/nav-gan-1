/// Tail lamp: brake light, reverse light, turn indicator, LED array
/// Phase 421

#[derive(Debug, Clone)]
pub struct TailLamp {
    pub brake_ok: bool,
    pub reverse_ok: bool,
    pub turn_ok: bool,
    pub running_ok: bool,
    pub led_failures: u32,
}

impl Default for TailLamp {
    fn default() -> Self {
        Self::new()
    }
}

impl TailLamp {
    pub fn new() -> Self {
        Self {
            brake_ok: true,
            reverse_ok: true,
            turn_ok: true,
            running_ok: true,
            led_failures: 0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.brake_ok
            && self.reverse_ok
            && self.turn_ok
            && self.running_ok
            && self.led_failures == 0
    }

    pub fn safety_ok(&self) -> bool {
        self.brake_ok && self.turn_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.brake_ok || !self.turn_ok || self.led_failures > 3
    }

    pub fn functional_pct(&self) -> f64 {
        let total = 4u32;
        let working = [
            self.brake_ok,
            self.reverse_ok,
            self.turn_ok,
            self.running_ok,
        ]
        .iter()
        .filter(|&&x| x)
        .count() as f64;
        working / total as f64 * 100.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.brake_ok {
            return 0.0;
        }
        if !self.turn_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let t = TailLamp::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_safety() {
        let t = TailLamp::new();
        assert!(t.safety_ok());
    }

    #[test]
    fn test_no_service() {
        let t = TailLamp::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_functional() {
        let t = TailLamp::new();
        assert!((t.functional_pct() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_brake_out() {
        let mut t = TailLamp::new();
        t.brake_ok = false;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TailLamp::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
