/// data mart: define, populate, refresh, query, log
/// Phase 2219

#[derive(Debug, Clone)]
pub struct DataMart {
    pub define_ok: bool,
    pub populate_ok: bool,
    pub refresh_ok: bool,
    pub query_ok: bool,
    pub log_ok: bool,
}

impl Default for DataMart {
    fn default() -> Self {
        Self::new()
    }
}

impl DataMart {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            populate_ok: true,
            refresh_ok: true,
            query_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.populate_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.query_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.populate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = DataMart::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataMart::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataMart::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataMart::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataMart::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataMart::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
