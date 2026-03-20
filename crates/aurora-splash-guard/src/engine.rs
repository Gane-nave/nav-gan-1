/// Splash guard: mud flap, bracket, aerodynamic trim
/// Phase 559

#[derive(Debug, Clone)]
pub struct SplashGuard {
    pub flap_ok: bool,
    pub bracket_ok: bool,
    pub trim_ok: bool,
    pub fasteners_ok: bool,
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
            flap_ok: true,
            bracket_ok: true,
            trim_ok: true,
            fasteners_ok: true,
            coverage_ok: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.fasteners_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.flap_ok && self.coverage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.protection_ok() && self.trim_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.flap_ok || !self.bracket_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flap_ok { return 25.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = SplashGuard::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_protection() {
        let c = SplashGuard::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SplashGuard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SplashGuard::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_flap() {
        let mut c = SplashGuard::new();
        c.flap_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SplashGuard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
