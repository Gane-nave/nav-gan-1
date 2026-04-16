/// wf audit2: record, query, export, retain, log
/// Phase 2235

#[derive(Debug, Clone)]
pub struct WfAudit2 {
    pub record_ok: bool,
    pub query_ok: bool,
    pub export_ok: bool,
    pub retain_ok: bool,
    pub log_ok: bool,
}

impl Default for WfAudit2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfAudit2 {
    pub fn new() -> Self {
        Self {
            record_ok: true,
            query_ok: true,
            export_ok: true,
            retain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.record_ok && self.query_ok && self.export_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.record_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.record_ok {
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
        let c = WfAudit2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfAudit2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfAudit2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfAudit2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfAudit2::new();
        c.record_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfAudit2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
