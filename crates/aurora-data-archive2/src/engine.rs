/// data archive2: select, compress, store, retrieve, log
/// Phase 2213

#[derive(Debug, Clone)]
pub struct DataArchive2 {
    pub select_ok: bool,
    pub compress_ok: bool,
    pub store_ok: bool,
    pub retrieve_ok: bool,
    pub log_ok: bool,
}

impl Default for DataArchive2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataArchive2 {
    pub fn new() -> Self {
        Self {
            select_ok: true,
            compress_ok: true,
            store_ok: true,
            retrieve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.select_ok && self.compress_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retrieve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.select_ok || !self.compress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.select_ok {
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
        let c = DataArchive2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataArchive2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataArchive2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataArchive2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataArchive2::new();
        c.select_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataArchive2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
