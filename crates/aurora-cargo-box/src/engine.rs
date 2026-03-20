/// Cargo box: aerodynamic, lock, mount, seal, capacity
/// Phase 850

#[derive(Debug, Clone)]
pub struct CargoBox {
    pub aero_ok: bool,
    pub lock_ok: bool,
    pub mount_ok: bool,
    pub seal_ok: bool,
    pub capacity_ok: bool,
}

impl Default for CargoBox {
    fn default() -> Self {
        Self::new()
    }
}

impl CargoBox {
    pub fn new() -> Self {
        Self {
            aero_ok: true,
            lock_ok: true,
            mount_ok: true,
            seal_ok: true,
            capacity_ok: true,
        }
    }

    pub fn security_ok(&self) -> bool {
        self.lock_ok && self.mount_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.aero_ok && self.seal_ok && self.capacity_ok
    }

    pub fn all_ok(&self) -> bool {
        self.security_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.lock_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seal_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        let c = CargoBox::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_protection() {
        let c = CargoBox::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CargoBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CargoBox::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_seal() {
        let mut c = CargoBox::new();
        c.seal_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CargoBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
