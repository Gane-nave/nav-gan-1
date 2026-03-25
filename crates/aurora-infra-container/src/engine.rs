/// infra container: create, start, stop, remove, log
/// Phase 2144

#[derive(Debug, Clone)]
pub struct InfraContainer {
    pub create_ok: bool,
    pub start_ok: bool,
    pub stop_ok: bool,
    pub remove_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraContainer {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            start_ok: true,
            stop_ok: true,
            remove_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.start_ok && self.stop_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.remove_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.start_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = InfraContainer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraContainer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraContainer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraContainer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraContainer::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraContainer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
