/// eba sys: detect, warn, assist, brake, release
/// Phase 1161

#[derive(Debug, Clone)]
pub struct EbaSys {
    pub detect_ok: bool,
    pub warn_ok: bool,
    pub assist_ok: bool,
    pub brake_ok: bool,
    pub release_ok: bool,
}

impl Default for EbaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl EbaSys {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            warn_ok: true,
            assist_ok: true,
            brake_ok: true,
            release_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.warn_ok && self.assist_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.brake_ok && self.release_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.warn_ok
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
        let c = EbaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EbaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EbaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EbaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EbaSys::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EbaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
