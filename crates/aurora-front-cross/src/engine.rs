/// front cross: detect, classify, warn, brake, clear
/// Phase 1328

#[derive(Debug, Clone)]
pub struct FrontCross {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub warn_ok: bool,
    pub brake_ok: bool,
    pub clear_ok: bool,
}

impl Default for FrontCross {
    fn default() -> Self {
        Self::new()
    }
}

impl FrontCross {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            warn_ok: true,
            brake_ok: true,
            clear_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.warn_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.brake_ok && self.clear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = FrontCross::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FrontCross::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FrontCross::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FrontCross::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FrontCross::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FrontCross::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
