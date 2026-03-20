/// Parts catalog: search, compatibility, price, order
/// Phase 973

#[derive(Debug, Clone)]
pub struct PartsCat {
    pub search_ok: bool,
    pub compat_ok: bool,
    pub price_ok: bool,
    pub order_ok: bool,
    pub database_ok: bool,
}

impl Default for PartsCat {
    fn default() -> Self {
        Self::new()
    }
}

impl PartsCat {
    pub fn new() -> Self {
        Self {
            search_ok: true,
            compat_ok: true,
            price_ok: true,
            order_ok: true,
            database_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.search_ok && self.compat_ok && self.database_ok
    }

    pub fn commerce_ok(&self) -> bool {
        self.price_ok && self.order_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.commerce_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.price_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.database_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery() {
        let c = PartsCat::new();
        assert!(c.discovery_ok());
    }

    #[test]
    fn test_commerce() {
        let c = PartsCat::new();
        assert!(c.commerce_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PartsCat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = PartsCat::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = PartsCat::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = PartsCat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
