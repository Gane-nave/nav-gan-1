/// Vault client: secret, policy, lease, rotate, transit
/// Phase 1004

#[derive(Debug, Clone)]
pub struct VaultClient {
    pub secret_ok: bool,
    pub policy_ok: bool,
    pub lease_ok: bool,
    pub rotate_ok: bool,
    pub transit_ok: bool,
}

impl Default for VaultClient {
    fn default() -> Self {
        Self::new()
    }
}

impl VaultClient {
    pub fn new() -> Self {
        Self {
            secret_ok: true,
            policy_ok: true,
            lease_ok: true,
            rotate_ok: true,
            transit_ok: true,
        }
    }

    pub fn access_ok(&self) -> bool {
        self.secret_ok && self.policy_ok && self.lease_ok
    }

    pub fn crypto_ok(&self) -> bool {
        self.rotate_ok && self.transit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.access_ok() && self.crypto_ok()
    }

    pub fn needs_renewal(&self) -> bool {
        !self.lease_ok || !self.rotate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.secret_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let c = VaultClient::new();
        assert!(c.access_ok());
    }

    #[test]
    fn test_crypto() {
        let c = VaultClient::new();
        assert!(c.crypto_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VaultClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_renewal() {
        let c = VaultClient::new();
        assert!(!c.needs_renewal());
    }

    #[test]
    fn test_lease() {
        let mut c = VaultClient::new();
        c.lease_ok = false;
        assert!(c.needs_renewal());
    }

    #[test]
    fn test_health() {
        let c = VaultClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
