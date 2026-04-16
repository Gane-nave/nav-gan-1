/// aurora-svc-circuit: svc circuit
/// Phase 2570

#[derive(Debug, Clone)]
pub struct SvcCircuit {
    pub open_ok: bool,
    pub close_ok: bool,
    pub halfopen_ok: bool,
    pub report_ok: bool,
    pub reset_ok: bool,
}

impl Default for SvcCircuit {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcCircuit {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            halfopen_ok: true,
            report_ok: true,
            reset_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.halfopen_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.reset_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = SvcCircuit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcCircuit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcCircuit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcCircuit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcCircuit::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcCircuit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcCircuit::default();
        assert!(c.all_ok());
    }
}
