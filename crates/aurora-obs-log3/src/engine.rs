/// obs log3: ingest, query, aggregate, retain, log
/// Phase 2150

#[derive(Debug, Clone)]
pub struct ObsLog3 {
    pub ingest_ok: bool,
    pub query_ok: bool,
    pub aggregate_ok: bool,
    pub retain_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsLog3 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsLog3 {
    pub fn new() -> Self {
        Self {
            ingest_ok: true,
            query_ok: true,
            aggregate_ok: true,
            retain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ingest_ok && self.query_ok && self.aggregate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ingest_ok || !self.query_ok
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
        let c = ObsLog3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsLog3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsLog3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsLog3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsLog3::new();
        c.ingest_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsLog3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
