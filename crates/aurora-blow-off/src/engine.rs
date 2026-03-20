/// Blow-off valve: piston, spring, recirculation
/// Phase 604

#[derive(Debug, Clone)]
pub struct BlowOffValve {
    pub piston_ok: bool,
    pub spring_ok: bool,
    pub recirc_ok: bool,
    pub seal_ok: bool,
    pub response_ok: bool,
}

impl Default for BlowOffValve {
    fn default() -> Self {
        Self::new()
    }
}

impl BlowOffValve {
    pub fn new() -> Self {
        Self {
            piston_ok: true,
            spring_ok: true,
            recirc_ok: true,
            seal_ok: true,
            response_ok: true,
        }
    }

    pub fn valve_ok(&self) -> bool {
        self.piston_ok && self.spring_ok && self.seal_ok
    }

    pub fn system_ok(&self) -> bool {
        self.valve_ok() && self.recirc_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.response_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.piston_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.piston_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve() {
        let c = BlowOffValve::new();
        assert!(c.valve_ok());
    }

    #[test]
    fn test_system() {
        let c = BlowOffValve::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlowOffValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BlowOffValve::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_piston() {
        let mut c = BlowOffValve::new();
        c.piston_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BlowOffValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
