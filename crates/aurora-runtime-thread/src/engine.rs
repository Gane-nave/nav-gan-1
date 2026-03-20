/// runtime thread: spawn, join, park, unpark, log
/// Phase 1794

#[derive(Debug, Clone)]
pub struct RuntimeThread {
    pub spawn_ok: bool,
    pub join_ok: bool,
    pub park_ok: bool,
    pub unpark_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeThread {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeThread {
    pub fn new() -> Self {
        Self {
            spawn_ok: true,
            join_ok: true,
            park_ok: true,
            unpark_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spawn_ok && self.join_ok && self.park_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.unpark_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spawn_ok || !self.join_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spawn_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RuntimeThread::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeThread::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeThread::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeThread::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeThread::new();
        c.spawn_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeThread::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
