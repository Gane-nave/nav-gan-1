/// blind spot: detect, track, warn, inhibit, clear
/// Phase 1326

#[derive(Debug, Clone)]
pub struct BlindSpot2 {
    pub detect_ok: bool,
    pub track_ok: bool,
    pub warn_ok: bool,
    pub inhibit_ok: bool,
    pub clear_ok: bool,
}

impl Default for BlindSpot2 {
    fn default() -> Self {
        Self::new()
    }
}

impl BlindSpot2 {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            track_ok: true,
            warn_ok: true,
            inhibit_ok: true,
            clear_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.track_ok && self.warn_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.inhibit_ok && self.clear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BlindSpot2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BlindSpot2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlindSpot2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BlindSpot2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BlindSpot2::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BlindSpot2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
