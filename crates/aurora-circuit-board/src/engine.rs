/// circuit board: power, signal, ground, protect, check
/// Phase 1268

#[derive(Debug, Clone)]
pub struct CircuitBoard {
    pub power_ok: bool,
    pub signal_ok: bool,
    pub ground_ok: bool,
    pub protect_ok: bool,
    pub check_ok: bool,
}

impl Default for CircuitBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl CircuitBoard {
    pub fn new() -> Self {
        Self {
            power_ok: true,
            signal_ok: true,
            ground_ok: true,
            protect_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.power_ok && self.signal_ok && self.ground_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.power_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.power_ok {
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
        let c = CircuitBoard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CircuitBoard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CircuitBoard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CircuitBoard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CircuitBoard::new();
        c.power_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CircuitBoard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
