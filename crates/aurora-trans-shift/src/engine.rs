/// Transmission shift: gear selection, shift timing, torque converter
/// Phase 161

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GearPosition {
    Park,
    Reverse,
    Neutral,
    Drive,
    Manual(u8),
}

impl GearPosition {
    pub fn is_drive(&self) -> bool {
        matches!(self, GearPosition::Drive | GearPosition::Manual(_))
    }

    pub fn gear_number(&self) -> Option<u8> {
        match self {
            GearPosition::Manual(g) => Some(*g),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransmissionSystem {
    pub position: GearPosition,
    pub current_gear: u8,
    pub max_gears: u8,
    pub rpm: f64,
    pub shift_rpm_up: f64,
    pub shift_rpm_down: f64,
    pub fluid_temp_c: f64,
}

impl Default for TransmissionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TransmissionSystem {
    pub fn new() -> Self {
        Self {
            position: GearPosition::Park,
            current_gear: 1,
            max_gears: 8,
            rpm: 800.0,
            shift_rpm_up: 6000.0,
            shift_rpm_down: 1500.0,
            fluid_temp_c: 80.0,
        }
    }

    pub fn should_upshift(&self) -> bool {
        self.position.is_drive()
            && self.rpm > self.shift_rpm_up
            && self.current_gear < self.max_gears
    }

    pub fn should_downshift(&self) -> bool {
        self.position.is_drive() && self.rpm < self.shift_rpm_down && self.current_gear > 1
    }

    pub fn gear_ratio(&self) -> f64 {
        match self.current_gear {
            1 => 3.5,
            2 => 2.5,
            3 => 1.8,
            4 => 1.4,
            5 => 1.1,
            6 => 0.9,
            7 => 0.75,
            8 => 0.65,
            _ => 1.0,
        }
    }

    pub fn fluid_overheated(&self) -> bool {
        self.fluid_temp_c > 130.0
    }

    pub fn is_in_park(&self) -> bool {
        matches!(self.position, GearPosition::Park)
    }

    pub fn optimal_gear_for_speed(&self, speed_kmh: f64) -> u8 {
        if speed_kmh < 20.0 {
            1
        } else if speed_kmh < 40.0 {
            2
        } else if speed_kmh < 60.0 {
            3
        } else if speed_kmh < 80.0 {
            4
        } else if speed_kmh < 100.0 {
            5
        } else if speed_kmh < 120.0 {
            6
        } else if speed_kmh < 150.0 {
            7
        } else {
            8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_drive() {
        assert!(GearPosition::Drive.is_drive());
        assert!(!GearPosition::Park.is_drive());
    }

    #[test]
    fn test_manual_gear() {
        assert_eq!(GearPosition::Manual(3).gear_number(), Some(3));
    }

    #[test]
    fn test_upshift() {
        let mut s = TransmissionSystem::new();
        s.position = GearPosition::Drive;
        s.rpm = 6500.0;
        assert!(s.should_upshift());
    }

    #[test]
    fn test_downshift() {
        let mut s = TransmissionSystem::new();
        s.position = GearPosition::Drive;
        s.current_gear = 3;
        s.rpm = 1200.0;
        assert!(s.should_downshift());
    }

    #[test]
    fn test_gear_ratio() {
        let s = TransmissionSystem::new();
        assert!(s.gear_ratio() > 3.0);
    }

    #[test]
    fn test_not_overheated() {
        let s = TransmissionSystem::new();
        assert!(!s.fluid_overheated());
    }

    #[test]
    fn test_in_park() {
        let s = TransmissionSystem::new();
        assert!(s.is_in_park());
    }

    #[test]
    fn test_optimal_gear() {
        let s = TransmissionSystem::new();
        assert_eq!(s.optimal_gear_for_speed(90.0), 5);
    }
}
