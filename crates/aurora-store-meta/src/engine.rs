/// store meta: set, get, delete, list, log
/// Phase 1979

#[derive(Debug, Clone)]
pub struct StoreMeta {
    pub set_ok: bool,
    pub get_ok: bool,
    pub delete_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreMeta {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            get_ok: true,
            delete_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.get_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = StoreMeta::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreMeta::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreMeta::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreMeta::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreMeta::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreMeta::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
