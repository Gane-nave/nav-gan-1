/// archive tar2: create, extract, list, append, log
/// Phase 2320

#[derive(Debug, Clone)]
pub struct ArchiveTar2 {
    pub create_ok: bool,
    pub extract_ok: bool,
    pub list_ok: bool,
    pub append_ok: bool,
    pub log_ok: bool,
}

impl Default for ArchiveTar2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchiveTar2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            extract_ok: true,
            list_ok: true,
            append_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.extract_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.append_ok && self.log_ok
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
        let c = ArchiveTar2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ArchiveTar2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ArchiveTar2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ArchiveTar2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ArchiveTar2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ArchiveTar2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
