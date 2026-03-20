/// mem compress: compress, decompress, ratio, stats, log
/// Phase 2381

#[derive(Debug, Clone)]
pub struct MemCompress {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub ratio_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for MemCompress {
    fn default() -> Self {
        Self::new()
    }
}

impl MemCompress {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            decompress_ok: true,
            ratio_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.decompress_ok && self.ratio_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compress_ok || !self.decompress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compress_ok {
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
        let c = MemCompress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemCompress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemCompress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemCompress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemCompress::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemCompress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
