/// cloud sync: detect, upload, download, merge, log
/// Phase 1446

#[derive(Debug, Clone)]
pub struct CloudSync {
    pub detect_ok: bool,
    pub upload_ok: bool,
    pub download_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudSync {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudSync {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            upload_ok: true,
            download_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.upload_ok && self.download_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.upload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudSync::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudSync::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudSync::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudSync::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudSync::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudSync::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
