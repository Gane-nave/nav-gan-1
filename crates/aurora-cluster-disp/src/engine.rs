/// cluster disp: speed, rpm, fuel, temp, warning
/// Phase 1184

#[derive(Debug, Clone)]
pub struct ClusterDisp {
    pub speed_ok: bool,
    pub rpm_ok: bool,
    pub fuel_ok: bool,
    pub temp_ok: bool,
    pub warning_ok: bool,
}

impl Default for ClusterDisp {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterDisp {
    pub fn new() -> Self {
        Self {
            speed_ok: true,
            rpm_ok: true,
            fuel_ok: true,
            temp_ok: true,
            warning_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.speed_ok && self.rpm_ok && self.fuel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.temp_ok && self.warning_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.speed_ok || !self.rpm_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.speed_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ClusterDisp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ClusterDisp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClusterDisp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ClusterDisp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ClusterDisp::new();
        c.speed_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ClusterDisp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
