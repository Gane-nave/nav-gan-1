/// Transmission ECU: shift logic, torque converter, lockup
/// Phase 710

#[derive(Debug, Clone)]
pub struct TransEcu {
    pub shift_ok: bool,
    pub converter_ok: bool,
    pub lockup_ok: bool,
    pub adaptive_ok: bool,
    pub comm_ok: bool,
}

impl Default for TransEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl TransEcu {
    pub fn new() -> Self {
        Self {
            shift_ok: true,
            converter_ok: true,
            lockup_ok: true,
            adaptive_ok: true,
            comm_ok: true,
        }
    }

    pub fn shifting_ok(&self) -> bool {
        self.shift_ok && self.converter_ok && self.lockup_ok
    }

    pub fn learning_ok(&self) -> bool {
        self.adaptive_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.shifting_ok() && self.learning_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.shift_ok || !self.converter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.shift_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shifting() {
        let c = TransEcu::new();
        assert!(c.shifting_ok());
    }

    #[test]
    fn test_learning() {
        let c = TransEcu::new();
        assert!(c.learning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TransEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_shift() {
        let mut c = TransEcu::new();
        c.shift_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TransEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
