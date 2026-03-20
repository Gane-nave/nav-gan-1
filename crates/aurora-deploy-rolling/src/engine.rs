/// deploy rolling: batch, update, verify, complete, log
/// Phase 1592

#[derive(Debug, Clone)]
pub struct DeployRolling {
    pub batch_ok: bool,
    pub update_ok: bool,
    pub verify_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployRolling {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployRolling {
    pub fn new() -> Self {
        Self {
            batch_ok: true,
            update_ok: true,
            verify_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.batch_ok && self.update_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.batch_ok || !self.update_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.batch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployRolling::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployRolling::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployRolling::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployRolling::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployRolling::new();
        c.batch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployRolling::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
