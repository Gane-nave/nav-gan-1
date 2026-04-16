/// aurora-dash-kpi: dash kpi
/// Phase 2451

#[derive(Debug, Clone)]
pub struct DashKpi {
    pub render_ok: bool,
    pub trend_ok: bool,
    pub target_ok: bool,
    pub alert_ok: bool,
    pub compare_ok: bool,
}

impl Default for DashKpi {
    fn default() -> Self {
        Self::new()
    }
}

impl DashKpi {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            trend_ok: true,
            target_ok: true,
            alert_ok: true,
            compare_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.trend_ok && self.target_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.compare_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.trend_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.render_ok {
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
        let c = DashKpi::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashKpi::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashKpi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashKpi::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashKpi::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashKpi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashKpi::default();
        assert!(c.all_ok());
    }
}
