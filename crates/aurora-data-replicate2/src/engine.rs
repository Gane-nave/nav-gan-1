/// data replicate2: source, target, sync, monitor, log
/// Phase 2214

#[derive(Debug, Clone)]
pub struct DataReplicate2 {
    pub source_ok: bool,
    pub target_ok: bool,
    pub sync_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for DataReplicate2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataReplicate2 {
    pub fn new() -> Self {
        Self {
            source_ok: true,
            target_ok: true,
            sync_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.source_ok && self.target_ok && self.sync_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.source_ok || !self.target_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.source_ok {
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
        let c = DataReplicate2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataReplicate2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataReplicate2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataReplicate2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataReplicate2::new();
        c.source_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataReplicate2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
