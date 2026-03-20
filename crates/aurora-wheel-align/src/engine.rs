/// Wheel alignment: camber, caster, toe angles, alignment monitoring
/// Phase 169

#[derive(Debug, Clone)]
pub struct WheelAlignment {
    pub camber_deg: f64,
    pub caster_deg: f64,
    pub toe_deg: f64,
    pub target_camber: f64,
    pub target_caster: f64,
    pub target_toe: f64,
}

impl Default for WheelAlignment {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelAlignment {
    pub fn new() -> Self {
        Self {
            camber_deg: -0.5,
            caster_deg: 3.0,
            toe_deg: 0.1,
            target_camber: -0.5,
            target_caster: 3.0,
            target_toe: 0.1,
        }
    }

    pub fn camber_error(&self) -> f64 {
        (self.camber_deg - self.target_camber).abs()
    }

    pub fn caster_error(&self) -> f64 {
        (self.caster_deg - self.target_caster).abs()
    }

    pub fn toe_error(&self) -> f64 {
        (self.toe_deg - self.target_toe).abs()
    }

    pub fn needs_alignment(&self) -> bool {
        self.camber_error() > 0.5 || self.caster_error() > 0.5 || self.toe_error() > 0.3
    }

    pub fn tire_wear_factor(&self) -> f64 {
        1.0 + self.camber_error() * 0.1 + self.toe_error() * 0.2
    }

    pub fn alignment_score(&self) -> f64 {
        let camber_s = (1.0 - self.camber_error() / 2.0).clamp(0.0, 1.0) * 35.0;
        let caster_s = (1.0 - self.caster_error() / 2.0).clamp(0.0, 1.0) * 30.0;
        let toe_s = (1.0 - self.toe_error() / 1.0).clamp(0.0, 1.0) * 35.0;
        camber_s + caster_s + toe_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aligned() {
        let w = WheelAlignment::new();
        assert!(!w.needs_alignment());
    }

    #[test]
    fn test_needs_alignment() {
        let mut w = WheelAlignment::new();
        w.camber_deg = 1.5;
        assert!(w.needs_alignment());
    }

    #[test]
    fn test_camber_error() {
        let w = WheelAlignment::new();
        assert!(w.camber_error() < 0.01);
    }

    #[test]
    fn test_tire_wear() {
        let w = WheelAlignment::new();
        assert!((w.tire_wear_factor() - 1.0).abs() < 0.05);
    }

    #[test]
    fn test_score_perfect() {
        let w = WheelAlignment::new();
        assert!(w.alignment_score() > 95.0);
    }

    #[test]
    fn test_score_misaligned() {
        let mut w = WheelAlignment::new();
        w.camber_deg = 2.0;
        w.toe_deg = 0.8;
        assert!(w.alignment_score() < 70.0);
    }
}
