/// Coil spring: suspension spring rate, sag, corrosion
/// Phase 474

#[derive(Debug, Clone)]
pub struct CoilSpring {
    pub spring_rate_nmm: f64,
    pub free_length_mm: f64,
    pub compressed_mm: f64,
    pub sag_ok: bool,
    pub corrosion_free: bool,
}

impl Default for CoilSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl CoilSpring {
    pub fn new() -> Self {
        Self {
            spring_rate_nmm: 35.0,
            free_length_mm: 350.0,
            compressed_mm: 200.0,
            sag_ok: true,
            corrosion_free: true,
        }
    }

    pub fn travel_mm(&self) -> f64 {
        self.free_length_mm - self.compressed_mm
    }

    pub fn not_sagged(&self) -> bool {
        self.sag_ok && self.corrosion_free
    }

    pub fn all_ok(&self) -> bool {
        self.sag_ok && self.corrosion_free
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sag_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sag_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel() {
        let c = CoilSpring::new();
        assert!(c.travel_mm() > 100.0);
    }

    #[test]
    fn test_not_sagged() {
        let c = CoilSpring::new();
        assert!(c.not_sagged());
    }

    #[test]
    fn test_all_ok() {
        let c = CoilSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CoilSpring::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sagged() {
        let mut c = CoilSpring::new();
        c.sag_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CoilSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
