/// MirrorLink: display, input, audio, USB, certificate
/// Phase 993

#[derive(Debug, Clone)]
pub struct MirrorLink {
    pub display_ok: bool,
    pub input_ok: bool,
    pub audio_ok: bool,
    pub usb_ok: bool,
    pub cert_ok: bool,
}

impl Default for MirrorLink {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorLink {
    pub fn new() -> Self {
        Self {
            display_ok: true,
            input_ok: true,
            audio_ok: true,
            usb_ok: true,
            cert_ok: true,
        }
    }

    pub fn output_ok(&self) -> bool {
        self.display_ok && self.audio_ok
    }

    pub fn connection_ok(&self) -> bool {
        self.input_ok && self.usb_ok && self.cert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.output_ok() && self.connection_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.usb_ok || !self.cert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.usb_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let c = MirrorLink::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_connection() {
        let c = MirrorLink::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MirrorLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = MirrorLink::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_usb() {
        let mut c = MirrorLink::new();
        c.usb_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = MirrorLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
