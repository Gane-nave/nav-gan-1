/// aurora-svc-trace: svc trace
/// Phase 2573

#[derive(Debug, Clone)]
pub struct SvcTrace {
    pub start_ok: bool,
    pub end_ok: bool,
    pub tag_ok: bool,
    pub log_ok: bool,
    pub export_ok: bool,
}

impl Default for SvcTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcTrace {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            end_ok: true,
            tag_ok: true,
            log_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.end_ok && self.tag_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.log_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.end_ok
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
        let c = SvcTrace::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcTrace::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcTrace::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcTrace::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcTrace::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcTrace::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcTrace::default();
        assert!(c.all_ok());
    }
}
