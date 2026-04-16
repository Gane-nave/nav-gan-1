/// fleet comply: check, document, audit, certify, log
/// Phase 1421

#[derive(Debug, Clone)]
pub struct FleetComply {
    pub check_ok: bool,
    pub document_ok: bool,
    pub audit_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetComply {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetComply {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            document_ok: true,
            audit_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.document_ok && self.audit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.document_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = FleetComply::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetComply::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetComply::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetComply::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetComply::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetComply::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
