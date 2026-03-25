/// fmt parquet: read, write, validate, compress, log
/// Phase 1667

#[derive(Debug, Clone)]
pub struct FmtParquet {
    pub read_ok: bool,
    pub write_ok: bool,
    pub validate_ok: bool,
    pub compress_ok: bool,
    pub log_ok: bool,
}

impl Default for FmtParquet {
    fn default() -> Self {
        Self::new()
    }
}

impl FmtParquet {
    pub fn new() -> Self {
        Self {
            read_ok: true,
            write_ok: true,
            validate_ok: true,
            compress_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.read_ok && self.write_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compress_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.read_ok || !self.write_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.read_ok {
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
        let c = FmtParquet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FmtParquet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FmtParquet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FmtParquet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FmtParquet::new();
        c.read_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FmtParquet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
