/// trans congestion2: detect, reduce, recover, adapt, log
/// Phase 2279

#[derive(Debug, Clone)]
pub struct TransCongestion2 {
    pub detect_ok: bool,
    pub reduce_ok: bool,
    pub recover_ok: bool,
    pub adapt_ok: bool,
    pub log_ok: bool,
}

impl Default for TransCongestion2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransCongestion2 {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            reduce_ok: true,
            recover_ok: true,
            adapt_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.reduce_ok && self.recover_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.reduce_ok
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
        let c = TransCongestion2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransCongestion2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransCongestion2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransCongestion2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransCongestion2::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransCongestion2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
