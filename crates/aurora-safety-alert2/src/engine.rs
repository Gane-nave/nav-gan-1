/// safety alert: detect, classify, escalate, resolve, log
/// Phase 1485

#[derive(Debug, Clone)]
pub struct SafetyAlert2 {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub escalate_ok: bool,
    pub resolve_ok: bool,
    pub log_ok: bool,
}

impl Default for SafetyAlert2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyAlert2 {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            escalate_ok: true,
            resolve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.escalate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resolve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SafetyAlert2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SafetyAlert2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SafetyAlert2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SafetyAlert2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SafetyAlert2::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SafetyAlert2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
