/// Data recorder: event, pre-crash, post-crash, tamper, cert
/// Phase 958

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub event_ok: bool,
    pub pre_crash_ok: bool,
    pub post_crash_ok: bool,
    pub tamper_ok: bool,
    pub cert_ok: bool,
}

impl Default for DataRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl DataRecord {
    pub fn new() -> Self {
        Self {
            event_ok: true,
            pre_crash_ok: true,
            post_crash_ok: true,
            tamper_ok: true,
            cert_ok: true,
        }
    }

    pub fn recording_ok(&self) -> bool {
        self.event_ok && self.pre_crash_ok && self.post_crash_ok
    }

    pub fn integrity_ok(&self) -> bool {
        self.tamper_ok && self.cert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recording_ok() && self.integrity_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.tamper_ok || !self.cert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tamper_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording() {
        let c = DataRecord::new();
        assert!(c.recording_ok());
    }

    #[test]
    fn test_integrity() {
        let c = DataRecord::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataRecord::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DataRecord::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_tamper() {
        let mut c = DataRecord::new();
        c.tamper_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DataRecord::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
