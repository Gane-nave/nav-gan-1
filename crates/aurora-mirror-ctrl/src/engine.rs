/// mirror ctrl: adjust, fold, heat, dim, memory
/// Phase 1187

#[derive(Debug, Clone)]
pub struct MirrorCtrl {
    pub adjust_ok: bool,
    pub fold_ok: bool,
    pub heat_ok: bool,
    pub dim_ok: bool,
    pub memory_ok: bool,
}

impl Default for MirrorCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorCtrl {
    pub fn new() -> Self {
        Self {
            adjust_ok: true,
            fold_ok: true,
            heat_ok: true,
            dim_ok: true,
            memory_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.adjust_ok && self.fold_ok && self.heat_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dim_ok && self.memory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.adjust_ok || !self.fold_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.adjust_ok {
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
        let c = MirrorCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MirrorCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MirrorCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MirrorCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MirrorCtrl::new();
        c.adjust_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MirrorCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
