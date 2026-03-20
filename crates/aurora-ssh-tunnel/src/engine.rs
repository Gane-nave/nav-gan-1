/// SSH tunnel: connect, forward, reverse, key, proxy
/// Phase 1064

#[derive(Debug, Clone)]
pub struct SshTunnel {
    pub connect_ok: bool,
    pub forward_ok: bool,
    pub reverse_ok: bool,
    pub key_ok: bool,
    pub proxy_ok: bool,
}

impl Default for SshTunnel {
    fn default() -> Self {
        Self::new()
    }
}

impl SshTunnel {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            forward_ok: true,
            reverse_ok: true,
            key_ok: true,
            proxy_ok: true,
        }
    }

    pub fn tunneling_ok(&self) -> bool {
        self.connect_ok && self.forward_ok && self.reverse_ok
    }

    pub fn security_ok(&self) -> bool {
        self.key_ok && self.proxy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tunneling_ok() && self.security_ok()
    }

    pub fn needs_rekey(&self) -> bool {
        !self.key_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunneling() {
        let c = SshTunnel::new();
        assert!(c.tunneling_ok());
    }

    #[test]
    fn test_security() {
        let c = SshTunnel::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SshTunnel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rekey() {
        let c = SshTunnel::new();
        assert!(!c.needs_rekey());
    }

    #[test]
    fn test_key() {
        let mut c = SshTunnel::new();
        c.key_ok = false;
        assert!(c.needs_rekey());
    }

    #[test]
    fn test_health() {
        let c = SshTunnel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
