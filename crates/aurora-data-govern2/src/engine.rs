/// data govern2: classify, policy, access, audit, log
/// Phase 2211

#[derive(Debug, Clone)]
pub struct DataGovern2 {
    pub classify_ok: bool,
    pub policy_ok: bool,
    pub access_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for DataGovern2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataGovern2 {
    pub fn new() -> Self {
        Self {
            classify_ok: true,
            policy_ok: true,
            access_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.classify_ok && self.policy_ok && self.access_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.classify_ok || !self.policy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.classify_ok {
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
        let c = DataGovern2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataGovern2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataGovern2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataGovern2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataGovern2::new();
        c.classify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataGovern2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
