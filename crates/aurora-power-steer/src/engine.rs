/// Power steering: electric/hydraulic assist, torque sensing, rack force
/// Phase 222

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SteeringType {
    ElectricAssist,
    HydraulicAssist,
    ElectroHydraulic,
}

#[derive(Debug, Clone)]
pub struct PowerSteering {
    pub steering_type: SteeringType,
    pub assist_level_pct: f64,
    pub torque_nm: f64,
    pub motor_current_a: f64,
    pub fluid_level_ok: bool,
    pub fault: bool,
}

impl Default for PowerSteering {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerSteering {
    pub fn new() -> Self {
        Self {
            steering_type: SteeringType::ElectricAssist,
            assist_level_pct: 80.0,
            torque_nm: 5.0,
            motor_current_a: 15.0,
            fluid_level_ok: true,
            fault: false,
        }
    }

    pub fn assist_ok(&self) -> bool {
        self.assist_level_pct > 20.0 && !self.fault
    }

    pub fn heavy_steering(&self) -> bool {
        self.assist_level_pct < 30.0
    }

    pub fn motor_overloaded(&self) -> bool {
        self.motor_current_a > 50.0
    }

    pub fn needs_service(&self) -> bool {
        self.fault || self.motor_overloaded() || !self.fluid_level_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.fault {
            return 10.0;
        }
        let mut score: f64 = 100.0;
        if self.motor_overloaded() {
            score -= 30.0;
        }
        if !self.fluid_level_ok {
            score -= 25.0;
        }
        if self.heavy_steering() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assist_ok() {
        let p = PowerSteering::new();
        assert!(p.assist_ok());
    }

    #[test]
    fn test_not_heavy() {
        let p = PowerSteering::new();
        assert!(!p.heavy_steering());
    }

    #[test]
    fn test_not_overloaded() {
        let p = PowerSteering::new();
        assert!(!p.motor_overloaded());
    }

    #[test]
    fn test_no_service() {
        let p = PowerSteering::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_fault() {
        let mut p = PowerSteering::new();
        p.fault = true;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = PowerSteering::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
