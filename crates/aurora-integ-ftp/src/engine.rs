/// integ ftp: connect, upload, download, list, log
/// Phase 1656

#[derive(Debug, Clone)]
pub struct IntegFtp {
    pub connect_ok: bool,
    pub upload_ok: bool,
    pub download_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegFtp {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegFtp {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            upload_ok: true,
            download_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.upload_ok && self.download_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.upload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = IntegFtp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegFtp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegFtp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegFtp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegFtp::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegFtp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
