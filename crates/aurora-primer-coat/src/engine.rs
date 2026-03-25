/// Primer coat: adhesion promoter, filler, anti-chip, UV protection
/// Phase 388

#[derive(Debug, Clone)]
pub struct PrimerCoat {
    pub thickness_um: f64,
    pub min_thickness_um: f64,
    pub adhesion_ok: bool,
    pub sanded: bool,
    pub cured: bool,
}

impl Default for PrimerCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimerCoat {
    pub fn new() -> Self {
        Self {
            thickness_um: 35.0,
            min_thickness_um: 20.0,
            adhesion_ok: true,
            sanded: true,
            cured: true,
        }
    }

    pub fn thickness_ok(&self) -> bool {
        self.thickness_um >= self.min_thickness_um
    }

    pub fn ready_for_paint(&self) -> bool {
        self.sanded && self.cured && self.adhesion_ok
    }

    pub fn effective(&self) -> bool {
        self.thickness_ok() && self.adhesion_ok
    }

    pub fn needs_respray(&self) -> bool {
        !self.adhesion_ok || !self.thickness_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhesion_ok {
            return 0.0;
        }
        if !self.cured {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thickness() {
        let p = PrimerCoat::new();
        assert!(p.thickness_ok());
    }

    #[test]
    fn test_ready() {
        let p = PrimerCoat::new();
        assert!(p.ready_for_paint());
    }

    #[test]
    fn test_effective() {
        let p = PrimerCoat::new();
        assert!(p.effective());
    }

    #[test]
    fn test_no_respray() {
        let p = PrimerCoat::new();
        assert!(!p.needs_respray());
    }

    #[test]
    fn test_bad_adhesion() {
        let mut p = PrimerCoat::new();
        p.adhesion_ok = false;
        assert!(p.needs_respray());
    }

    #[test]
    fn test_health() {
        let p = PrimerCoat::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
