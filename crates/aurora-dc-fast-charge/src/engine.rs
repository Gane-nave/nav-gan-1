/// DC fast charging: CCS/CHAdeMO, high-power delivery, thermal management
/// Phase 290

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DcConnector {
    Ccs,
    Chademo,
    Tesla,
    Gbt,
}

#[derive(Debug, Clone)]
pub struct DcFastCharge {
    pub connector: DcConnector,
    pub power_kw: f64,
    pub max_power_kw: f64,
    pub voltage_v: f64,
    pub current_a: f64,
    pub temp_ok: bool,
}

impl Default for DcFastCharge {
    fn default() -> Self {
        Self::new()
    }
}

impl DcFastCharge {
    pub fn new() -> Self {
        Self {
            connector: DcConnector::Ccs,
            power_kw: 0.0,
            max_power_kw: 150.0,
            voltage_v: 400.0,
            current_a: 0.0,
            temp_ok: true,
        }
    }

    pub fn is_charging(&self) -> bool {
        self.power_kw > 1.0
    }

    pub fn ultra_fast(&self) -> bool {
        self.max_power_kw >= 250.0
    }

    pub fn power_pct(&self) -> f64 {
        if self.max_power_kw <= 0.0 {
            return 0.0;
        }
        self.power_kw / self.max_power_kw * 100.0
    }

    pub fn throttled(&self) -> bool {
        !self.temp_ok && self.power_kw < self.max_power_kw * 0.5
    }

    pub fn health_score(&self) -> f64 {
        if !self.temp_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_charging() {
        let d = DcFastCharge::new();
        assert!(!d.is_charging());
    }

    #[test]
    fn test_not_ultra() {
        let d = DcFastCharge::new();
        assert!(!d.ultra_fast());
    }

    #[test]
    fn test_zero_power() {
        let d = DcFastCharge::new();
        assert!(d.power_pct() < 0.1);
    }

    #[test]
    fn test_not_throttled() {
        let d = DcFastCharge::new();
        assert!(!d.throttled());
    }

    #[test]
    fn test_charging() {
        let mut d = DcFastCharge::new();
        d.power_kw = 100.0;
        assert!(d.is_charging());
    }

    #[test]
    fn test_health() {
        let d = DcFastCharge::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
