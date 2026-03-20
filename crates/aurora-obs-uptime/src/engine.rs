/// obs uptime: monitor, alert, report, trend, log
/// Phase 2154

#[derive(Debug, Clone)]
pub struct ObsUptime {
    pub monitor_ok: bool,
    pub alert_ok: bool,
    pub report_ok: bool,
    pub trend_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsUptime {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsUptime {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            alert_ok: true,
            report_ok: true,
            trend_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.alert_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.trend_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.alert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok {
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
        let c = ObsUptime::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsUptime::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsUptime::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsUptime::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsUptime::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsUptime::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
