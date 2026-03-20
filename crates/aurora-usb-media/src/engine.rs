/// usb media: detect, mount, index, play, eject
/// Phase 1179

#[derive(Debug, Clone)]
pub struct UsbMedia {
    pub detect_ok: bool,
    pub mount_ok: bool,
    pub index_ok: bool,
    pub play_ok: bool,
    pub eject_ok: bool,
}

impl Default for UsbMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbMedia {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            mount_ok: true,
            index_ok: true,
            play_ok: true,
            eject_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.mount_ok && self.index_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.play_ok && self.eject_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.mount_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = UsbMedia::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UsbMedia::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UsbMedia::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UsbMedia::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UsbMedia::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UsbMedia::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
