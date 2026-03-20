/// load manage: prioritize, shed, restore, balance, log
/// Phase 1372

#[derive(Debug, Clone)]
pub struct LoadManage {
    pub prioritize_ok: bool,
    pub shed_ok: bool,
    pub restore_ok: bool,
    pub balance_ok: bool,
    pub log_ok: bool,
}

impl Default for LoadManage {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadManage {
    pub fn new() -> Self {
        Self {
            prioritize_ok: true,
            shed_ok: true,
            restore_ok: true,
            balance_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.prioritize_ok && self.shed_ok && self.restore_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.prioritize_ok || !self.shed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.prioritize_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = LoadManage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LoadManage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LoadManage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LoadManage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LoadManage::new();
        c.prioritize_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LoadManage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
