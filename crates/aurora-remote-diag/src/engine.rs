/// remote diagnostics: scan, read, clear, stream, report
/// Phase 1132

#[derive(Debug, Clone)]
pub struct RemoteDiag {
    pub scan_ok: bool,
    pub read_ok: bool,
    pub clear_ok: bool,
    pub stream_ok: bool,
    pub report_ok: bool,
}

impl Default for RemoteDiag {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteDiag {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            read_ok: true,
            clear_ok: true,
            stream_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.read_ok && self.clear_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
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
        let c = RemoteDiag::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RemoteDiag::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RemoteDiag::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RemoteDiag::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RemoteDiag::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RemoteDiag::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
