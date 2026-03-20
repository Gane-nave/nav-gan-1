/// Cloud sync: upload, download, conflict, merge, status
/// Phase 978

#[derive(Debug, Clone)]
pub struct CloudSync {
    pub upload_ok: bool,
    pub download_ok: bool,
    pub conflict_ok: bool,
    pub merge_ok: bool,
    pub status_ok: bool,
}

impl Default for CloudSync {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudSync {
    pub fn new() -> Self {
        Self {
            upload_ok: true,
            download_ok: true,
            conflict_ok: true,
            merge_ok: true,
            status_ok: true,
        }
    }

    pub fn transfer_ok(&self) -> bool {
        self.upload_ok && self.download_ok && self.status_ok
    }

    pub fn resolution_ok(&self) -> bool {
        self.conflict_ok && self.merge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.transfer_ok() && self.resolution_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.upload_ok || !self.download_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.upload_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let c = CloudSync::new();
        assert!(c.transfer_ok());
    }

    #[test]
    fn test_resolution() {
        let c = CloudSync::new();
        assert!(c.resolution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudSync::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = CloudSync::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_upload() {
        let mut c = CloudSync::new();
        c.upload_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = CloudSync::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
