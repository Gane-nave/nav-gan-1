/// trans backoff2: calculate, apply, reset, jitter, log
/// Phase 2286

#[derive(Debug, Clone)]
pub struct TransBackoff2 {
    pub calculate_ok: bool,
    pub apply_ok: bool,
    pub reset_ok: bool,
    pub jitter_ok: bool,
    pub log_ok: bool,
}

impl Default for TransBackoff2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransBackoff2 {
    pub fn new() -> Self {
        Self {
            calculate_ok: true,
            apply_ok: true,
            reset_ok: true,
            jitter_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.calculate_ok && self.apply_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.jitter_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.calculate_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.calculate_ok {
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
        let c = TransBackoff2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransBackoff2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransBackoff2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransBackoff2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransBackoff2::new();
        c.calculate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransBackoff2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
