/// trans compress2: negotiate, compress, decompress, flush, log
/// Phase 2281

#[derive(Debug, Clone)]
pub struct TransCompress2 {
    pub negotiate_ok: bool,
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for TransCompress2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransCompress2 {
    pub fn new() -> Self {
        Self {
            negotiate_ok: true,
            compress_ok: true,
            decompress_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.negotiate_ok && self.compress_ok && self.decompress_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.negotiate_ok || !self.compress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.negotiate_ok {
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
        let c = TransCompress2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransCompress2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransCompress2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransCompress2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransCompress2::new();
        c.negotiate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransCompress2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
