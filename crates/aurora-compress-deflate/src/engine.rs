/// compress deflate: compress, decompress, stream, level, log
/// Phase 1721

#[derive(Debug, Clone)]
pub struct CompressDeflate {
    pub compress_ok: bool,
    pub decompress_ok: bool,
    pub stream_ok: bool,
    pub level_ok: bool,
    pub log_ok: bool,
}

impl Default for CompressDeflate {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressDeflate {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            decompress_ok: true,
            stream_ok: true,
            level_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.decompress_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.level_ok && self.log_ok
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
        let c = CompressDeflate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CompressDeflate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompressDeflate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CompressDeflate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CompressDeflate::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CompressDeflate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
