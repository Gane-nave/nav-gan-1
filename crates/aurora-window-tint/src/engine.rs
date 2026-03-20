/// Window tint: VLT, UV rejection, IR rejection, legal
/// Phase 769

#[derive(Debug, Clone)]
pub struct WindowTint {
    pub vlt_ok: bool,
    pub uv_ok: bool,
    pub ir_ok: bool,
    pub legal_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for WindowTint {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowTint {
    pub fn new() -> Self {
        Self {
            vlt_ok: true,
            uv_ok: true,
            ir_ok: true,
            legal_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn performance_ok(&self) -> bool {
        self.uv_ok && self.ir_ok
    }

    pub fn compliance_ok(&self) -> bool {
        self.vlt_ok && self.legal_ok && self.adhesion_ok
    }

    pub fn all_ok(&self) -> bool {
        self.performance_ok() && self.compliance_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.adhesion_ok || !self.legal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhesion_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance() {
        let c = WindowTint::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_compliance() {
        let c = WindowTint::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WindowTint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WindowTint::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_adhesion() {
        let mut c = WindowTint::new();
        c.adhesion_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WindowTint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
