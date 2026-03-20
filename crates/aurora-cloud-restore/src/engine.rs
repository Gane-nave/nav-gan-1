/// cloud restore: select, download, decompress, apply, log
/// Phase 1448

#[derive(Debug, Clone)]
pub struct CloudRestore {
    pub select_ok: bool,
    pub download_ok: bool,
    pub decompress_ok: bool,
    pub apply_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudRestore {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudRestore {
    pub fn new() -> Self {
        Self {
            select_ok: true,
            download_ok: true,
            decompress_ok: true,
            apply_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.select_ok && self.download_ok && self.decompress_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.apply_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.select_ok || !self.download_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.select_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudRestore::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudRestore::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudRestore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudRestore::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudRestore::new();
        c.select_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudRestore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
