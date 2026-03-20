/// Paint protection film: clarity, self-heal, thickness
/// Phase 770

#[derive(Debug, Clone)]
pub struct Ppf {
    pub clarity_ok: bool,
    pub self_heal_ok: bool,
    pub thickness_ok: bool,
    pub adhesion_ok: bool,
    pub edge_ok: bool,
}

impl Default for Ppf {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppf {
    pub fn new() -> Self {
        Self {
            clarity_ok: true,
            self_heal_ok: true,
            thickness_ok: true,
            adhesion_ok: true,
            edge_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.thickness_ok && self.self_heal_ok
    }

    pub fn appearance_ok(&self) -> bool {
        self.clarity_ok && self.adhesion_ok && self.edge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.appearance_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.clarity_ok || !self.adhesion_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clarity_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = Ppf::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_appearance() {
        let c = Ppf::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Ppf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Ppf::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_clarity() {
        let mut c = Ppf::new();
        c.clarity_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Ppf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
