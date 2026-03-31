/// aurora-cloud-storage: cloud storage
/// Phase 2544

#[derive(Debug, Clone)]
pub struct CloudStorage {
    pub upload_ok: bool,
    pub download_ok: bool,
    pub delete_ok: bool,
    pub list_ok: bool,
    pub presign_ok: bool,
}

impl Default for CloudStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudStorage {
    pub fn new() -> Self {
        Self {
            upload_ok: true,
            download_ok: true,
            delete_ok: true,
            list_ok: true,
            presign_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.upload_ok && self.download_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.presign_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.upload_ok || !self.download_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.upload_ok {
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
        let c = CloudStorage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudStorage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudStorage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudStorage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudStorage::new();
        c.upload_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudStorage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CloudStorage::default();
        assert!(c.all_ok());
    }
}
