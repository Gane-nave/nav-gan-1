/// Emission test: HC, CO, NOx, PM, lambda
/// Phase 820

#[derive(Debug, Clone)]
pub struct EmissionTest {
    pub hc_ok: bool,
    pub co_ok: bool,
    pub nox_ok: bool,
    pub pm_ok: bool,
    pub lambda_ok: bool,
}

impl Default for EmissionTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EmissionTest {
    pub fn new() -> Self {
        Self {
            hc_ok: true,
            co_ok: true,
            nox_ok: true,
            pm_ok: true,
            lambda_ok: true,
        }
    }

    pub fn gas_ok(&self) -> bool {
        self.hc_ok && self.co_ok && self.nox_ok
    }

    pub fn particulate_ok(&self) -> bool {
        self.pm_ok && self.lambda_ok
    }

    pub fn all_ok(&self) -> bool {
        self.gas_ok() && self.particulate_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.hc_ok || !self.nox_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.nox_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas() {
        let c = EmissionTest::new();
        assert!(c.gas_ok());
    }

    #[test]
    fn test_particulate() {
        let c = EmissionTest::new();
        assert!(c.particulate_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EmissionTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = EmissionTest::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_nox() {
        let mut c = EmissionTest::new();
        c.nox_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = EmissionTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
