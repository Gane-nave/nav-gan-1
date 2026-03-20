/// deploy blue: prepare, switch, verify, rollback, log
/// Phase 1590

#[derive(Debug, Clone)]
pub struct DeployBlue {
    pub prepare_ok: bool,
    pub switch_ok: bool,
    pub verify_ok: bool,
    pub rollback_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployBlue {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployBlue {
    pub fn new() -> Self {
        Self {
            prepare_ok: true,
            switch_ok: true,
            verify_ok: true,
            rollback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.prepare_ok && self.switch_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.prepare_ok || !self.switch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.prepare_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployBlue::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployBlue::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployBlue::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployBlue::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployBlue::new();
        c.prepare_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployBlue::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
