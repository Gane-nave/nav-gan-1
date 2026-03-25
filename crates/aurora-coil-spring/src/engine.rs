/// coil spring: compress, extend, rate, preload, check
/// Phase 1195

#[derive(Debug, Clone)]
pub struct CoilSpring {
    pub compress_ok: bool,
    pub extend_ok: bool,
    pub rate_ok: bool,
    pub preload_ok: bool,
    pub check_ok: bool,
}

impl Default for CoilSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl CoilSpring {
    pub fn new() -> Self {
        Self {
            compress_ok: true,
            extend_ok: true,
            rate_ok: true,
            preload_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compress_ok && self.extend_ok && self.rate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.preload_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compress_ok || !self.extend_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compress_ok {
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
        let c = CoilSpring::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CoilSpring::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CoilSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CoilSpring::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CoilSpring::new();
        c.compress_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CoilSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
