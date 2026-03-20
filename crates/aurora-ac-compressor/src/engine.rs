/// AC compressor: clutch, refrigerant, pressure
/// Phase 622

#[derive(Debug, Clone)]
pub struct AcCompressor {
    pub clutch_ok: bool,
    pub refrigerant_ok: bool,
    pub high_pressure_ok: bool,
    pub low_pressure_ok: bool,
    pub oil_ok: bool,
}

impl Default for AcCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl AcCompressor {
    pub fn new() -> Self {
        Self {
            clutch_ok: true,
            refrigerant_ok: true,
            high_pressure_ok: true,
            low_pressure_ok: true,
            oil_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.clutch_ok && self.oil_ok
    }

    pub fn refrigerant_good(&self) -> bool {
        self.refrigerant_ok && self.high_pressure_ok && self.low_pressure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.refrigerant_good()
    }

    pub fn needs_service(&self) -> bool {
        !self.clutch_ok || !self.refrigerant_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clutch_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = AcCompressor::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_refrigerant() {
        let c = AcCompressor::new();
        assert!(c.refrigerant_good());
    }

    #[test]
    fn test_all_ok() {
        let c = AcCompressor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AcCompressor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_clutch() {
        let mut c = AcCompressor::new();
        c.clutch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AcCompressor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
