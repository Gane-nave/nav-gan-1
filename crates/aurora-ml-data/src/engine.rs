/// ml data: collect, clean, label, augment, log
/// Phase 1466

#[derive(Debug, Clone)]
pub struct MlData {
    pub collect_ok: bool,
    pub clean_ok: bool,
    pub label_ok: bool,
    pub augment_ok: bool,
    pub log_ok: bool,
}

impl Default for MlData {
    fn default() -> Self {
        Self::new()
    }
}

impl MlData {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            clean_ok: true,
            label_ok: true,
            augment_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.clean_ok && self.label_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.augment_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.clean_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = MlData::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlData::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlData::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlData::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlData::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlData::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
