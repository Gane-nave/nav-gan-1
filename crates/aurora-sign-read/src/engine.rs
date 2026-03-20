/// sign read: capture, segment, classify, display, log
/// Phase 1332

#[derive(Debug, Clone)]
pub struct SignRead {
    pub capture_ok: bool,
    pub segment_ok: bool,
    pub classify_ok: bool,
    pub display_ok: bool,
    pub log_ok: bool,
}

impl Default for SignRead {
    fn default() -> Self {
        Self::new()
    }
}

impl SignRead {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            segment_ok: true,
            classify_ok: true,
            display_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.segment_ok && self.classify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.display_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.segment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SignRead::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SignRead::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SignRead::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SignRead::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SignRead::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SignRead::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
