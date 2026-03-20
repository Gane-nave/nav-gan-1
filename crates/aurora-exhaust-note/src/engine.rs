/// Exhaust note: sound design, frequency tuning, active valve control
/// Phase 369

#[derive(Debug, Clone)]
pub struct ExhaustNote {
    pub frequency_hz: f64,
    pub volume_db: f64,
    pub valve_open: bool,
    pub sport_mode: bool,
    pub active_sound: bool,
}

impl Default for ExhaustNote {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustNote {
    pub fn new() -> Self {
        Self {
            frequency_hz: 250.0,
            volume_db: 70.0,
            valve_open: false,
            sport_mode: false,
            active_sound: false,
        }
    }

    pub fn legal(&self) -> bool {
        self.volume_db < 95.0
    }

    pub fn sporty(&self) -> bool {
        self.sport_mode && self.valve_open
    }

    pub fn quiet_mode(&self) -> bool {
        !self.valve_open && self.volume_db < 75.0
    }

    pub fn deep_tone(&self) -> bool {
        self.frequency_hz < 200.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.legal() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal() {
        let e = ExhaustNote::new();
        assert!(e.legal());
    }

    #[test]
    fn test_not_sporty() {
        let e = ExhaustNote::new();
        assert!(!e.sporty());
    }

    #[test]
    fn test_quiet() {
        let e = ExhaustNote::new();
        assert!(e.quiet_mode());
    }

    #[test]
    fn test_not_deep() {
        let e = ExhaustNote::new();
        assert!(!e.deep_tone());
    }

    #[test]
    fn test_too_loud() {
        let mut e = ExhaustNote::new();
        e.volume_db = 100.0;
        assert!(!e.legal());
    }

    #[test]
    fn test_health() {
        let e = ExhaustNote::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
