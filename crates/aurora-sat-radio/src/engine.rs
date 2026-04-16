/// sat radio: acquire, track, decode, relay, log
/// Phase 1347

#[derive(Debug, Clone)]
pub struct SatRadio {
    pub acquire_ok: bool,
    pub track_ok: bool,
    pub decode_ok: bool,
    pub relay_ok: bool,
    pub log_ok: bool,
}

impl Default for SatRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl SatRadio {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            track_ok: true,
            decode_ok: true,
            relay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.track_ok && self.decode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.relay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.acquire_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.acquire_ok {
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
        let c = SatRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SatRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SatRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SatRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SatRadio::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SatRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
