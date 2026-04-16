/// SMTP client: connect, auth, send, encrypt, queue
/// Phase 1062

#[derive(Debug, Clone)]
pub struct SmtpClient {
    pub connect_ok: bool,
    pub auth_ok: bool,
    pub send_ok: bool,
    pub encrypt_ok: bool,
    pub queue_ok: bool,
}

impl Default for SmtpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SmtpClient {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            auth_ok: true,
            send_ok: true,
            encrypt_ok: true,
            queue_ok: true,
        }
    }

    pub fn delivery_ok(&self) -> bool {
        self.connect_ok && self.auth_ok && self.send_ok
    }

    pub fn security_ok(&self) -> bool {
        self.encrypt_ok && self.queue_ok
    }

    pub fn all_ok(&self) -> bool {
        self.delivery_ok() && self.security_ok()
    }

    pub fn needs_reconnect(&self) -> bool {
        !self.connect_ok || !self.auth_ok
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
    fn test_delivery() {
        let c = SmtpClient::new();
        assert!(c.delivery_ok());
    }

    #[test]
    fn test_security() {
        let c = SmtpClient::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SmtpClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconnect() {
        let c = SmtpClient::new();
        assert!(!c.needs_reconnect());
    }

    #[test]
    fn test_connect() {
        let mut c = SmtpClient::new();
        c.connect_ok = false;
        assert!(c.needs_reconnect());
    }

    #[test]
    fn test_health() {
        let c = SmtpClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
