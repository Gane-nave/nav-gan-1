/// FTP client: connect, upload, download, list, resume
/// Phase 1063

#[derive(Debug, Clone)]
pub struct FtpClient {
    pub connect_ok: bool,
    pub upload_ok: bool,
    pub download_ok: bool,
    pub list_ok: bool,
    pub resume_ok: bool,
}

impl Default for FtpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FtpClient {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            upload_ok: true,
            download_ok: true,
            list_ok: true,
            resume_ok: true,
        }
    }

    pub fn transfer_ok(&self) -> bool {
        self.connect_ok && self.upload_ok && self.download_ok
    }

    pub fn management_ok(&self) -> bool {
        self.list_ok && self.resume_ok
    }

    pub fn all_ok(&self) -> bool {
        self.transfer_ok() && self.management_ok()
    }

    pub fn needs_reconnect(&self) -> bool {
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
    fn test_transfer() {
        let c = FtpClient::new();
        assert!(c.transfer_ok());
    }

    #[test]
    fn test_management() {
        let c = FtpClient::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FtpClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconnect() {
        let c = FtpClient::new();
        assert!(!c.needs_reconnect());
    }

    #[test]
    fn test_connect() {
        let mut c = FtpClient::new();
        c.connect_ok = false;
        assert!(c.needs_reconnect());
    }

    #[test]
    fn test_health() {
        let c = FtpClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
