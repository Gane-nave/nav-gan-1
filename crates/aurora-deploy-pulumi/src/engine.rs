/// deploy pulumi: preview, update, destroy, export, log
/// Phase 2117

#[derive(Debug, Clone)]
pub struct DeployPulumi {
    pub preview_ok: bool,
    pub update_ok: bool,
    pub destroy_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployPulumi {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployPulumi {
    pub fn new() -> Self {
        Self {
            preview_ok: true,
            update_ok: true,
            destroy_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.preview_ok && self.update_ok && self.destroy_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.preview_ok || !self.update_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.preview_ok {
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
        let c = DeployPulumi::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployPulumi::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployPulumi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployPulumi::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployPulumi::new();
        c.preview_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployPulumi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
