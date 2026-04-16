/// fleet alert: detect, classify, notify, escalate, log
/// Phase 1418

#[derive(Debug, Clone)]
pub struct FleetAlert {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub notify_ok: bool,
    pub escalate_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetAlert {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetAlert {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            notify_ok: true,
            escalate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.notify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.escalate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.classify_ok
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
        let c = FleetAlert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetAlert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetAlert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetAlert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetAlert::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetAlert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
