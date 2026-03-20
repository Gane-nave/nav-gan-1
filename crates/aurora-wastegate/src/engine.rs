/// Wastegate: actuator, spring, diaphragm, boost control
/// Phase 603

#[derive(Debug, Clone)]
pub struct Wastegate {
    pub actuator_ok: bool,
    pub spring_ok: bool,
    pub diaphragm_ok: bool,
    pub boost_ctrl_ok: bool,
    pub linkage_ok: bool,
}

impl Default for Wastegate {
    fn default() -> Self {
        Self::new()
    }
}

impl Wastegate {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            spring_ok: true,
            diaphragm_ok: true,
            boost_ctrl_ok: true,
            linkage_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.spring_ok && self.diaphragm_ok && self.linkage_ok
    }

    pub fn control_ok(&self) -> bool {
        self.actuator_ok && self.boost_ctrl_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.diaphragm_ok || !self.actuator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.diaphragm_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = Wastegate::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_control() {
        let c = Wastegate::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Wastegate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Wastegate::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_diaphragm() {
        let mut c = Wastegate::new();
        c.diaphragm_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Wastegate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
