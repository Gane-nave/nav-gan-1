/// nfc reader: poll, detect, read, write, log
/// Phase 1354

#[derive(Debug, Clone)]
pub struct NfcReader {
    pub poll_ok: bool,
    pub detect_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub log_ok: bool,
}

impl Default for NfcReader {
    fn default() -> Self {
        Self::new()
    }
}

impl NfcReader {
    pub fn new() -> Self {
        Self {
            poll_ok: true,
            detect_ok: true,
            read_ok: true,
            write_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.poll_ok && self.detect_ok && self.read_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.write_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.poll_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.poll_ok {
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
        let c = NfcReader::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NfcReader::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NfcReader::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NfcReader::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NfcReader::new();
        c.poll_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NfcReader::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
