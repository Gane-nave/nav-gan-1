/// archive ar: create, extract, list, replace, log
/// Phase 2326

#[derive(Debug, Clone)]
pub struct ArchiveAr {
    pub create_ok: bool,
    pub extract_ok: bool,
    pub list_ok: bool,
    pub replace_ok: bool,
    pub log_ok: bool,
}

impl Default for ArchiveAr {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchiveAr {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            extract_ok: true,
            list_ok: true,
            replace_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.extract_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replace_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.extract_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = ArchiveAr::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ArchiveAr::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ArchiveAr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ArchiveAr::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ArchiveAr::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ArchiveAr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
