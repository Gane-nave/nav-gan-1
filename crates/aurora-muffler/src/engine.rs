/// Muffler: baffles, packing, shell, resonance
/// Phase 614

#[derive(Debug, Clone)]
pub struct Muffler {
    pub baffles_ok: bool,
    pub packing_ok: bool,
    pub shell_ok: bool,
    pub resonance_ok: bool,
    pub mount_ok: bool,
}

impl Default for Muffler {
    fn default() -> Self {
        Self::new()
    }
}

impl Muffler {
    pub fn new() -> Self {
        Self {
            baffles_ok: true,
            packing_ok: true,
            shell_ok: true,
            resonance_ok: true,
            mount_ok: true,
        }
    }

    pub fn internals_ok(&self) -> bool {
        self.baffles_ok && self.packing_ok
    }

    pub fn structure_ok(&self) -> bool {
        self.shell_ok && self.mount_ok
    }

    pub fn all_ok(&self) -> bool {
        self.internals_ok() && self.structure_ok() && self.resonance_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.baffles_ok || !self.shell_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.shell_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internals() {
        let c = Muffler::new();
        assert!(c.internals_ok());
    }

    #[test]
    fn test_structure() {
        let c = Muffler::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Muffler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Muffler::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_shell() {
        let mut c = Muffler::new();
        c.shell_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Muffler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
