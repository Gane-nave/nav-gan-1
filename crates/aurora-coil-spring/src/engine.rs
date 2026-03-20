/// Coil spring: rate, sag, crack, coating
/// Phase 643

#[derive(Debug, Clone)]
pub struct CoilSpring {
    pub rate_ok: bool,
    pub sag_mm: f64,
    pub max_sag_mm: f64,
    pub cracked: bool,
    pub coating_ok: bool,
}

impl Default for CoilSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl CoilSpring {
    pub fn new() -> Self {
        Self {
            rate_ok: true,
            sag_mm: 0.0,
            max_sag_mm: 15.0,
            cracked: false,
            coating_ok: true,
        }
    }

    pub fn height_ok(&self) -> bool {
        self.sag_mm < self.max_sag_mm
    }

    pub fn structural_ok(&self) -> bool {
        !self.cracked && self.coating_ok
    }

    pub fn all_ok(&self) -> bool {
        self.height_ok() && self.structural_ok() && self.rate_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || self.sag_mm > self.max_sag_mm
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        let c = CoilSpring::new();
        assert!(c.height_ok());
    }

    #[test]
    fn test_structural() {
        let c = CoilSpring::new();
        assert!(c.structural_ok());
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
    fn test_crack() {
        let mut c = CoilSpring::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CoilSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
