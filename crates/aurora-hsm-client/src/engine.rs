/// HSM client: connect, sign, encrypt, rotate, backup
/// Phase 999

#[derive(Debug, Clone)]
pub struct HsmClient {
    pub connect_ok: bool,
    pub sign_ok: bool,
    pub encrypt_ok: bool,
    pub rotate_ok: bool,
    pub backup_ok: bool,
}

impl Default for HsmClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmClient {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            sign_ok: true,
            encrypt_ok: true,
            rotate_ok: true,
            backup_ok: true,
        }
    }

    pub fn operations_ok(&self) -> bool {
        self.connect_ok && self.sign_ok && self.encrypt_ok
    }

    pub fn management_ok(&self) -> bool {
        self.rotate_ok && self.backup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.operations_ok() && self.management_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.connect_ok || !self.sign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let c = HsmClient::new();
        assert!(c.operations_ok());
    }

    #[test]
    fn test_management() {
        let c = HsmClient::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HsmClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = HsmClient::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_connect() {
        let mut c = HsmClient::new();
        c.connect_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = HsmClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
