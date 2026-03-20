/// Blow-off valve: compressor surge protection, diverter
/// Phase 506

#[derive(Debug, Clone)]
pub struct BlowOffValve {
    pub spring_tension_n: f64,
    pub seal_ok: bool,
    pub diaphragm_ok: bool,
    pub stuck_open: bool,
    pub stuck_closed: bool,
}

impl Default for BlowOffValve {
    fn default() -> Self {
        Self::new()
    }
}

impl BlowOffValve {
    pub fn new() -> Self {
        Self {
            spring_tension_n: 50.0,
            seal_ok: true,
            diaphragm_ok: true,
            stuck_open: false,
            stuck_closed: false,
        }
    }

    pub fn tension_ok(&self) -> bool {
        self.spring_tension_n > 20.0 && self.spring_tension_n < 100.0
    }

    pub fn seals_ok(&self) -> bool {
        self.seal_ok && self.diaphragm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tension_ok() && self.seals_ok() && !self.stuck_open && !self.stuck_closed
    }

    pub fn needs_service(&self) -> bool {
        self.stuck_open || self.stuck_closed
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck_open || self.stuck_closed { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension() {
        let c = BlowOffValve::new();
        assert!(c.tension_ok());
    }

    #[test]
    fn test_seals() {
        let c = BlowOffValve::new();
        assert!(c.seals_ok());
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
    fn test_stuck_open() {
        let mut c = BlowOffValve::new();
        c.stuck_open = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BlowOffValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
