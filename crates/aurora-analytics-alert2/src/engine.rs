/// analytics alert2: define, evaluate, notify, silence, log
/// Phase 2198

#[derive(Debug, Clone)]
pub struct AnalyticsAlert2 {
    pub define_ok: bool,
    pub evaluate_ok: bool,
    pub notify_ok: bool,
    pub silence_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsAlert2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsAlert2 {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            evaluate_ok: true,
            notify_ok: true,
            silence_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.evaluate_ok && self.notify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.silence_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.evaluate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = AnalyticsAlert2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsAlert2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsAlert2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsAlert2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsAlert2::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsAlert2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
