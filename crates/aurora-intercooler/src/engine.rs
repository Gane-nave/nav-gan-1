/// Intercooler: efficiency, pressure drop, leak test
/// Phase 611

#[derive(Debug, Clone)]
pub struct Intercooler {
    pub efficiency_pct: f64,
    pub pressure_drop_ok: bool,
    pub leak_free: bool,
    pub fins_ok: bool,
    pub piping_ok: bool,
}

impl Default for Intercooler {
    fn default() -> Self {
        Self::new()
    }
}

impl Intercooler {
    pub fn new() -> Self {
        Self {
            efficiency_pct: 85.0,
            pressure_drop_ok: true,
            leak_free: true,
            fins_ok: true,
            piping_ok: true,
        }
    }

    pub fn cooling_ok(&self) -> bool {
        self.efficiency_pct > 60.0 && self.fins_ok
    }

    pub fn integrity_ok(&self) -> bool {
        self.leak_free && self.pressure_drop_ok && self.piping_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cooling_ok() && self.integrity_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.leak_free || !self.fins_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling() {
        let c = Intercooler::new();
        assert!(c.cooling_ok());
    }

    #[test]
    fn test_integrity() {
        let c = Intercooler::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Intercooler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Intercooler::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = Intercooler::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Intercooler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
