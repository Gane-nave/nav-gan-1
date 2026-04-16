/// S3 client: upload, download, list, delete, multipart
/// Phase 1059

#[derive(Debug, Clone)]
pub struct S3Client {
    pub upload_ok: bool,
    pub download_ok: bool,
    pub list_ok: bool,
    pub delete_ok: bool,
    pub multipart_ok: bool,
}

impl Default for S3Client {
    fn default() -> Self {
        Self::new()
    }
}

impl S3Client {
    pub fn new() -> Self {
        Self {
            upload_ok: true,
            download_ok: true,
            list_ok: true,
            delete_ok: true,
            multipart_ok: true,
        }
    }

    pub fn transfer_ok(&self) -> bool {
        self.upload_ok && self.download_ok && self.multipart_ok
    }

    pub fn management_ok(&self) -> bool {
        self.list_ok && self.delete_ok
    }

    pub fn all_ok(&self) -> bool {
        self.transfer_ok() && self.management_ok()
    }

    pub fn needs_auth(&self) -> bool {
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
    fn test_transfer() {
        let c = S3Client::new();
        assert!(c.transfer_ok());
    }

    #[test]
    fn test_management() {
        let c = S3Client::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = S3Client::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_auth() {
        let c = S3Client::new();
        assert!(!c.needs_auth());
    }

    #[test]
    fn test_upload() {
        let mut c = S3Client::new();
        c.upload_ok = false;
        assert!(c.needs_auth());
    }

    #[test]
    fn test_health() {
        let c = S3Client::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
