/// ml augment: transform, generate, validate, mix, log
/// Phase 1960

#[derive(Debug, Clone)]
pub struct MlAugment {
    pub transform_ok: bool,
    pub generate_ok: bool,
    pub validate_ok: bool,
    pub mix_ok: bool,
    pub log_ok: bool,
}

impl Default for MlAugment {
    fn default() -> Self {
        Self::new()
    }
}

impl MlAugment {
    pub fn new() -> Self {
        Self {
            transform_ok: true,
            generate_ok: true,
            validate_ok: true,
            mix_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.transform_ok && self.generate_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.mix_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.transform_ok || !self.generate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transform_ok {
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
        let c = MlAugment::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlAugment::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlAugment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlAugment::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlAugment::new();
        c.transform_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlAugment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
