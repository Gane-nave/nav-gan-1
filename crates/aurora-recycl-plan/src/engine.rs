/// Recycling planner: material, disassemble, recover, dispose
/// Phase 977

#[derive(Debug, Clone)]
pub struct RecyclPlan {
    pub material_ok: bool,
    pub disassemble_ok: bool,
    pub recover_ok: bool,
    pub dispose_ok: bool,
    pub comply_ok: bool,
}

impl Default for RecyclPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl RecyclPlan {
    pub fn new() -> Self {
        Self {
            material_ok: true,
            disassemble_ok: true,
            recover_ok: true,
            dispose_ok: true,
            comply_ok: true,
        }
    }

    pub fn process_ok(&self) -> bool {
        self.material_ok && self.disassemble_ok && self.recover_ok
    }

    pub fn compliance_ok(&self) -> bool {
        self.dispose_ok && self.comply_ok
    }

    pub fn all_ok(&self) -> bool {
        self.process_ok() && self.compliance_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.comply_ok || !self.material_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.material_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let c = RecyclPlan::new();
        assert!(c.process_ok());
    }

    #[test]
    fn test_compliance() {
        let c = RecyclPlan::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RecyclPlan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = RecyclPlan::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_comply() {
        let mut c = RecyclPlan::new();
        c.comply_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = RecyclPlan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
