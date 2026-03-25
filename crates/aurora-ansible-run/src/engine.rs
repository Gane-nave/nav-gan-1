/// Ansible runner: playbook, role, task, handler, vault
/// Phase 1072

#[derive(Debug, Clone)]
pub struct AnsibleRun {
    pub playbook_ok: bool,
    pub role_ok: bool,
    pub task_ok: bool,
    pub handler_ok: bool,
    pub vault_ok: bool,
}

impl Default for AnsibleRun {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsibleRun {
    pub fn new() -> Self {
        Self {
            playbook_ok: true,
            role_ok: true,
            task_ok: true,
            handler_ok: true,
            vault_ok: true,
        }
    }

    pub fn execution_ok(&self) -> bool {
        self.playbook_ok && self.role_ok && self.task_ok
    }

    pub fn management_ok(&self) -> bool {
        self.handler_ok && self.vault_ok
    }

    pub fn all_ok(&self) -> bool {
        self.execution_ok() && self.management_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.playbook_ok || !self.role_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.playbook_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution() {
        let c = AnsibleRun::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_management() {
        let c = AnsibleRun::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnsibleRun::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = AnsibleRun::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_playbook() {
        let mut c = AnsibleRun::new();
        c.playbook_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = AnsibleRun::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
