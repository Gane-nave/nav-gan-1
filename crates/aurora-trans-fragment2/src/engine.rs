/// trans fragment2: split, reassemble, order, verify, log
/// Phase 2280

#[derive(Debug, Clone)]
pub struct TransFragment2 {
    pub split_ok: bool,
    pub reassemble_ok: bool,
    pub order_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for TransFragment2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransFragment2 {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            reassemble_ok: true,
            order_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.split_ok && self.reassemble_ok && self.order_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.split_ok || !self.reassemble_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
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
        let c = TransFragment2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransFragment2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransFragment2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransFragment2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransFragment2::new();
        c.split_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransFragment2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
