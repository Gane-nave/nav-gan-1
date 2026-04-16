/// codec parquet2: write, read, schema, stats, log
/// Phase 2030

#[derive(Debug, Clone)]
pub struct CodecParquet2 {
    pub write_ok: bool,
    pub read_ok: bool,
    pub schema_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for CodecParquet2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecParquet2 {
    pub fn new() -> Self {
        Self {
            write_ok: true,
            read_ok: true,
            schema_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.write_ok && self.read_ok && self.schema_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.write_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.write_ok {
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
        let c = CodecParquet2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CodecParquet2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CodecParquet2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CodecParquet2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CodecParquet2::new();
        c.write_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CodecParquet2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
