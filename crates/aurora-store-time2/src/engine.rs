/// store time2: write, query, downsample, retain, log
/// Phase 1971

#[derive(Debug, Clone)]
pub struct StoreTime2 {
    pub write_ok: bool,
    pub query_ok: bool,
    pub downsample_ok: bool,
    pub retain_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreTime2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreTime2 {
    pub fn new() -> Self {
        Self {
            write_ok: true,
            query_ok: true,
            downsample_ok: true,
            retain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.write_ok && self.query_ok && self.downsample_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.write_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.write_ok {
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
        let c = StoreTime2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreTime2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreTime2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreTime2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreTime2::new();
        c.write_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreTime2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
