/// Seatbelt pretensioner: pyrotechnic, retractor, buckle
/// Phase 529

#[derive(Debug, Clone)]
pub struct SeatbeltTensioner {
    pub retractor_ok: bool,
    pub buckle_ok: bool,
    pub pretensioner_ok: bool,
    pub webbing_ok: bool,
    pub deployed: bool,
}

impl Default for SeatbeltTensioner {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatbeltTensioner {
    pub fn new() -> Self {
        Self {
            retractor_ok: true,
            buckle_ok: true,
            pretensioner_ok: true,
            webbing_ok: true,
            deployed: false,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.retractor_ok && self.buckle_ok && self.webbing_ok
    }

    pub fn pyro_ok(&self) -> bool {
        self.pretensioner_ok && !self.deployed
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.pyro_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.deployed || !self.pretensioner_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.deployed { return 0.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = SeatbeltTensioner::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_pyro() {
        let c = SeatbeltTensioner::new();
        assert!(c.pyro_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatbeltTensioner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = SeatbeltTensioner::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_deployed() {
        let mut c = SeatbeltTensioner::new();
        c.deployed = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = SeatbeltTensioner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
