/// Piston ring: compression, oil scraper, blow-by monitoring
/// Phase 314

#[derive(Debug, Clone)]
pub struct PistonRing {
    pub compression_psi: f64,
    pub min_compression_psi: f64,
    pub blow_by_lpm: f64,
    pub oil_consumption_ml_per_1000km: f64,
    pub ring_gap_ok: bool,
}

impl Default for PistonRing {
    fn default() -> Self {
        Self::new()
    }
}

impl PistonRing {
    pub fn new() -> Self {
        Self {
            compression_psi: 180.0,
            min_compression_psi: 120.0,
            blow_by_lpm: 5.0,
            oil_consumption_ml_per_1000km: 50.0,
            ring_gap_ok: true,
        }
    }

    pub fn compression_ok(&self) -> bool {
        self.compression_psi >= self.min_compression_psi
    }

    pub fn blow_by_ok(&self) -> bool {
        self.blow_by_lpm < 15.0
    }

    pub fn oil_ok(&self) -> bool {
        self.oil_consumption_ml_per_1000km < 200.0
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.compression_ok() || !self.ring_gap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compression_ok() {
            return 0.0;
        }
        if !self.blow_by_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression() {
        let p = PistonRing::new();
        assert!(p.compression_ok());
    }

    #[test]
    fn test_blow_by() {
        let p = PistonRing::new();
        assert!(p.blow_by_ok());
    }

    #[test]
    fn test_oil_ok() {
        let p = PistonRing::new();
        assert!(p.oil_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let p = PistonRing::new();
        assert!(!p.needs_rebuild());
    }

    #[test]
    fn test_low_compression() {
        let mut p = PistonRing::new();
        p.compression_psi = 90.0;
        assert!(p.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let p = PistonRing::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
