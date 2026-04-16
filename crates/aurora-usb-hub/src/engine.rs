/// USB hub: charging, data transfer, port condition, power delivery
/// Phase 435

#[derive(Debug, Clone)]
pub struct UsbHub {
    pub port_count: u8,
    pub working_ports: u8,
    pub charging_ok: bool,
    pub data_ok: bool,
    pub power_w: f64,
}

impl Default for UsbHub {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbHub {
    pub fn new() -> Self {
        Self {
            port_count: 4,
            working_ports: 4,
            charging_ok: true,
            data_ok: true,
            power_w: 15.0,
        }
    }

    pub fn all_ports_ok(&self) -> bool {
        self.working_ports >= self.port_count
    }

    pub fn all_ok(&self) -> bool {
        self.all_ports_ok() && self.charging_ok && self.data_ok
    }

    pub fn needs_service(&self) -> bool {
        self.working_ports == 0 || !self.charging_ok
    }

    pub fn fast_charge(&self) -> bool {
        self.power_w >= 15.0
    }

    pub fn health_score(&self) -> f64 {
        if self.working_ports == 0 {
            return 0.0;
        }
        if !self.all_ports_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ports() {
        let u = UsbHub::new();
        assert!(u.all_ports_ok());
    }

    #[test]
    fn test_all_ok() {
        let u = UsbHub::new();
        assert!(u.all_ok());
    }

    #[test]
    fn test_no_service() {
        let u = UsbHub::new();
        assert!(!u.needs_service());
    }

    #[test]
    fn test_fast_charge() {
        let u = UsbHub::new();
        assert!(u.fast_charge());
    }

    #[test]
    fn test_dead_ports() {
        let mut u = UsbHub::new();
        u.working_ports = 0;
        assert!(u.needs_service());
    }

    #[test]
    fn test_health() {
        let u = UsbHub::new();
        assert!((u.health_score() - 100.0).abs() < 0.1);
    }
}
