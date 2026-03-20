/// analytics realtime: ingest, process, query, alert, log
/// Phase 2193

#[derive(Debug, Clone)]
pub struct AnalyticsRealtime {
    pub ingest_ok: bool,
    pub process_ok: bool,
    pub query_ok: bool,
    pub alert_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsRealtime {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsRealtime {
    pub fn new() -> Self {
        Self {
            ingest_ok: true,
            process_ok: true,
            query_ok: true,
            alert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ingest_ok && self.process_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ingest_ok || !self.process_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ingest_ok {
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
        let c = AnalyticsRealtime::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsRealtime::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsRealtime::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsRealtime::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsRealtime::new();
        c.ingest_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsRealtime::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
