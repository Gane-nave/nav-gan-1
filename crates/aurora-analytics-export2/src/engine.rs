/// analytics export2: select, format, deliver, schedule, log
/// Phase 2195

#[derive(Debug, Clone)]
pub struct AnalyticsExport2 {
    pub select_ok: bool,
    pub format_ok: bool,
    pub deliver_ok: bool,
    pub schedule_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsExport2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsExport2 {
    pub fn new() -> Self {
        Self {
            select_ok: true,
            format_ok: true,
            deliver_ok: true,
            schedule_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.select_ok && self.format_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.schedule_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.select_ok || !self.format_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.select_ok {
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
        let c = AnalyticsExport2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsExport2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsExport2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsExport2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsExport2::new();
        c.select_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsExport2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
