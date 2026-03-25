/// compress snappy: compress, decompress, stream, validate, log
/// Phase 1719

#[derive(Debug, Clone)]
pub struct CompressSnappy {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub stream_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for CompressSnappy {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressSnappy {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            decompress_ok: true,
            stream_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.decompress_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
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
        let c = CompressSnappy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CompressSnappy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompressSnappy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CompressSnappy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CompressSnappy::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CompressSnappy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
