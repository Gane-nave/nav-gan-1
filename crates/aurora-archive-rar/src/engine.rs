/// archive rar: extract, list, test, repair, log
/// Phase 2323

#[derive(Debug, Clone)]
pub struct ArchiveRar {
    pub extract_ok: bool,
    pub list_ok: bool,
    pub test_ok: bool,
    pub repair_ok: bool,
    pub log_ok: bool,
}

impl Default for ArchiveRar {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchiveRar {
    pub fn new() -> Self {
        Self {
            extract_ok: true,
            list_ok: true,
            test_ok: true,
            repair_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.extract_ok && self.list_ok && self.test_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.repair_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.extract_ok || !self.list_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.extract_ok {
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
        let c = ArchiveRar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ArchiveRar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ArchiveRar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ArchiveRar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ArchiveRar::new();
        c.extract_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ArchiveRar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
