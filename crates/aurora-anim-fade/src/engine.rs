/// aurora-anim-fade: anim fade
/// Phase 2470

#[derive(Debug, Clone)]
pub struct AnimFade {
    pub in_ok: bool,
    pub out_ok: bool,
    pub duration_ok: bool,
    pub delay_ok: bool,
    pub easing_ok: bool,
}

impl Default for AnimFade {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimFade {
    pub fn new() -> Self {
        Self {
            in_ok: true,
            out_ok: true,
            duration_ok: true,
            delay_ok: true,
            easing_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.in_ok && self.out_ok && self.duration_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delay_ok && self.easing_ok
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
        let c = AnimFade::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnimFade::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnimFade::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnimFade::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnimFade::new();
        c.in_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnimFade::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AnimFade::default();
        assert!(c.all_ok());
    }
}
