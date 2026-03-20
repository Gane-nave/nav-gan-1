/// ldw sys: track, drift, warn, vibrate, log
/// Phase 1162

#[derive(Debug, Clone)]
pub struct LdwSys {
    pub track_ok: bool,
    pub drift_ok: bool,
    pub warn_ok: bool,
    pub vibrate_ok: bool,
    pub log_ok: bool,
}

impl Default for LdwSys {
    fn default() -> Self {
        Self::new()
    }
}

impl LdwSys {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            drift_ok: true,
            warn_ok: true,
            vibrate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.drift_ok && self.warn_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.vibrate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.drift_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = LdwSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LdwSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LdwSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LdwSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LdwSys::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LdwSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
