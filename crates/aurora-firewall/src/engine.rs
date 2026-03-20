/// Firewall: bulkhead, grommet, insulation, seal
/// Phase 794

#[derive(Debug, Clone)]
pub struct Firewall {
    pub bulkhead_ok: bool,
    pub grommet_ok: bool,
    pub insulation_ok: bool,
    pub seal_ok: bool,
    pub coating_ok: bool,
}

impl Default for Firewall {
    fn default() -> Self {
        Self::new()
    }
}

impl Firewall {
    pub fn new() -> Self {
        Self {
            bulkhead_ok: true,
            grommet_ok: true,
            insulation_ok: true,
            seal_ok: true,
            coating_ok: true,
        }
    }

    pub fn barrier_ok(&self) -> bool {
        self.bulkhead_ok && self.insulation_ok
    }

    pub fn penetration_ok(&self) -> bool {
        self.grommet_ok && self.seal_ok && self.coating_ok
    }

    pub fn all_ok(&self) -> bool {
        self.barrier_ok() && self.penetration_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.bulkhead_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bulkhead_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier() {
        let c = Firewall::new();
        assert!(c.barrier_ok());
    }

    #[test]
    fn test_penetration() {
        let c = Firewall::new();
        assert!(c.penetration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Firewall::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = Firewall::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_bulkhead() {
        let mut c = Firewall::new();
        c.bulkhead_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = Firewall::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
