/// comp brotli2: compress, decompress, stream, quality, log
/// Phase 2314

#[derive(Debug, Clone)]
pub struct CompBrotli2 {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub stream_ok: bool,
    pub quality_ok: bool,
    pub log_ok: bool,
}

impl Default for CompBrotli2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CompBrotli2 {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            decompress_ok: true,
            stream_ok: true,
            quality_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.decompress_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.quality_ok && self.log_ok
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
        let c = CompBrotli2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CompBrotli2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompBrotli2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CompBrotli2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CompBrotli2::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CompBrotli2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
