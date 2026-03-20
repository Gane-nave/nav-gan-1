/// cloud log: collect, index, search, archive, log
/// Phase 1453

#[derive(Debug, Clone)]
pub struct CloudLog {
    pub collect_ok: bool,
    pub index_ok: bool,
    pub search_ok: bool,
    pub archive_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudLog {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudLog {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            index_ok: true,
            search_ok: true,
            archive_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.index_ok && self.search_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.archive_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.index_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = CloudLog::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudLog::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudLog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudLog::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudLog::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudLog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
