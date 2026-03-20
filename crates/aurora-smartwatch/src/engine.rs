/// Smartwatch: notify, control, status, health, find
/// Phase 995

#[derive(Debug, Clone)]
pub struct Smartwatch {
    pub notify_ok: bool,
    pub control_ok: bool,
    pub status_ok: bool,
    pub health_ok: bool,
    pub find_ok: bool,
}

impl Default for Smartwatch {
    fn default() -> Self {
        Self::new()
    }
}

impl Smartwatch {
    pub fn new() -> Self {
        Self {
            notify_ok: true,
            control_ok: true,
            status_ok: true,
            health_ok: true,
            find_ok: true,
        }
    }

    pub fn alerts_ok(&self) -> bool {
        self.notify_ok && self.status_ok && self.health_ok
    }

    pub fn remote_ok(&self) -> bool {
        self.control_ok && self.find_ok
    }

    pub fn all_ok(&self) -> bool {
        self.alerts_ok() && self.remote_ok()
    }

    pub fn needs_pair(&self) -> bool {
        !self.notify_ok || !self.control_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.notify_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerts() {
        let c = Smartwatch::new();
        assert!(c.alerts_ok());
    }

    #[test]
    fn test_remote() {
        let c = Smartwatch::new();
        assert!(c.remote_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Smartwatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_pair() {
        let c = Smartwatch::new();
        assert!(!c.needs_pair());
    }

    #[test]
    fn test_notify() {
        let mut c = Smartwatch::new();
        c.notify_ok = false;
        assert!(c.needs_pair());
    }

    #[test]
    fn test_health() {
        let c = Smartwatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
