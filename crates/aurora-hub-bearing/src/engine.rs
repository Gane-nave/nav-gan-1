/// Hub bearing: preload, seal, noise, ABS tone ring
/// Phase 668

#[derive(Debug, Clone)]
pub struct HubBearing {
    pub preload_ok: bool,
    pub seal_ok: bool,
    pub noise_free: bool,
    pub tone_ring_ok: bool,
    pub play_ok: bool,
}

impl Default for HubBearing {
    fn default() -> Self {
        Self::new()
    }
}

impl HubBearing {
    pub fn new() -> Self {
        Self {
            preload_ok: true,
            seal_ok: true,
            noise_free: true,
            tone_ring_ok: true,
            play_ok: true,
        }
    }

    pub fn bearing_ok(&self) -> bool {
        self.preload_ok && self.noise_free && self.play_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.seal_ok && self.tone_ring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.bearing_ok() && self.sealing_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.preload_ok || !self.noise_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.preload_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearing() {
        let c = HubBearing::new();
        assert!(c.bearing_ok());
    }

    #[test]
    fn test_sealing() {
        let c = HubBearing::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HubBearing::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HubBearing::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_preload() {
        let mut c = HubBearing::new();
        c.preload_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HubBearing::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
