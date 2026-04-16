/// rfid reader: scan, detect, read, identify, log
/// Phase 1355

#[derive(Debug, Clone)]
pub struct RfidReader {
    pub scan_ok: bool,
    pub detect_ok: bool,
    pub read_ok: bool,
    pub identify_ok: bool,
    pub log_ok: bool,
}

impl Default for RfidReader {
    fn default() -> Self {
        Self::new()
    }
}

impl RfidReader {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            detect_ok: true,
            read_ok: true,
            identify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.detect_ok && self.read_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.identify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.detect_ok
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
        let c = RfidReader::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RfidReader::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RfidReader::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RfidReader::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RfidReader::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RfidReader::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
