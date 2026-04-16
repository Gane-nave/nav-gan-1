/// trans keepalive: probe, detect, timeout, recover, log
/// Phase 2284

#[derive(Debug, Clone)]
pub struct TransKeepalive {
    pub probe_ok: bool,
    pub detect_ok: bool,
    pub timeout_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for TransKeepalive {
    fn default() -> Self {
        Self::new()
    }
}

impl TransKeepalive {
    pub fn new() -> Self {
        Self {
            probe_ok: true,
            detect_ok: true,
            timeout_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.probe_ok && self.detect_ok && self.timeout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.probe_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.probe_ok {
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
        let c = TransKeepalive::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransKeepalive::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransKeepalive::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransKeepalive::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransKeepalive::new();
        c.probe_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransKeepalive::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
