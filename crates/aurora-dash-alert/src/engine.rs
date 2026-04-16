/// aurora-dash-alert: dash alert
/// Phase 2450

#[derive(Debug, Clone)]
pub struct DashAlert {
    pub trigger_ok: bool,
    pub dismiss_ok: bool,
    pub severity_ok: bool,
    pub log_ok: bool,
    pub route_ok: bool,
}

impl Default for DashAlert {
    fn default() -> Self {
        Self::new()
    }
}

impl DashAlert {
    pub fn new() -> Self {
        Self {
            trigger_ok: true,
            dismiss_ok: true,
            severity_ok: true,
            log_ok: true,
            route_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.trigger_ok && self.dismiss_ok && self.severity_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.log_ok && self.route_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.trigger_ok || !self.dismiss_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trigger_ok {
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
        let c = DashAlert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashAlert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashAlert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashAlert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashAlert::new();
        c.trigger_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashAlert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashAlert::default();
        assert!(c.all_ok());
    }
}
