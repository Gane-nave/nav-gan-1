/// integ mysql2: connect, query, execute, pool, log
/// Phase 2254

#[derive(Debug, Clone)]
pub struct IntegMysql2 {
    pub connect_ok: bool,
    pub query_ok: bool,
    pub execute_ok: bool,
    pub pool_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegMysql2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegMysql2 {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            query_ok: true,
            execute_ok: true,
            pool_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.query_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pool_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = IntegMysql2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegMysql2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegMysql2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegMysql2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegMysql2::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegMysql2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
