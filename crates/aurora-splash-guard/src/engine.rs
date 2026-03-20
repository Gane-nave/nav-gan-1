/// Splash guard: front, rear, mounting, material
/// Phase 760

#[derive(Debug, Clone)]
pub struct SplashGuard {
    pub front_ok: bool,
    pub rear_ok: bool,
    pub mounting_ok: bool,
    pub material_ok: bool,
    pub coverage_ok: bool,
}

impl Default for SplashGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl SplashGuard {
    pub fn new() -> Self {
        Self {
            front_ok: true,
            rear_ok: true,
            mounting_ok: true,
            material_ok: true,
            coverage_ok: true,
        }
    }

    pub fn guards_ok(&self) -> bool {
        self.front_ok && self.rear_ok && self.coverage_ok
    }

    pub fn install_ok(&self) -> bool {
        self.mounting_ok && self.material_ok
    }

    pub fn all_ok(&self) -> bool {
        self.guards_ok() && self.install_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.front_ok || !self.rear_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.front_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guards() {
        let c = SplashGuard::new();
        assert!(c.guards_ok());
    }

    #[test]
    fn test_install() {
        let c = SplashGuard::new();
        assert!(c.install_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SplashGuard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = SplashGuard::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_front() {
        let mut c = SplashGuard::new();
        c.front_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = SplashGuard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
