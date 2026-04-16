/// Car sharing: booking, access, billing, condition check
/// Phase 894

#[derive(Debug, Clone)]
pub struct CarShare {
    pub booking_ok: bool,
    pub access_ok: bool,
    pub billing_ok: bool,
    pub condition_ok: bool,
    pub comm_ok: bool,
}

impl Default for CarShare {
    fn default() -> Self {
        Self::new()
    }
}

impl CarShare {
    pub fn new() -> Self {
        Self {
            booking_ok: true,
            access_ok: true,
            billing_ok: true,
            condition_ok: true,
            comm_ok: true,
        }
    }

    pub fn service_ok(&self) -> bool {
        self.booking_ok && self.access_ok && self.comm_ok
    }

    pub fn business_ok(&self) -> bool {
        self.billing_ok && self.condition_ok
    }

    pub fn all_ok(&self) -> bool {
        self.service_ok() && self.business_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.booking_ok || !self.access_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.booking_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service() {
        let c = CarShare::new();
        assert!(c.service_ok());
    }

    #[test]
    fn test_business() {
        let c = CarShare::new();
        assert!(c.business_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CarShare::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = CarShare::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_booking() {
        let mut c = CarShare::new();
        c.booking_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = CarShare::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
