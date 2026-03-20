/// cloud backup: snapshot, compress, upload, verify, log
/// Phase 1447

#[derive(Debug, Clone)]
pub struct CloudBackup {
    pub snapshot_ok: bool,
    pub compress_ok: bool,
    pub upload_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudBackup {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudBackup {
    pub fn new() -> Self {
        Self {
            snapshot_ok: true,
            compress_ok: true,
            upload_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.snapshot_ok && self.compress_ok && self.upload_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.snapshot_ok || !self.compress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.snapshot_ok {
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
        let c = CloudBackup::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudBackup::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudBackup::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudBackup::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudBackup::new();
        c.snapshot_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudBackup::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
