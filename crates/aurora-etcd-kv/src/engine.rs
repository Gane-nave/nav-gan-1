/// aurora-etcd-kv: etcd kv
/// Phase 2600

#[derive(Debug, Clone)]
pub struct EtcdKv {
    pub put_ok: bool,
    pub get_ok: bool,
    pub delete_ok: bool,
    pub watch_ok: bool,
    pub lease_ok: bool,
}

impl Default for EtcdKv {
    fn default() -> Self {
        Self::new()
    }
}

impl EtcdKv {
    pub fn new() -> Self {
        Self {
            put_ok: true,
            get_ok: true,
            delete_ok: true,
            watch_ok: true,
            lease_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.put_ok && self.get_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.watch_ok && self.lease_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.put_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.put_ok {
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
        let c = EtcdKv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EtcdKv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EtcdKv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EtcdKv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EtcdKv::new();
        c.put_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EtcdKv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EtcdKv::default();
        assert!(c.all_ok());
    }
}
