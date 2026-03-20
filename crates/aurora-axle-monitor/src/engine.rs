/// Axle monitoring for vehicle health and load management
/// Phase 133: Tracks axle temperatures, bearing wear, alignment

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxlePosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

impl AxlePosition {
    pub fn is_front(&self) -> bool {
        matches!(self, AxlePosition::FrontLeft | AxlePosition::FrontRight)
    }

    pub fn is_drive_axle(&self, awd: bool) -> bool {
        if awd {
            true
        } else {
            self.is_front()
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AxlePosition::FrontLeft => "FL",
            AxlePosition::FrontRight => "FR",
            AxlePosition::RearLeft => "RL",
            AxlePosition::RearRight => "RR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxleCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl AxleCondition {
    pub fn needs_service(&self) -> bool {
        matches!(self, AxleCondition::Poor | AxleCondition::Critical)
    }

    pub fn max_speed_kmh(&self) -> f64 {
        match self {
            AxleCondition::Excellent => 200.0,
            AxleCondition::Good => 160.0,
            AxleCondition::Fair => 120.0,
            AxleCondition::Poor => 80.0,
            AxleCondition::Critical => 40.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AxleSensor {
    pub position: AxlePosition,
    pub temperature_c: f64,
    pub vibration_hz: f64,
    pub bearing_wear_pct: f64,
    pub alignment_offset_deg: f64,
}

impl AxleSensor {
    pub fn new(pos: AxlePosition, temp: f64, vib: f64, wear: f64, align: f64) -> Self {
        Self {
            position: pos,
            temperature_c: temp,
            vibration_hz: vib,
            bearing_wear_pct: wear,
            alignment_offset_deg: align,
        }
    }

    pub fn condition(&self) -> AxleCondition {
        if self.bearing_wear_pct > 90.0 || self.temperature_c > 120.0 {
            AxleCondition::Critical
        } else if self.bearing_wear_pct > 70.0 || self.temperature_c > 100.0 {
            AxleCondition::Poor
        } else if self.bearing_wear_pct > 50.0 || self.temperature_c > 80.0 {
            AxleCondition::Fair
        } else if self.bearing_wear_pct > 25.0 {
            AxleCondition::Good
        } else {
            AxleCondition::Excellent
        }
    }

    pub fn is_overheating(&self) -> bool {
        self.temperature_c > 100.0
    }

    pub fn is_misaligned(&self) -> bool {
        self.alignment_offset_deg.abs() > 1.0
    }

    pub fn excessive_vibration(&self) -> bool {
        self.vibration_hz > 50.0
    }

    pub fn remaining_life_km(&self) -> f64 {
        let remaining_pct = 100.0 - self.bearing_wear_pct;
        let base_life = 150_000.0;
        (remaining_pct / 100.0 * base_life).max(0.0)
    }

    pub fn service_urgency(&self) -> &'static str {
        match self.condition() {
            AxleCondition::Critical => "immediate",
            AxleCondition::Poor => "soon",
            AxleCondition::Fair => "scheduled",
            _ => "none",
        }
    }

    pub fn health_score(&self) -> f64 {
        let wear_score = 100.0 - self.bearing_wear_pct;
        let temp_score = if self.temperature_c < 60.0 {
            100.0
        } else if self.temperature_c < 100.0 {
            100.0 - (self.temperature_c - 60.0) * 1.5
        } else {
            20.0
        };
        let align_score = if self.alignment_offset_deg.abs() < 0.5 {
            100.0
        } else {
            (100.0 - self.alignment_offset_deg.abs() * 20.0).max(0.0)
        };
        (wear_score * 0.5 + temp_score * 0.3 + align_score * 0.2).min(100.0)
    }
}

#[derive(Debug, Clone)]
pub struct AxleSet {
    pub sensors: Vec<AxleSensor>,
}

impl Default for AxleSet {
    fn default() -> Self {
        Self::new()
    }
}

impl AxleSet {
    pub fn new() -> Self {
        Self {
            sensors: Vec::new(),
        }
    }

    pub fn add(&mut self, s: AxleSensor) {
        self.sensors.push(s);
    }

    pub fn worst_condition(&self) -> AxleCondition {
        self.sensors
            .iter()
            .map(|s| s.condition())
            .min_by_key(|c| match c {
                AxleCondition::Critical => 0,
                AxleCondition::Poor => 1,
                AxleCondition::Fair => 2,
                AxleCondition::Good => 3,
                AxleCondition::Excellent => 4,
            })
            .unwrap_or(AxleCondition::Excellent)
    }

    pub fn any_overheating(&self) -> bool {
        self.sensors.iter().any(|s| s.is_overheating())
    }

    pub fn any_misaligned(&self) -> bool {
        self.sensors.iter().any(|s| s.is_misaligned())
    }

    pub fn min_remaining_life_km(&self) -> f64 {
        self.sensors
            .iter()
            .map(|s| s.remaining_life_km())
            .fold(f64::MAX, f64::min)
    }

    pub fn average_health(&self) -> f64 {
        if self.sensors.is_empty() {
            return 100.0;
        }
        let total: f64 = self.sensors.iter().map(|s| s.health_score()).sum();
        total / self.sensors.len() as f64
    }

    pub fn max_safe_speed(&self) -> f64 {
        self.sensors
            .iter()
            .map(|s| s.condition().max_speed_kmh())
            .fold(f64::MAX, f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_front() {
        assert!(AxlePosition::FrontLeft.is_front());
        assert!(!AxlePosition::RearLeft.is_front());
    }

    #[test]
    fn test_drive_axle_awd() {
        assert!(AxlePosition::RearLeft.is_drive_axle(true));
    }

    #[test]
    fn test_condition_excellent() {
        let s = AxleSensor::new(AxlePosition::FrontLeft, 40.0, 10.0, 10.0, 0.1);
        assert_eq!(s.condition(), AxleCondition::Excellent);
    }

    #[test]
    fn test_condition_critical() {
        let s = AxleSensor::new(AxlePosition::RearRight, 130.0, 60.0, 95.0, 2.0);
        assert_eq!(s.condition(), AxleCondition::Critical);
    }

    #[test]
    fn test_overheating() {
        let s = AxleSensor::new(AxlePosition::FrontRight, 110.0, 20.0, 30.0, 0.5);
        assert!(s.is_overheating());
    }

    #[test]
    fn test_misaligned() {
        let s = AxleSensor::new(AxlePosition::FrontLeft, 50.0, 15.0, 20.0, 1.5);
        assert!(s.is_misaligned());
    }

    #[test]
    fn test_remaining_life() {
        let s = AxleSensor::new(AxlePosition::RearLeft, 50.0, 10.0, 50.0, 0.2);
        assert!((s.remaining_life_km() - 75000.0).abs() < 100.0);
    }

    #[test]
    fn test_service_urgency() {
        let s = AxleSensor::new(AxlePosition::FrontLeft, 40.0, 10.0, 10.0, 0.1);
        assert_eq!(s.service_urgency(), "none");
    }

    #[test]
    fn test_health_score() {
        let s = AxleSensor::new(AxlePosition::FrontLeft, 40.0, 10.0, 10.0, 0.1);
        assert!(s.health_score() > 80.0);
    }

    #[test]
    fn test_set_worst() {
        let mut set = AxleSet::new();
        set.add(AxleSensor::new(
            AxlePosition::FrontLeft,
            40.0,
            10.0,
            10.0,
            0.1,
        ));
        set.add(AxleSensor::new(
            AxlePosition::RearRight,
            130.0,
            60.0,
            95.0,
            2.0,
        ));
        assert_eq!(set.worst_condition(), AxleCondition::Critical);
    }

    #[test]
    fn test_set_overheating() {
        let mut set = AxleSet::new();
        set.add(AxleSensor::new(
            AxlePosition::FrontLeft,
            40.0,
            10.0,
            10.0,
            0.1,
        ));
        set.add(AxleSensor::new(
            AxlePosition::RearLeft,
            105.0,
            20.0,
            30.0,
            0.5,
        ));
        assert!(set.any_overheating());
    }

    #[test]
    fn test_max_safe_speed() {
        let mut set = AxleSet::new();
        set.add(AxleSensor::new(
            AxlePosition::FrontLeft,
            40.0,
            10.0,
            10.0,
            0.1,
        ));
        set.add(AxleSensor::new(
            AxlePosition::RearRight,
            90.0,
            40.0,
            75.0,
            1.0,
        ));
        assert!(set.max_safe_speed() <= 80.0);
    }

    #[test]
    fn test_average_health() {
        let mut set = AxleSet::new();
        set.add(AxleSensor::new(
            AxlePosition::FrontLeft,
            40.0,
            10.0,
            10.0,
            0.1,
        ));
        assert!(set.average_health() > 50.0);
    }
}
