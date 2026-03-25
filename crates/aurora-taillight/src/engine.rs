/// Taillight: LED array, brake light, turn signal
/// Phase 532

#[derive(Debug, Clone)]
pub struct Taillight {
    pub led_count: u32,
    pub failed_leds: u32,
    pub brake_ok: bool,
    pub turn_ok: bool,
    pub reverse_ok: bool,
}

impl Default for Taillight {
    fn default() -> Self {
        Self::new()
    }
}

impl Taillight {
    pub fn new() -> Self {
        Self {
            led_count: 24,
            failed_leds: 0,
            brake_ok: true,
            turn_ok: true,
            reverse_ok: true,
        }
    }

    pub fn leds_ok(&self) -> bool {
        self.failed_leds == 0
    }

    pub fn signals_ok(&self) -> bool {
        self.brake_ok && self.turn_ok && self.reverse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.leds_ok() && self.signals_ok()
    }

    pub fn needs_service(&self) -> bool {
        self.failed_leds > 3 || !self.brake_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.brake_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leds() {
        let c = Taillight::new();
        assert!(c.leds_ok());
    }

    #[test]
    fn test_signals() {
        let c = Taillight::new();
        assert!(c.signals_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Taillight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Taillight::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_brake_fail() {
        let mut c = Taillight::new();
        c.brake_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Taillight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
