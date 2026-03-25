/// ml train: fit, validate, evaluate, export, log
/// Phase 1950

#[derive(Debug, Clone)]
pub struct MlTrain {
    pub fit_ok: bool,
    pub validate_ok: bool,
    pub evaluate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for MlTrain {
    fn default() -> Self {
        Self::new()
    }
}

impl MlTrain {
    pub fn new() -> Self {
        Self {
            fit_ok: true,
            validate_ok: true,
            evaluate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fit_ok && self.validate_ok && self.evaluate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fit_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fit_ok {
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
        let c = MlTrain::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlTrain::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlTrain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlTrain::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlTrain::new();
        c.fit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlTrain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
