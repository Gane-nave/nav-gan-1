/// analytics session: start, track, end, summarize, log
/// Phase 1555

#[derive(Debug, Clone)]
pub struct AnalyticsSession {
    pub start_ok: bool,
    pub track_ok: bool,
    pub end_ok: bool,
    pub summarize_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsSession {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsSession {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            track_ok: true,
            end_ok: true,
            summarize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.track_ok && self.end_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.summarize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = AnalyticsSession::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsSession::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsSession::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsSession::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsSession::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsSession::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
