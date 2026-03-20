/// integ azure2: upload, download, list, delete, log
/// Phase 2248

#[derive(Debug, Clone)]
pub struct IntegAzure2 {
    pub upload_ok: bool,
    pub download_ok: bool,
    pub list_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegAzure2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegAzure2 {
    pub fn new() -> Self {
        Self {
            upload_ok: true,
            download_ok: true,
            list_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.upload_ok && self.download_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
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
        let c = IntegAzure2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegAzure2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegAzure2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegAzure2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegAzure2::new();
        c.upload_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegAzure2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
