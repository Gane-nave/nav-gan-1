/// PKI manager: CA, cert, CRL, OCSP, renewal
/// Phase 1000

#[derive(Debug, Clone)]
pub struct PkiManager {
    pub ca_ok: bool,
    pub cert_ok: bool,
    pub crl_ok: bool,
    pub ocsp_ok: bool,
    pub renewal_ok: bool,
}

impl Default for PkiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PkiManager {
    pub fn new() -> Self {
        Self {
            ca_ok: true,
            cert_ok: true,
            crl_ok: true,
            ocsp_ok: true,
            renewal_ok: true,
        }
    }

    pub fn issuance_ok(&self) -> bool {
        self.ca_ok && self.cert_ok && self.renewal_ok
    }

    pub fn revocation_ok(&self) -> bool {
        self.crl_ok && self.ocsp_ok
    }

    pub fn all_ok(&self) -> bool {
        self.issuance_ok() && self.revocation_ok()
    }

    pub fn needs_renewal(&self) -> bool {
        !self.cert_ok || !self.renewal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ca_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issuance() {
        let c = PkiManager::new();
        assert!(c.issuance_ok());
    }

    #[test]
    fn test_revocation() {
        let c = PkiManager::new();
        assert!(c.revocation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PkiManager::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_renewal() {
        let c = PkiManager::new();
        assert!(!c.needs_renewal());
    }

    #[test]
    fn test_cert() {
        let mut c = PkiManager::new();
        c.cert_ok = false;
        assert!(c.needs_renewal());
    }

    #[test]
    fn test_health() {
        let c = PkiManager::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
