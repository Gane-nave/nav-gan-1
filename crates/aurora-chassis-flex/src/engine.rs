/// Chassis flex: torsional rigidity, welds, mounts
/// Phase 549

#[derive(Debug, Clone)]
pub struct ChassisFlex {
    pub rigidity_nm_deg: f64,
    pub min_rigidity: f64,
    pub welds_ok: bool,
    pub mounts_ok: bool,
    pub corrosion_free: bool,
}

impl Default for ChassisFlex {
    fn default() -> Self {
        Self::new()
    }
}

impl ChassisFlex {
    pub fn new() -> Self {
        Self {
            rigidity_nm_deg: 25000.0,
            min_rigidity: 15000.0,
            welds_ok: true,
            mounts_ok: true,
            corrosion_free: true,
        }
    }

    pub fn rigidity_ok(&self) -> bool {
        self.rigidity_nm_deg > self.min_rigidity
    }

    pub fn structural_ok(&self) -> bool {
        self.welds_ok && self.mounts_ok && self.corrosion_free
    }

    pub fn all_ok(&self) -> bool {
        self.rigidity_ok() && self.structural_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.welds_ok || !self.mounts_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.welds_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigidity() {
        let c = ChassisFlex::new();
        assert!(c.rigidity_ok());
    }

    #[test]
    fn test_structural() {
        let c = ChassisFlex::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChassisFlex::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = ChassisFlex::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_welds() {
        let mut c = ChassisFlex::new();
        c.welds_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = ChassisFlex::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
