/// report csv: format, encode, stream, compress, log
/// Phase 1565

#[derive(Debug, Clone)]
pub struct ReportCsv {
    pub format_ok: bool,
    pub encode_ok: bool,
    pub stream_ok: bool,
    pub compress_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportCsv {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportCsv {
    pub fn new() -> Self {
        Self {
            format_ok: true,
            encode_ok: true,
            stream_ok: true,
            compress_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.format_ok && self.encode_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compress_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.format_ok || !self.encode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.format_ok {
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
        let c = ReportCsv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportCsv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportCsv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportCsv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportCsv::new();
        c.format_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportCsv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
