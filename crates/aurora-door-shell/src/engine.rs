/// Door shell: outer panel, inner panel, hinge, check
/// Phase 789

#[derive(Debug, Clone)]
pub struct DoorShell {
    pub outer_ok: bool,
    pub inner_ok: bool,
    pub hinge_ok: bool,
    pub check_ok: bool,
    pub seal_ok: bool,
}

impl Default for DoorShell {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorShell {
    pub fn new() -> Self {
        Self {
            outer_ok: true,
            inner_ok: true,
            hinge_ok: true,
            check_ok: true,
            seal_ok: true,
        }
    }

    pub fn panels_ok(&self) -> bool {
        self.outer_ok && self.inner_ok
    }

    pub fn mechanism_ok(&self) -> bool {
        self.hinge_ok && self.check_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.panels_ok() && self.mechanism_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.outer_ok || !self.hinge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hinge_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panels() {
        let c = DoorShell::new();
        assert!(c.panels_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = DoorShell::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DoorShell::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = DoorShell::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_outer() {
        let mut c = DoorShell::new();
        c.outer_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = DoorShell::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
