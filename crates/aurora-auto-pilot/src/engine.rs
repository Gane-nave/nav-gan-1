/// Auto pilot: perceive, plan, act, monitor, disengage
/// Phase 1104

#[derive(Debug, Clone)]
pub struct AutoPilot {
    pub perceive_ok: bool,
    pub plan_ok: bool,
    pub act_ok: bool,
    pub monitor_ok: bool,
    pub disengage_ok: bool,
}

impl Default for AutoPilot {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoPilot {
    pub fn new() -> Self {
        Self {
            perceive_ok: true,
            plan_ok: true,
            act_ok: true,
            monitor_ok: true,
            disengage_ok: true,
        }
    }

    pub fn driving_ok(&self) -> bool {
        self.perceive_ok && self.plan_ok && self.act_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.monitor_ok && self.disengage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.driving_ok() && self.safety_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.perceive_ok || !self.plan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.perceive_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driving() {
        let c = AutoPilot::new();
        assert!(c.driving_ok());
    }

    #[test]
    fn test_safety() {
        let c = AutoPilot::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoPilot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = AutoPilot::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_perceive() {
        let mut c = AutoPilot::new();
        c.perceive_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = AutoPilot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
