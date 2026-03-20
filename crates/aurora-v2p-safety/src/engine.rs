/// v2p safety: detect, warn, brake, track, log
/// Phase 1125

#[derive(Debug, Clone)]
pub struct V2pSafety {
    pub detect_ok: bool,
    pub warn_ok: bool,
    pub brake_ok: bool,
    pub track_ok: bool,
    pub log_ok: bool,
}

impl Default for V2pSafety {
    fn default() -> Self {
        Self::new()
    }
}

impl V2pSafety {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            warn_ok: true,
            brake_ok: true,
            track_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.warn_ok && self.brake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.track_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.warn_ok
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
        let c = V2pSafety::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2pSafety::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2pSafety::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2pSafety::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2pSafety::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2pSafety::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
