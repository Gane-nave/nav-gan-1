/// comp snappy2: compress, decompress, stream, validate, log
/// Phase 2313

#[derive(Debug, Clone)]
pub struct CompSnappy2 {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub stream_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for CompSnappy2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CompSnappy2 {
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
        let c = CompSnappy2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CompSnappy2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompSnappy2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CompSnappy2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CompSnappy2::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CompSnappy2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
