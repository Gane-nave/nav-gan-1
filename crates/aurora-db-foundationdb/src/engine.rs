/// db foundationdb: connect, get, set, clear, log
/// Phase 1622

#[derive(Debug, Clone)]
pub struct DbFoundationdb {
    pub connect_ok: bool,
    pub get_ok: bool,
    pub set_ok: bool,
    pub clear_ok: bool,
    pub log_ok: bool,
}

impl Default for DbFoundationdb {
    fn default() -> Self {
        Self::new()
    }
}

impl DbFoundationdb {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            get_ok: true,
            set_ok: true,
            clear_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.get_ok && self.set_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = DbFoundationdb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbFoundationdb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbFoundationdb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbFoundationdb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbFoundationdb::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbFoundationdb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
