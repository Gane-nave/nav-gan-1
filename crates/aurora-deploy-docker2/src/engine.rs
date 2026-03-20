/// deploy docker: build, push, pull, compose, log
/// Phase 1602

#[derive(Debug, Clone)]
pub struct DeployDocker2 {
    pub build_ok: bool,
    pub push_ok: bool,
    pub pull_ok: bool,
    pub compose_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployDocker2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployDocker2 {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            push_ok: true,
            pull_ok: true,
            compose_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.build_ok && self.push_ok && self.pull_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compose_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.build_ok || !self.push_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok {
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
        let c = DeployDocker2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployDocker2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployDocker2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployDocker2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployDocker2::new();
        c.build_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployDocker2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
