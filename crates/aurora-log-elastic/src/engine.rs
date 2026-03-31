/// aurora-log-elastic: log elastic
/// Phase 2614

#[derive(Debug, Clone)]
pub struct LogElastic {
    pub index_ok: bool,
    pub search_ok: bool,
    pub aggregate_ok: bool,
    pub alert_ok: bool,
    pub export_ok: bool,
}

impl Default for LogElastic {
    fn default() -> Self {
        Self::new()
    }
}

impl LogElastic {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            search_ok: true,
            aggregate_ok: true,
            alert_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.index_ok && self.search_ok && self.aggregate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.index_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.index_ok {
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
        let c = LogElastic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LogElastic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LogElastic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LogElastic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LogElastic::new();
        c.index_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LogElastic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = LogElastic::default();
        assert!(c.all_ok());
    }
}
