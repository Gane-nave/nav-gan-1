/// aurora-anim-slide: anim slide
/// Phase 2471

#[derive(Debug, Clone)]
pub struct AnimSlide {
    pub in_ok: bool,
    pub out_ok: bool,
    pub direction_ok: bool,
    pub duration_ok: bool,
    pub easing_ok: bool,
}

impl Default for AnimSlide {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimSlide {
    pub fn new() -> Self {
        Self {
            in_ok: true,
            out_ok: true,
            direction_ok: true,
            duration_ok: true,
            easing_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.in_ok && self.out_ok && self.direction_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.duration_ok && self.easing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.in_ok || !self.out_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.in_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AnimSlide::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnimSlide::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnimSlide::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnimSlide::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnimSlide::new();
        c.in_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnimSlide::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AnimSlide::default();
        assert!(c.all_ok());
    }
}
