/// torque conv: multiply, lock, slip, stall, report
/// Phase 1216

#[derive(Debug, Clone)]
pub struct TorqueConv {
    pub multiply_ok: bool,
    pub lock_ok: bool,
    pub slip_ok: bool,
    pub stall_ok: bool,
    pub report_ok: bool,
}

impl Default for TorqueConv {
    fn default() -> Self {
        Self::new()
    }
}

impl TorqueConv {
    pub fn new() -> Self {
        Self {
            multiply_ok: true,
            lock_ok: true,
            slip_ok: true,
            stall_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.multiply_ok && self.lock_ok && self.slip_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stall_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.multiply_ok || !self.lock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.multiply_ok {
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
        let c = TorqueConv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TorqueConv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TorqueConv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TorqueConv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TorqueConv::new();
        c.multiply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TorqueConv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
