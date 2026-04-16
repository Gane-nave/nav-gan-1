/// Compression test: cylinder, PSI, leak-down, variance
/// Phase 821

#[derive(Debug, Clone)]
pub struct CompressionTest {
    pub cylinder_ok: bool,
    pub psi_ok: bool,
    pub leak_ok: bool,
    pub variance_ok: bool,
    pub spec_ok: bool,
}

impl Default for CompressionTest {
    fn default() -> Self {
        Self::new()
    }
}

impl CompressionTest {
    pub fn new() -> Self {
        Self {
            cylinder_ok: true,
            psi_ok: true,
            leak_ok: true,
            variance_ok: true,
            spec_ok: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.cylinder_ok && self.psi_ok && self.spec_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.leak_ok && self.variance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.sealing_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.psi_ok || !self.leak_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.psi_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = CompressionTest::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_sealing() {
        let c = CompressionTest::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CompressionTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = CompressionTest::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_psi() {
        let mut c = CompressionTest::new();
        c.psi_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = CompressionTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
