/// aurora-ui-tree: ui tree
/// Phase 2414

#[derive(Debug, Clone)]
pub struct UiTree {
    pub expand_ok: bool,
    pub collapse_ok: bool,
    pub select_ok: bool,
    pub search_ok: bool,
    pub drag_ok: bool,
}

impl Default for UiTree {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTree {
    pub fn new() -> Self {
        Self {
            expand_ok: true,
            collapse_ok: true,
            select_ok: true,
            search_ok: true,
            drag_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.expand_ok && self.collapse_ok && self.select_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.search_ok && self.drag_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.expand_ok || !self.collapse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.expand_ok {
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
        let c = UiTree::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiTree::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiTree::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiTree::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiTree::new();
        c.expand_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiTree::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiTree::default();
        assert!(c.all_ok());
    }
}
