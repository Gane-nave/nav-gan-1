/// Secure enclave: TEE, attestation, seal, key, crypto
/// Phase 998

#[derive(Debug, Clone)]
pub struct SecureEnclave {
    pub tee_ok: bool,
    pub attest_ok: bool,
    pub seal_ok: bool,
    pub key_ok: bool,
    pub crypto_ok: bool,
}

impl Default for SecureEnclave {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureEnclave {
    pub fn new() -> Self {
        Self {
            tee_ok: true,
            attest_ok: true,
            seal_ok: true,
            key_ok: true,
            crypto_ok: true,
        }
    }

    pub fn hardware_ok(&self) -> bool {
        self.tee_ok && self.attest_ok && self.seal_ok
    }

    pub fn crypto_ops_ok(&self) -> bool {
        self.key_ok && self.crypto_ok
    }

    pub fn all_ok(&self) -> bool {
        self.hardware_ok() && self.crypto_ops_ok()
    }

    pub fn needs_provision(&self) -> bool {
        !self.tee_ok || !self.key_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tee_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware() {
        let c = SecureEnclave::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_crypto_ops() {
        let c = SecureEnclave::new();
        assert!(c.crypto_ops_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecureEnclave::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_provision() {
        let c = SecureEnclave::new();
        assert!(!c.needs_provision());
    }

    #[test]
    fn test_tee() {
        let mut c = SecureEnclave::new();
        c.tee_ok = false;
        assert!(c.needs_provision());
    }

    #[test]
    fn test_health() {
        let c = SecureEnclave::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
