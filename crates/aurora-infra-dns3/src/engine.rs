/// infra dns3: create, update, delete, resolve, log
/// Phase 2134

#[derive(Debug, Clone)]
pub struct InfraDns3 {
    pub create_ok: bool,
    pub update_ok: bool,
    pub delete_ok: bool,
    pub resolve_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraDns3 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraDns3 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            update_ok: true,
            delete_ok: true,
            resolve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.update_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resolve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.update_ok
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
        let c = InfraDns3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraDns3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraDns3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraDns3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraDns3::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraDns3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
