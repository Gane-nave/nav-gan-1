/// analytics dashboard3: create, configure, share, refresh, log
/// Phase 2197

#[derive(Debug, Clone)]
pub struct AnalyticsDashboard3 {
    pub create_ok: bool,
    pub configure_ok: bool,
    pub share_ok: bool,
    pub refresh_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsDashboard3 {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsDashboard3 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            configure_ok: true,
            share_ok: true,
            refresh_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.configure_ok && self.share_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.refresh_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.configure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = AnalyticsDashboard3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsDashboard3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsDashboard3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsDashboard3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsDashboard3::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsDashboard3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
