/// ml label: annotate, review, export, quality, log
/// Phase 1959

#[derive(Debug, Clone)]
pub struct MlLabel {
    pub annotate_ok: bool,
    pub review_ok: bool,
    pub export_ok: bool,
    pub quality_ok: bool,
    pub log_ok: bool,
}

impl Default for MlLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl MlLabel {
    pub fn new() -> Self {
        Self {
            annotate_ok: true,
            review_ok: true,
            export_ok: true,
            quality_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.annotate_ok && self.review_ok && self.export_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.quality_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.annotate_ok || !self.review_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.annotate_ok {
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
        let c = MlLabel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlLabel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlLabel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlLabel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlLabel::new();
        c.annotate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlLabel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
