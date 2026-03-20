/// safety log: record, classify, archive, retrieve, log
/// Phase 1484

#[derive(Debug, Clone)]
pub struct SafetyLog {
    pub record_ok: bool,
    pub classify_ok: bool,
    pub archive_ok: bool,
    pub retrieve_ok: bool,
    pub log_ok: bool,
}

impl Default for SafetyLog {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyLog {
    pub fn new() -> Self {
        Self {
            record_ok: true,
            classify_ok: true,
            archive_ok: true,
            retrieve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.record_ok && self.classify_ok && self.archive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retrieve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.record_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.record_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SafetyLog::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SafetyLog::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SafetyLog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SafetyLog::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SafetyLog::new();
        c.record_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SafetyLog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
