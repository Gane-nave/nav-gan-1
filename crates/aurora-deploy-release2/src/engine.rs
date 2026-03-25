/// deploy release2: tag, build, publish, notify, log
/// Phase 2126

#[derive(Debug, Clone)]
pub struct DeployRelease2 {
    pub tag_ok: bool,
    pub build_ok: bool,
    pub publish_ok: bool,
    pub notify_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployRelease2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployRelease2 {
    pub fn new() -> Self {
        Self {
            tag_ok: true,
            build_ok: true,
            publish_ok: true,
            notify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tag_ok && self.build_ok && self.publish_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.notify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tag_ok || !self.build_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tag_ok {
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
        let c = DeployRelease2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployRelease2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployRelease2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployRelease2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployRelease2::new();
        c.tag_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployRelease2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
