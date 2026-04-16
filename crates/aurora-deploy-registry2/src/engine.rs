/// deploy registry: publish, tag, pull, scan, log
/// Phase 1598

#[derive(Debug, Clone)]
pub struct DeployRegistry2 {
    pub publish_ok: bool,
    pub tag_ok: bool,
    pub pull_ok: bool,
    pub scan_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployRegistry2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployRegistry2 {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            tag_ok: true,
            pull_ok: true,
            scan_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.tag_ok && self.pull_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scan_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.publish_ok || !self.tag_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.publish_ok {
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
        let c = DeployRegistry2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployRegistry2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployRegistry2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployRegistry2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployRegistry2::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployRegistry2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
