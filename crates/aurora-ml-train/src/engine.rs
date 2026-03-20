/// ml train: prepare, train, validate, export, log
/// Phase 1465

#[derive(Debug, Clone)]
pub struct MlTrain {
    pub prepare_ok: bool,
    pub train_ok: bool,
    pub validate_ok: bool,
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
            prepare_ok: true,
            train_ok: true,
            validate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.prepare_ok && self.train_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.prepare_ok || !self.train_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.prepare_ok {
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
        c.prepare_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlTrain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
