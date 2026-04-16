/// store log2: append, read, truncate, compact, log
/// Phase 1978

#[derive(Debug, Clone)]
pub struct StoreLog2 {
    pub append_ok: bool,
    pub read_ok: bool,
    pub truncate_ok: bool,
    pub compact_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreLog2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreLog2 {
    pub fn new() -> Self {
        Self {
            append_ok: true,
            read_ok: true,
            truncate_ok: true,
            compact_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.append_ok && self.read_ok && self.truncate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.append_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.append_ok {
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
        let c = StoreLog2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreLog2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreLog2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreLog2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreLog2::new();
        c.append_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreLog2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
