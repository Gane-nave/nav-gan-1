/// devops secret2: create, rotate, inject, audit, log
/// Phase 2178

#[derive(Debug, Clone)]
pub struct DevopsSecret2 {
    pub create_ok: bool,
    pub rotate_ok: bool,
    pub inject_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsSecret2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsSecret2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            rotate_ok: true,
            inject_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.rotate_ok && self.inject_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.rotate_ok
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
        let c = DevopsSecret2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsSecret2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsSecret2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsSecret2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsSecret2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsSecret2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
