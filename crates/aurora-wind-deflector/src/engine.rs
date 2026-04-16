/// Wind deflector: channel, visor, adhesive, fit
/// Phase 761

#[derive(Debug, Clone)]
pub struct WindDeflector {
    pub channel_ok: bool,
    pub visor_ok: bool,
    pub adhesive_ok: bool,
    pub fit_ok: bool,
    pub clarity_ok: bool,
}

impl Default for WindDeflector {
    fn default() -> Self {
        Self::new()
    }
}

impl WindDeflector {
    pub fn new() -> Self {
        Self {
            channel_ok: true,
            visor_ok: true,
            adhesive_ok: true,
            fit_ok: true,
            clarity_ok: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.channel_ok && self.adhesive_ok && self.fit_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.visor_ok && self.clarity_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.visor_ok || !self.adhesive_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.visor_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = WindDeflector::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_condition() {
        let c = WindDeflector::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WindDeflector::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WindDeflector::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_visor() {
        let mut c = WindDeflector::new();
        c.visor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WindDeflector::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
