/// ux layout: define, arrange, adapt, animate, log
/// Phase 1501

#[derive(Debug, Clone)]
pub struct UxLayout {
    pub define_ok: bool,
    pub arrange_ok: bool,
    pub adapt_ok: bool,
    pub animate_ok: bool,
    pub log_ok: bool,
}

impl Default for UxLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl UxLayout {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            arrange_ok: true,
            adapt_ok: true,
            animate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.arrange_ok && self.adapt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.animate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.arrange_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = UxLayout::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxLayout::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxLayout::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxLayout::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxLayout::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxLayout::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
