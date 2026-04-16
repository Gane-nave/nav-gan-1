/// codec flatbuf2: build, read, verify, convert, log
/// Phase 2034

#[derive(Debug, Clone)]
pub struct CodecFlatbuf2 {
    pub build_ok: bool,
    pub read_ok: bool,
    pub verify_ok: bool,
    pub convert_ok: bool,
    pub log_ok: bool,
}

impl Default for CodecFlatbuf2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecFlatbuf2 {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            read_ok: true,
            verify_ok: true,
            convert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.build_ok && self.read_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.convert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.build_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok {
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
        let c = CodecFlatbuf2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CodecFlatbuf2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CodecFlatbuf2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CodecFlatbuf2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CodecFlatbuf2::new();
        c.build_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CodecFlatbuf2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
