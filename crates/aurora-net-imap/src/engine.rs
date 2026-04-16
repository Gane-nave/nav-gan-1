/// net imap: connect, list, fetch, search, log
/// Phase 1552

#[derive(Debug, Clone)]
pub struct NetImap {
    pub connect_ok: bool,
    pub list_ok: bool,
    pub fetch_ok: bool,
    pub search_ok: bool,
    pub log_ok: bool,
}

impl Default for NetImap {
    fn default() -> Self {
        Self::new()
    }
}

impl NetImap {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            list_ok: true,
            fetch_ok: true,
            search_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.list_ok && self.fetch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.search_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.list_ok
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
        let c = NetImap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetImap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetImap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetImap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetImap::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetImap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
