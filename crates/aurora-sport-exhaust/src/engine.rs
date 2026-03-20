/// Sport exhaust: valve, sound, mode, backfire, rumble
/// Phase 946

#[derive(Debug, Clone)]
pub struct SportExhaust {
    pub valve_ok: bool,
    pub sound_ok: bool,
    pub mode_ok: bool,
    pub backfire_ok: bool,
    pub rumble_ok: bool,
}

impl Default for SportExhaust {
    fn default() -> Self {
        Self::new()
    }
}

impl SportExhaust {
    pub fn new() -> Self {
        Self {
            valve_ok: true,
            sound_ok: true,
            mode_ok: true,
            backfire_ok: true,
            rumble_ok: true,
        }
    }

    pub fn actuation_ok(&self) -> bool {
        self.valve_ok && self.mode_ok
    }

    pub fn acoustics_ok(&self) -> bool {
        self.sound_ok && self.backfire_ok && self.rumble_ok
    }

    pub fn all_ok(&self) -> bool {
        self.actuation_ok() && self.acoustics_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.valve_ok || !self.mode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.valve_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuation() {
        let c = SportExhaust::new();
        assert!(c.actuation_ok());
    }

    #[test]
    fn test_acoustics() {
        let c = SportExhaust::new();
        assert!(c.acoustics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SportExhaust::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SportExhaust::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_valve() {
        let mut c = SportExhaust::new();
        c.valve_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SportExhaust::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
