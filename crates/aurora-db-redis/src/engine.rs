/// db redis: connect, get, set, del_key, log
/// Phase 1611

#[derive(Debug, Clone)]
pub struct DbRedis {
    pub connect_ok: bool,
    pub get_ok: bool,
    pub set_ok: bool,
    pub del_key_ok: bool,
    pub log_ok: bool,
}

impl Default for DbRedis {
    fn default() -> Self {
        Self::new()
    }
}

impl DbRedis {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            get_ok: true,
            set_ok: true,
            del_key_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.get_ok && self.set_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.del_key_ok && self.log_ok
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
        let c = DbRedis::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbRedis::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbRedis::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbRedis::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbRedis::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbRedis::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
