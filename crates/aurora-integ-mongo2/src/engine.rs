/// integ mongo2: connect, find, insert, update, log
/// Phase 2255

#[derive(Debug, Clone)]
pub struct IntegMongo2 {
    pub connect_ok: bool,
    pub find_ok: bool,
    pub insert_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegMongo2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegMongo2 {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            find_ok: true,
            insert_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.find_ok && self.insert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.find_ok
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
        let c = IntegMongo2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegMongo2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegMongo2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegMongo2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegMongo2::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegMongo2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
