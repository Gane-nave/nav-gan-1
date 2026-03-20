/// CI pipeline: build, test, lint, deploy, notify
/// Phase 1073

#[derive(Debug, Clone)]
pub struct CiPipeline {
    pub build_ok: bool,
    pub test_ok: bool,
    pub lint_ok: bool,
    pub deploy_ok: bool,
    pub notify_ok: bool,
}

impl Default for CiPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CiPipeline {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            test_ok: true,
            lint_ok: true,
            deploy_ok: true,
            notify_ok: true,
        }
    }

    pub fn validation_ok(&self) -> bool {
        self.build_ok && self.test_ok && self.lint_ok
    }

    pub fn delivery_ok(&self) -> bool {
        self.deploy_ok && self.notify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.validation_ok() && self.delivery_ok()
    }

    pub fn needs_retry(&self) -> bool {
        !self.build_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let c = CiPipeline::new();
        assert!(c.validation_ok());
    }

    #[test]
    fn test_delivery() {
        let c = CiPipeline::new();
        assert!(c.delivery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CiPipeline::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retry() {
        let c = CiPipeline::new();
        assert!(!c.needs_retry());
    }

    #[test]
    fn test_build() {
        let mut c = CiPipeline::new();
        c.build_ok = false;
        assert!(c.needs_retry());
    }

    #[test]
    fn test_health() {
        let c = CiPipeline::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
