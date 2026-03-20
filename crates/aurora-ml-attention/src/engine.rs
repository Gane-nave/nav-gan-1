/// ml attention: compute, mask, multi, cross, log
/// Phase 1963

#[derive(Debug, Clone)]
pub struct MlAttention {
    pub compute_ok: bool,
    pub mask_ok: bool,
    pub multi_ok: bool,
    pub cross_ok: bool,
    pub log_ok: bool,
}

impl Default for MlAttention {
    fn default() -> Self {
        Self::new()
    }
}

impl MlAttention {
    pub fn new() -> Self {
        Self {
            compute_ok: true,
            mask_ok: true,
            multi_ok: true,
            cross_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compute_ok && self.mask_ok && self.multi_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cross_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compute_ok || !self.mask_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compute_ok {
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
        let c = MlAttention::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlAttention::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlAttention::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlAttention::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlAttention::new();
        c.compute_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlAttention::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
