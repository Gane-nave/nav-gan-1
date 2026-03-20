/// Horn system: relay control, dual tone, emergency horn
/// Phase 242

#[derive(Debug, Clone)]
pub struct HornSystem {
    pub relay_ok: bool,
    pub horn_active: bool,
    pub dual_tone: bool,
    pub current_draw_a: f64,
    pub sound_level_db: f64,
}

impl Default for HornSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HornSystem {
    pub fn new() -> Self {
        Self {
            relay_ok: true,
            horn_active: false,
            dual_tone: true,
            current_draw_a: 0.0,
            sound_level_db: 0.0,
        }
    }

    pub fn is_sounding(&self) -> bool {
        self.horn_active && self.relay_ok
    }

    pub fn volume_ok(&self) -> bool {
        !self.horn_active || self.sound_level_db > 100.0
    }

    pub fn current_ok(&self) -> bool {
        self.current_draw_a < 20.0
    }

    pub fn needs_service(&self) -> bool {
        !self.relay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.relay_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_sounding() {
        let h = HornSystem::new();
        assert!(!h.is_sounding());
    }

    #[test]
    fn test_volume_ok() {
        let h = HornSystem::new();
        assert!(h.volume_ok());
    }

    #[test]
    fn test_current_ok() {
        let h = HornSystem::new();
        assert!(h.current_ok());
    }

    #[test]
    fn test_no_service() {
        let h = HornSystem::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_relay_fail() {
        let mut h = HornSystem::new();
        h.relay_ok = false;
        assert!(h.needs_service());
    }

    #[test]
    fn test_health() {
        let h = HornSystem::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
