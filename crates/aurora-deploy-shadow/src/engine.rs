/// deploy shadow: mirror, compare, report, switch, log
/// Phase 2123

#[derive(Debug, Clone)]
pub struct DeployShadow {
    pub mirror_ok: bool,
    pub compare_ok: bool,
    pub report_ok: bool,
    pub switch_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployShadow {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployShadow {
    pub fn new() -> Self {
        Self {
            mirror_ok: true,
            compare_ok: true,
            report_ok: true,
            switch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mirror_ok && self.compare_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.switch_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mirror_ok || !self.compare_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mirror_ok {
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
        let c = DeployShadow::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployShadow::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployShadow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployShadow::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployShadow::new();
        c.mirror_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployShadow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
