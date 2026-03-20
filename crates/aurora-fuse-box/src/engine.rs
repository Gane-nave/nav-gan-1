/// Fuse box: relay, fuse, terminal, cover
/// Phase 688

#[derive(Debug, Clone)]
pub struct FuseBox {
    pub relay_ok: bool,
    pub fuse_ok: bool,
    pub terminal_ok: bool,
    pub cover_ok: bool,
    pub corrosion_free: bool,
}

impl Default for FuseBox {
    fn default() -> Self {
        Self::new()
    }
}

impl FuseBox {
    pub fn new() -> Self {
        Self {
            relay_ok: true,
            fuse_ok: true,
            terminal_ok: true,
            cover_ok: true,
            corrosion_free: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.fuse_ok && self.relay_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.terminal_ok && self.corrosion_free && self.cover_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.condition_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.fuse_ok || !self.corrosion_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.fuse_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = FuseBox::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_condition() {
        let c = FuseBox::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuseBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuseBox::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_fuse() {
        let mut c = FuseBox::new();
        c.fuse_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuseBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
