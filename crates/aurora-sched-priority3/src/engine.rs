/// sched priority3: insert, extract, update, peek, log
/// Phase 2348

#[derive(Debug, Clone)]
pub struct SchedPriority3 {
    pub insert_ok: bool,
    pub extract_ok: bool,
    pub update_ok: bool,
    pub peek_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedPriority3 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedPriority3 {
    pub fn new() -> Self {
        Self {
            insert_ok: true,
            extract_ok: true,
            update_ok: true,
            peek_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.insert_ok && self.extract_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.peek_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.insert_ok || !self.extract_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.insert_ok {
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
        let c = SchedPriority3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedPriority3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedPriority3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedPriority3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedPriority3::new();
        c.insert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedPriority3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
