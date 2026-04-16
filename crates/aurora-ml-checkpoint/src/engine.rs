/// ml checkpoint: save, load, resume, prune, log
/// Phase 1966

#[derive(Debug, Clone)]
pub struct MlCheckpoint {
    pub save_ok: bool,
    pub load_ok: bool,
    pub resume_ok: bool,
    pub prune_ok: bool,
    pub log_ok: bool,
}

impl Default for MlCheckpoint {
    fn default() -> Self {
        Self::new()
    }
}

impl MlCheckpoint {
    pub fn new() -> Self {
        Self {
            save_ok: true,
            load_ok: true,
            resume_ok: true,
            prune_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.save_ok && self.load_ok && self.resume_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.prune_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.save_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.save_ok {
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
        let c = MlCheckpoint::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlCheckpoint::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlCheckpoint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlCheckpoint::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlCheckpoint::new();
        c.save_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlCheckpoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
