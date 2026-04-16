/// comp lz4v2: compress, decompress, stream, frame, log
/// Phase 2312

#[derive(Debug, Clone)]
pub struct CompLz4v2 {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub stream_ok: bool,
    pub frame_ok: bool,
    pub log_ok: bool,
}

impl Default for CompLz4v2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CompLz4v2 {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            decompress_ok: true,
            stream_ok: true,
            frame_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.decompress_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.frame_ok && self.log_ok
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
        let c = CompLz4v2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CompLz4v2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompLz4v2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CompLz4v2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CompLz4v2::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CompLz4v2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
