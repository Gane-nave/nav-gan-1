/// tim alert: zone, severity, duration, action, acknowledge
/// Phase 1131

#[derive(Debug, Clone)]
pub struct TimAlert {
    pub zone_ok: bool,
    pub severity_ok: bool,
    pub duration_ok: bool,
    pub action_ok: bool,
    pub acknowledge_ok: bool,
}

impl Default for TimAlert {
    fn default() -> Self {
        Self::new()
    }
}

impl TimAlert {
    pub fn new() -> Self {
        Self {
            zone_ok: true,
            severity_ok: true,
            duration_ok: true,
            action_ok: true,
            acknowledge_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.zone_ok && self.severity_ok && self.duration_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.action_ok && self.acknowledge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.zone_ok || !self.severity_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.zone_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TimAlert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TimAlert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TimAlert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TimAlert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TimAlert::new();
        c.zone_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TimAlert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
