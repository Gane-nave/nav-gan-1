/// Airbag control: deployment logic, crash severity, occupant classification
/// Phase 225

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AirbagStatus {
    Ready,
    Deployed,
    Fault,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct AirbagController {
    pub driver_status: AirbagStatus,
    pub passenger_status: AirbagStatus,
    pub side_curtain_status: AirbagStatus,
    pub passenger_present: bool,
    pub child_seat_detected: bool,
    pub crash_severity_g: f64,
}

impl Default for AirbagController {
    fn default() -> Self {
        Self::new()
    }
}

impl AirbagController {
    pub fn new() -> Self {
        Self {
            driver_status: AirbagStatus::Ready,
            passenger_status: AirbagStatus::Ready,
            side_curtain_status: AirbagStatus::Ready,
            passenger_present: true,
            child_seat_detected: false,
            crash_severity_g: 0.0,
        }
    }

    pub fn all_ready(&self) -> bool {
        self.driver_status == AirbagStatus::Ready
            && self.passenger_status == AirbagStatus::Ready
            && self.side_curtain_status == AirbagStatus::Ready
    }

    pub fn any_deployed(&self) -> bool {
        self.driver_status == AirbagStatus::Deployed
            || self.passenger_status == AirbagStatus::Deployed
            || self.side_curtain_status == AirbagStatus::Deployed
    }

    pub fn has_fault(&self) -> bool {
        self.driver_status == AirbagStatus::Fault
            || self.passenger_status == AirbagStatus::Fault
            || self.side_curtain_status == AirbagStatus::Fault
    }

    pub fn passenger_disabled(&self) -> bool {
        self.child_seat_detected || !self.passenger_present
    }

    pub fn health_score(&self) -> f64 {
        if self.has_fault() {
            return 20.0;
        }
        if self.any_deployed() {
            return 0.0;
        }
        if !self.all_ready() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ready() {
        let a = AirbagController::new();
        assert!(a.all_ready());
    }

    #[test]
    fn test_none_deployed() {
        let a = AirbagController::new();
        assert!(!a.any_deployed());
    }

    #[test]
    fn test_no_fault() {
        let a = AirbagController::new();
        assert!(!a.has_fault());
    }

    #[test]
    fn test_passenger_not_disabled() {
        let a = AirbagController::new();
        assert!(!a.passenger_disabled());
    }

    #[test]
    fn test_child_seat() {
        let mut a = AirbagController::new();
        a.child_seat_detected = true;
        assert!(a.passenger_disabled());
    }

    #[test]
    fn test_health() {
        let a = AirbagController::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
