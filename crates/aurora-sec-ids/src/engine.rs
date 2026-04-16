/// sec ids: detect, alert, classify, respond, log
/// Phase 2055

#[derive(Debug, Clone)]
pub struct SecIds {
    pub detect_ok: bool,
    pub alert_ok: bool,
    pub classify_ok: bool,
    pub respond_ok: bool,
    pub log_ok: bool,
}

impl Default for SecIds {
    fn default() -> Self {
        Self::new()
    }
}

impl SecIds {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            alert_ok: true,
            classify_ok: true,
            respond_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.alert_ok && self.classify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.respond_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.alert_ok
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
        let c = SecIds::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecIds::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecIds::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecIds::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecIds::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecIds::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
