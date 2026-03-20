/// Container runtime: create, start, stop, exec, inspect
/// Phase 1068

#[derive(Debug, Clone)]
pub struct ContainerRt {
    pub create_ok: bool,
    pub start_ok: bool,
    pub stop_ok: bool,
    pub exec_ok: bool,
    pub inspect_ok: bool,
}

impl Default for ContainerRt {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerRt {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            start_ok: true,
            stop_ok: true,
            exec_ok: true,
            inspect_ok: true,
        }
    }

    pub fn lifecycle_ok(&self) -> bool {
        self.create_ok && self.start_ok && self.stop_ok
    }

    pub fn management_ok(&self) -> bool {
        self.exec_ok && self.inspect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.lifecycle_ok() && self.management_ok()
    }

    pub fn needs_restart(&self) -> bool {
        !self.start_ok || !self.stop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle() {
        let c = ContainerRt::new();
        assert!(c.lifecycle_ok());
    }

    #[test]
    fn test_management() {
        let c = ContainerRt::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ContainerRt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_restart() {
        let c = ContainerRt::new();
        assert!(!c.needs_restart());
    }

    #[test]
    fn test_start() {
        let mut c = ContainerRt::new();
        c.start_ok = false;
        assert!(c.needs_restart());
    }

    #[test]
    fn test_health() {
        let c = ContainerRt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
