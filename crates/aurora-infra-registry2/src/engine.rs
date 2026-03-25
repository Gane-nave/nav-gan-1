/// infra registry2: push, pull, scan, clean, log
/// Phase 2140

#[derive(Debug, Clone)]
pub struct InfraRegistry2 {
    pub push_ok: bool,
    pub pull_ok: bool,
    pub scan_ok: bool,
    pub clean_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraRegistry2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraRegistry2 {
    pub fn new() -> Self {
        Self {
            push_ok: true,
            pull_ok: true,
            scan_ok: true,
            clean_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.push_ok && self.pull_ok && self.scan_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clean_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.push_ok || !self.pull_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.push_ok {
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
        let c = InfraRegistry2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraRegistry2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraRegistry2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraRegistry2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraRegistry2::new();
        c.push_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraRegistry2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
