/// Exhaust manifold: temperature, crack detection, gasket monitoring
/// Phase 319

#[derive(Debug, Clone)]
pub struct ExhaustManifold {
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub cracked: bool,
    pub gasket_ok: bool,
    pub leak_detected: bool,
}

impl Default for ExhaustManifold {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustManifold {
    pub fn new() -> Self {
        Self {
            temp_c: 500.0,
            max_temp_c: 900.0,
            cracked: false,
            gasket_ok: true,
            leak_detected: false,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn integrity_ok(&self) -> bool {
        !self.cracked && self.gasket_ok && !self.leak_detected
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > self.max_temp_c * 0.95
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        if !self.gasket_ok {
            return 30.0;
        }
        if self.leak_detected {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let e = ExhaustManifold::new();
        assert!(e.temp_ok());
    }

    #[test]
    fn test_integrity() {
        let e = ExhaustManifold::new();
        assert!(e.integrity_ok());
    }

    #[test]
    fn test_not_hot() {
        let e = ExhaustManifold::new();
        assert!(!e.overheating());
    }

    #[test]
    fn test_no_replace() {
        let e = ExhaustManifold::new();
        assert!(!e.needs_replacement());
    }

    #[test]
    fn test_cracked() {
        let mut e = ExhaustManifold::new();
        e.cracked = true;
        assert!(e.needs_replacement());
    }

    #[test]
    fn test_health() {
        let e = ExhaustManifold::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
