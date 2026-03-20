/// archive cpio: create, extract, list, convert, log
/// Phase 2324

#[derive(Debug, Clone)]
pub struct ArchiveCpio {
    pub create_ok: bool,
    pub extract_ok: bool,
    pub list_ok: bool,
    pub convert_ok: bool,
    pub log_ok: bool,
}

impl Default for ArchiveCpio {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchiveCpio {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            extract_ok: true,
            list_ok: true,
            convert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.extract_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.convert_ok && self.log_ok
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
        let c = ArchiveCpio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ArchiveCpio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ArchiveCpio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ArchiveCpio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ArchiveCpio::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ArchiveCpio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
