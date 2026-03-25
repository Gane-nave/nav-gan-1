/// Power outlet: 12V, USB-A, USB-C, 120V inverter
/// Phase 904

#[derive(Debug, Clone)]
pub struct PowerOutlet {
    pub dc12v_ok: bool,
    pub usb_a_ok: bool,
    pub usb_c_ok: bool,
    pub inverter_ok: bool,
    pub fuse_ok: bool,
}

impl Default for PowerOutlet {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerOutlet {
    pub fn new() -> Self {
        Self {
            dc12v_ok: true,
            usb_a_ok: true,
            usb_c_ok: true,
            inverter_ok: true,
            fuse_ok: true,
        }
    }

    pub fn low_power_ok(&self) -> bool {
        self.dc12v_ok && self.usb_a_ok && self.usb_c_ok
    }

    pub fn high_power_ok(&self) -> bool {
        self.inverter_ok && self.fuse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.low_power_ok() && self.high_power_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.fuse_ok || !self.inverter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fuse_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_power() {
        let c = PowerOutlet::new();
        assert!(c.low_power_ok());
    }

    #[test]
    fn test_high_power() {
        let c = PowerOutlet::new();
        assert!(c.high_power_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerOutlet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PowerOutlet::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_fuse() {
        let mut c = PowerOutlet::new();
        c.fuse_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PowerOutlet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
