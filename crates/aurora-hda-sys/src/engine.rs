/// hda sys: map, plan, execute, merge, exit
/// Phase 1169

#[derive(Debug, Clone)]
pub struct HdaSys {
    pub map_ok: bool,
    pub plan_ok: bool,
    pub execute_ok: bool,
    pub merge_ok: bool,
    pub exit_ok: bool,
}

impl Default for HdaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl HdaSys {
    pub fn new() -> Self {
        Self {
            map_ok: true,
            plan_ok: true,
            execute_ok: true,
            merge_ok: true,
            exit_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.map_ok && self.plan_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.exit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.map_ok || !self.plan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.map_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = HdaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HdaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HdaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HdaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HdaSys::new();
        c.map_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HdaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
