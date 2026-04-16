/// data mask2: detect, mask, tokenize, audit, log
/// Phase 2212

#[derive(Debug, Clone)]
pub struct DataMask2 {
    pub detect_ok: bool,
    pub mask_ok: bool,
    pub tokenize_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for DataMask2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataMask2 {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            mask_ok: true,
            tokenize_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.mask_ok && self.tokenize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.mask_ok
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
        let c = DataMask2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataMask2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataMask2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataMask2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataMask2::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataMask2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
