/// trans circuit2: close, open, halfopen, reset, log
/// Phase 2287

#[derive(Debug, Clone)]
pub struct TransCircuit2 {
    pub close_ok: bool,
    pub open_ok: bool,
    pub halfopen_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for TransCircuit2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransCircuit2 {
    pub fn new() -> Self {
        Self {
            close_ok: true,
            open_ok: true,
            halfopen_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.close_ok && self.open_ok && self.halfopen_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.close_ok || !self.open_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.close_ok {
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
        let c = TransCircuit2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransCircuit2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransCircuit2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransCircuit2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransCircuit2::new();
        c.close_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransCircuit2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
