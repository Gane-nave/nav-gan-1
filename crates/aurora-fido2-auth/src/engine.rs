/// FIDO2 auth: register, authenticate, attestation, assert
/// Phase 1003

#[derive(Debug, Clone)]
pub struct Fido2Auth {
    pub register_ok: bool,
    pub auth_ok: bool,
    pub attest_ok: bool,
    pub assert_ok: bool,
    pub device_ok: bool,
}

impl Default for Fido2Auth {
    fn default() -> Self {
        Self::new()
    }
}

impl Fido2Auth {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            auth_ok: true,
            attest_ok: true,
            assert_ok: true,
            device_ok: true,
        }
    }

    pub fn enrollment_ok(&self) -> bool {
        self.register_ok && self.attest_ok && self.device_ok
    }

    pub fn verification_ok(&self) -> bool {
        self.auth_ok && self.assert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.enrollment_ok() && self.verification_ok()
    }

    pub fn needs_setup(&self) -> bool {
        !self.register_ok || !self.device_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enrollment() {
        let c = Fido2Auth::new();
        assert!(c.enrollment_ok());
    }

    #[test]
    fn test_verification() {
        let c = Fido2Auth::new();
        assert!(c.verification_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Fido2Auth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_setup() {
        let c = Fido2Auth::new();
        assert!(!c.needs_setup());
    }

    #[test]
    fn test_register() {
        let mut c = Fido2Auth::new();
        c.register_ok = false;
        assert!(c.needs_setup());
    }

    #[test]
    fn test_health() {
        let c = Fido2Auth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
