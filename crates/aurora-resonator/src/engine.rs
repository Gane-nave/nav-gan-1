/// Exhaust resonator: tuning, chamber, pipe connection
/// Phase 615

#[derive(Debug, Clone)]
pub struct Resonator {
    pub tuning_ok: bool,
    pub chamber_ok: bool,
    pub pipe_ok: bool,
    pub seal_ok: bool,
    pub mount_ok: bool,
}

impl Default for Resonator {
    fn default() -> Self {
        Self::new()
    }
}

impl Resonator {
    pub fn new() -> Self {
        Self {
            tuning_ok: true,
            chamber_ok: true,
            pipe_ok: true,
            seal_ok: true,
            mount_ok: true,
        }
    }

    pub fn acoustics_ok(&self) -> bool {
        self.tuning_ok && self.chamber_ok
    }

    pub fn connection_ok(&self) -> bool {
        self.pipe_ok && self.seal_ok && self.mount_ok
    }

    pub fn all_ok(&self) -> bool {
        self.acoustics_ok() && self.connection_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.chamber_ok || !self.pipe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.chamber_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acoustics() {
        let c = Resonator::new();
        assert!(c.acoustics_ok());
    }

    #[test]
    fn test_connection() {
        let c = Resonator::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Resonator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Resonator::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_chamber() {
        let mut c = Resonator::new();
        c.chamber_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Resonator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
