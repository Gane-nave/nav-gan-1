/// devops tag: create, push, delete, list, log
/// Phase 2174

#[derive(Debug, Clone)]
pub struct DevopsTag {
    pub create_ok: bool,
    pub push_ok: bool,
    pub delete_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsTag {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsTag {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            push_ok: true,
            delete_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.push_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.push_ok
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
        let c = DevopsTag::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsTag::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsTag::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsTag::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsTag::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsTag::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
