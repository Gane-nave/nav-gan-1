/// Climate zone control: multi-zone HVAC, temperature regulation, air quality
/// Phase 138

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClimateZoneId {
    Driver,
    Passenger,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AirflowMode {
    Face,
    Feet,
    Defrost,
    FaceFeet,
    FeetDefrost,
}

#[derive(Debug, Clone)]
pub struct ClimateZone {
    pub id: ClimateZoneId,
    pub target_temp_c: f64,
    pub current_temp_c: f64,
    pub fan_speed: u8,
    pub airflow: AirflowMode,
    pub seat_heating: u8,
    pub seat_cooling: u8,
}

impl ClimateZone {
    pub fn new(id: ClimateZoneId, target: f64) -> Self {
        Self {
            id,
            target_temp_c: target,
            current_temp_c: 22.0,
            fan_speed: 3,
            airflow: AirflowMode::Face,
            seat_heating: 0,
            seat_cooling: 0,
        }
    }
    pub fn temp_error(&self) -> f64 {
        self.target_temp_c - self.current_temp_c
    }
    pub fn needs_heating(&self) -> bool {
        self.temp_error() > 1.0
    }
    pub fn needs_cooling(&self) -> bool {
        self.temp_error() < -1.0
    }
    pub fn at_target(&self) -> bool {
        self.temp_error().abs() <= 1.0
    }
    pub fn comfort_score(&self) -> f64 {
        let temp_score: f64 = (100.0 - self.temp_error().abs() * 10.0).max(0.0);
        temp_score.min(100.0)
    }
    pub fn power_draw_watts(&self) -> f64 {
        let fan_power = self.fan_speed as f64 * 50.0;
        let heat_cool = if self.needs_heating() {
            200.0
        } else if self.needs_cooling() {
            300.0
        } else {
            50.0
        };
        let seat_power = (self.seat_heating as f64 + self.seat_cooling as f64) * 40.0;
        fan_power + heat_cool + seat_power
    }
}

#[derive(Debug, Clone)]
pub struct ClimateSystem {
    pub zones: Vec<ClimateZone>,
    pub recirculate: bool,
    pub auto_mode: bool,
}

impl Default for ClimateSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ClimateSystem {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(),
            recirculate: false,
            auto_mode: true,
        }
    }
    pub fn add_zone(&mut self, z: ClimateZone) {
        self.zones.push(z);
    }
    pub fn all_at_target(&self) -> bool {
        self.zones.iter().all(|z| z.at_target())
    }
    pub fn total_power_watts(&self) -> f64 {
        self.zones.iter().map(|z| z.power_draw_watts()).sum()
    }
    pub fn average_comfort(&self) -> f64 {
        if self.zones.is_empty() {
            return 100.0;
        }
        let total: f64 = self.zones.iter().map(|z| z.comfort_score()).sum();
        total / self.zones.len() as f64
    }
    pub fn max_temp_error(&self) -> f64 {
        self.zones
            .iter()
            .map(|z| z.temp_error().abs())
            .fold(0.0_f64, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_needs_heating() {
        let z = ClimateZone::new(ClimateZoneId::Driver, 25.0);
        assert!(z.needs_heating());
    }
    #[test]
    fn test_needs_cooling() {
        let mut z = ClimateZone::new(ClimateZoneId::Passenger, 18.0);
        z.current_temp_c = 25.0;
        assert!(z.needs_cooling());
    }
    #[test]
    fn test_at_target() {
        let z = ClimateZone::new(ClimateZoneId::Driver, 22.0);
        assert!(z.at_target());
    }
    #[test]
    fn test_comfort() {
        let z = ClimateZone::new(ClimateZoneId::Driver, 22.0);
        assert!(z.comfort_score() > 90.0);
    }
    #[test]
    fn test_power_draw() {
        let z = ClimateZone::new(ClimateZoneId::Driver, 25.0);
        assert!(z.power_draw_watts() > 0.0);
    }
    #[test]
    fn test_system_all_target() {
        let mut s = ClimateSystem::new();
        s.add_zone(ClimateZone::new(ClimateZoneId::Driver, 22.0));
        assert!(s.all_at_target());
    }
    #[test]
    fn test_system_not_target() {
        let mut s = ClimateSystem::new();
        s.add_zone(ClimateZone::new(ClimateZoneId::Driver, 30.0));
        assert!(!s.all_at_target());
    }
    #[test]
    fn test_total_power() {
        let mut s = ClimateSystem::new();
        s.add_zone(ClimateZone::new(ClimateZoneId::Driver, 22.0));
        s.add_zone(ClimateZone::new(ClimateZoneId::Passenger, 22.0));
        assert!(s.total_power_watts() > 0.0);
    }
    #[test]
    fn test_avg_comfort() {
        let mut s = ClimateSystem::new();
        s.add_zone(ClimateZone::new(ClimateZoneId::Driver, 22.0));
        assert!(s.average_comfort() > 50.0);
    }
    #[test]
    fn test_max_error() {
        let mut s = ClimateSystem::new();
        s.add_zone(ClimateZone::new(ClimateZoneId::Driver, 30.0));
        assert!(s.max_temp_error() > 5.0);
    }
}
