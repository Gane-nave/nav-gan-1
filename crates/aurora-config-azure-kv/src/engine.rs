/// config azure kv: get, set, list, delete, log
/// Phase 1779

#[derive(Debug, Clone)]
pub struct ConfigAzureKv {
    pub get_ok: bool,
    pub set_ok: bool,
    pub list_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for ConfigAzureKv {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigAzureKv {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            set_ok: true,
            list_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.set_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.set_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
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
        let c = ConfigAzureKv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConfigAzureKv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConfigAzureKv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConfigAzureKv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConfigAzureKv::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConfigAzureKv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
