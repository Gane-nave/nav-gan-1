/// Primer: adhesion, corrosion protection, filler, sand
/// Phase 764

#[derive(Debug, Clone)]
pub struct Primer {
    pub adhesion_ok: bool,
    pub corrosion_ok: bool,
    pub filler_ok: bool,
    pub sanded_ok: bool,
    pub cured: bool,
}

impl Default for Primer {
    fn default() -> Self {
        Self::new()
    }
}

impl Primer {
    pub fn new() -> Self {
        Self {
            adhesion_ok: true,
            corrosion_ok: true,
            filler_ok: true,
            sanded_ok: true,
            cured: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.adhesion_ok && self.corrosion_ok
    }

    pub fn preparation_ok(&self) -> bool {
        self.filler_ok && self.sanded_ok && self.cured
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.preparation_ok()
    }

    pub fn needs_reapply(&self) -> bool {
        !self.adhesion_ok || !self.cured
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhesion_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = Primer::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_preparation() {
        let c = Primer::new();
        assert!(c.preparation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Primer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reapply() {
        let c = Primer::new();
        assert!(!c.needs_reapply());
    }

    #[test]
    fn test_adhesion() {
        let mut c = Primer::new();
        c.adhesion_ok = false;
        assert!(c.needs_reapply());
    }

    #[test]
    fn test_health() {
        let c = Primer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
