/// devops docs2: generate, build, publish, version, log
/// Phase 2176

#[derive(Debug, Clone)]
pub struct DevopsDocs2 {
    pub generate_ok: bool,
    pub build_ok: bool,
    pub publish_ok: bool,
    pub version_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsDocs2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsDocs2 {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            build_ok: true,
            publish_ok: true,
            version_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.build_ok && self.publish_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.version_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.build_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = DevopsDocs2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsDocs2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsDocs2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsDocs2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsDocs2::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsDocs2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
