/// Auto parking: space detection, automated steering, parallel/perpendicular
/// Phase 268

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParkType {
    Parallel,
    Perpendicular,
    Angled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParkPhase {
    Scanning,
    SpaceFound,
    Maneuvering,
    Complete,
    Idle,
}

#[derive(Debug, Clone)]
pub struct AutoPark {
    pub park_type: ParkType,
    pub phase: ParkPhase,
    pub space_length_m: f64,
    pub vehicle_length_m: f64,
    pub progress_pct: f64,
    pub system_ok: bool,
}

impl Default for AutoPark {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoPark {
    pub fn new() -> Self {
        Self {
            park_type: ParkType::Parallel,
            phase: ParkPhase::Idle,
            space_length_m: 0.0,
            vehicle_length_m: 4.5,
            progress_pct: 0.0,
            system_ok: true,
        }
    }

    pub fn space_adequate(&self) -> bool {
        self.space_length_m > self.vehicle_length_m * 1.3
    }

    pub fn is_active(&self) -> bool {
        self.phase != ParkPhase::Idle && self.phase != ParkPhase::Complete
    }

    pub fn is_complete(&self) -> bool {
        self.phase == ParkPhase::Complete
    }

    pub fn health_score(&self) -> f64 {
        if !self.system_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_active() {
        let a = AutoPark::new();
        assert!(!a.is_active());
    }

    #[test]
    fn test_not_complete() {
        let a = AutoPark::new();
        assert!(!a.is_complete());
    }

    #[test]
    fn test_space_too_small() {
        let a = AutoPark::new();
        assert!(!a.space_adequate());
    }

    #[test]
    fn test_space_ok() {
        let mut a = AutoPark::new();
        a.space_length_m = 7.0;
        assert!(a.space_adequate());
    }

    #[test]
    fn test_scanning() {
        let mut a = AutoPark::new();
        a.phase = ParkPhase::Scanning;
        assert!(a.is_active());
    }

    #[test]
    fn test_health() {
        let a = AutoPark::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
