/// Tire cavity resonance: Helmholtz frequency, foam insert, damping
/// Phase 368

#[derive(Debug, Clone)]
pub struct TireCavity {
    pub resonance_hz: f64,
    pub amplitude_db: f64,
    pub foam_insert: bool,
    pub damped: bool,
}

impl Default for TireCavity {
    fn default() -> Self {
        Self::new()
    }
}

impl TireCavity {
    pub fn new() -> Self {
        Self {
            resonance_hz: 220.0,
            amplitude_db: 15.0,
            foam_insert: false,
            damped: true,
        }
    }

    pub fn in_audible_range(&self) -> bool {
        self.resonance_hz > 20.0 && self.resonance_hz < 20000.0
    }

    pub fn annoying(&self) -> bool {
        self.amplitude_db > 25.0 && self.in_audible_range()
    }

    pub fn mitigated(&self) -> bool {
        self.foam_insert || self.damped
    }

    pub fn needs_treatment(&self) -> bool {
        self.annoying() && !self.mitigated()
    }

    pub fn health_score(&self) -> f64 {
        if self.annoying() && !self.mitigated() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audible() {
        let t = TireCavity::new();
        assert!(t.in_audible_range());
    }

    #[test]
    fn test_not_annoying() {
        let t = TireCavity::new();
        assert!(!t.annoying());
    }

    #[test]
    fn test_mitigated() {
        let t = TireCavity::new();
        assert!(t.mitigated());
    }

    #[test]
    fn test_no_treatment() {
        let t = TireCavity::new();
        assert!(!t.needs_treatment());
    }

    #[test]
    fn test_annoying() {
        let mut t = TireCavity::new();
        t.amplitude_db = 30.0;
        assert!(t.annoying());
    }

    #[test]
    fn test_health() {
        let t = TireCavity::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
