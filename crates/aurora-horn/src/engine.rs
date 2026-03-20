/// Horn: sound level, frequency, relay, wiring
/// Phase 533

#[derive(Debug, Clone)]
pub struct Horn {
    pub sound_db: f64,
    pub min_db: f64,
    pub frequency_hz: f64,
    pub relay_ok: bool,
    pub wiring_ok: bool,
}

impl Default for Horn {
    fn default() -> Self {
        Self::new()
    }
}

impl Horn {
    pub fn new() -> Self {
        Self {
            sound_db: 105.0,
            min_db: 93.0,
            frequency_hz: 420.0,
            relay_ok: true,
            wiring_ok: true,
        }
    }

    pub fn volume_ok(&self) -> bool {
        self.sound_db > self.min_db
    }

    pub fn frequency_ok(&self) -> bool {
        self.frequency_hz > 300.0 && self.frequency_hz < 600.0
    }

    pub fn all_ok(&self) -> bool {
        self.volume_ok() && self.frequency_ok() && self.relay_ok && self.wiring_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.relay_ok || self.sound_db < self.min_db
    }

    pub fn health_score(&self) -> f64 {
        if !self.relay_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume() {
        let c = Horn::new();
        assert!(c.volume_ok());
    }

    #[test]
    fn test_frequency() {
        let c = Horn::new();
        assert!(c.frequency_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Horn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Horn::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_relay() {
        let mut c = Horn::new();
        c.relay_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Horn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
