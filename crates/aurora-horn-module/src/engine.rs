/// Horn module: relay, switch, tone, ground
/// Phase 678

#[derive(Debug, Clone)]
pub struct HornModule {
    pub relay_ok: bool,
    pub switch_ok: bool,
    pub tone_ok: bool,
    pub ground_ok: bool,
    pub volume_ok: bool,
}

impl Default for HornModule {
    fn default() -> Self {
        Self::new()
    }
}

impl HornModule {
    pub fn new() -> Self {
        Self {
            relay_ok: true,
            switch_ok: true,
            tone_ok: true,
            ground_ok: true,
            volume_ok: true,
        }
    }

    pub fn circuit_ok(&self) -> bool {
        self.relay_ok && self.switch_ok && self.ground_ok
    }

    pub fn sound_ok(&self) -> bool {
        self.tone_ok && self.volume_ok
    }

    pub fn all_ok(&self) -> bool {
        self.circuit_ok() && self.sound_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.relay_ok || !self.tone_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.relay_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let c = HornModule::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_sound() {
        let c = HornModule::new();
        assert!(c.sound_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HornModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HornModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_relay() {
        let mut c = HornModule::new();
        c.relay_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HornModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
