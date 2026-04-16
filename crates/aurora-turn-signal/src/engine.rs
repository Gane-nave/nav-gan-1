/// Turn signal: flasher, bulb, lens, relay
/// Phase 675

#[derive(Debug, Clone)]
pub struct TurnSignal {
    pub flasher_ok: bool,
    pub bulb_ok: bool,
    pub lens_ok: bool,
    pub relay_ok: bool,
    pub rate_ok: bool,
}

impl Default for TurnSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnSignal {
    pub fn new() -> Self {
        Self {
            flasher_ok: true,
            bulb_ok: true,
            lens_ok: true,
            relay_ok: true,
            rate_ok: true,
        }
    }

    pub fn signal_ok(&self) -> bool {
        self.flasher_ok && self.bulb_ok && self.rate_ok
    }

    pub fn housing_ok(&self) -> bool {
        self.lens_ok && self.relay_ok
    }

    pub fn all_ok(&self) -> bool {
        self.signal_ok() && self.housing_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bulb_ok || !self.flasher_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flasher_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let c = TurnSignal::new();
        assert!(c.signal_ok());
    }

    #[test]
    fn test_housing() {
        let c = TurnSignal::new();
        assert!(c.housing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TurnSignal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TurnSignal::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bulb() {
        let mut c = TurnSignal::new();
        c.bulb_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TurnSignal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
