/// Grade/slope assistance for hill driving
/// Phase 131: Manages hill descent control, grade braking, engine braking

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradeType {
    Flat,
    Gentle,
    Moderate,
    Steep,
    Extreme,
}

impl GradeType {
    pub fn from_percent(grade: f64) -> Self {
        let abs_grade = grade.abs();
        if abs_grade < 2.0 {
            GradeType::Flat
        } else if abs_grade < 6.0 {
            GradeType::Gentle
        } else if abs_grade < 10.0 {
            GradeType::Moderate
        } else if abs_grade < 15.0 {
            GradeType::Steep
        } else {
            GradeType::Extreme
        }
    }

    pub fn max_safe_speed_kmh(&self) -> f64 {
        match self {
            GradeType::Flat => 120.0,
            GradeType::Gentle => 100.0,
            GradeType::Moderate => 80.0,
            GradeType::Steep => 60.0,
            GradeType::Extreme => 40.0,
        }
    }

    pub fn engine_brake_recommended(&self) -> bool {
        matches!(self, GradeType::Steep | GradeType::Extreme)
    }
}

#[derive(Debug, Clone)]
pub struct GradeSensor {
    pub grade_percent: f64,
    pub length_m: f64,
    pub vehicle_weight_kg: f64,
    pub current_speed_kmh: f64,
}

impl GradeSensor {
    pub fn new(grade_percent: f64, length_m: f64, weight_kg: f64, speed: f64) -> Self {
        Self {
            grade_percent,
            length_m,
            vehicle_weight_kg: weight_kg,
            current_speed_kmh: speed,
        }
    }

    pub fn grade_type(&self) -> GradeType {
        GradeType::from_percent(self.grade_percent)
    }

    pub fn is_uphill(&self) -> bool {
        self.grade_percent > 0.0
    }

    pub fn is_downhill(&self) -> bool {
        self.grade_percent < 0.0
    }

    pub fn elevation_change_m(&self) -> f64 {
        self.length_m * (self.grade_percent / 100.0)
    }

    pub fn gravitational_force_n(&self) -> f64 {
        let angle_rad = (self.grade_percent / 100.0).atan();
        self.vehicle_weight_kg * 9.81 * angle_rad.sin()
    }

    pub fn braking_distance_m(&self) -> f64 {
        let speed_ms = self.current_speed_kmh / 3.6;
        let friction = 0.7;
        let grade_rad = (self.grade_percent / 100.0).atan();
        let decel = 9.81 * (friction * grade_rad.cos() - grade_rad.sin());
        if decel <= 0.0 {
            return f64::MAX;
        }
        (speed_ms * speed_ms) / (2.0 * decel)
    }

    pub fn recommended_gear(&self) -> u8 {
        match self.grade_type() {
            GradeType::Flat => 6,
            GradeType::Gentle => 5,
            GradeType::Moderate => 4,
            GradeType::Steep => 3,
            GradeType::Extreme => 2,
        }
    }

    pub fn hill_hold_needed(&self) -> bool {
        self.is_uphill() && self.grade_percent > 5.0
    }

    pub fn descent_control_needed(&self) -> bool {
        self.is_downhill() && self.grade_percent.abs() > 8.0
    }

    pub fn fuel_penalty_factor(&self) -> f64 {
        if self.is_uphill() {
            1.0 + (self.grade_percent / 100.0) * 2.0
        } else {
            let factor: f64 = 1.0 - (self.grade_percent.abs() / 100.0) * 0.5;
            factor.max(0.5)
        }
    }

    pub fn traverse_time_sec(&self) -> f64 {
        let safe_speed = self
            .grade_type()
            .max_safe_speed_kmh()
            .min(self.current_speed_kmh);
        let speed_ms = safe_speed / 3.6;
        if speed_ms <= 0.0 {
            return f64::MAX;
        }
        self.length_m / speed_ms
    }
}

#[derive(Debug, Clone)]
pub struct GradeRoute {
    pub segments: Vec<GradeSensor>,
}

impl Default for GradeRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl GradeRoute {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, seg: GradeSensor) {
        self.segments.push(seg);
    }

    pub fn total_elevation_gain_m(&self) -> f64 {
        self.segments
            .iter()
            .filter(|s| s.is_uphill())
            .map(|s| s.elevation_change_m())
            .sum()
    }

    pub fn total_elevation_loss_m(&self) -> f64 {
        self.segments
            .iter()
            .filter(|s| s.is_downhill())
            .map(|s| s.elevation_change_m().abs())
            .sum()
    }

    pub fn max_grade_percent(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.grade_percent.abs())
            .fold(0.0_f64, f64::max)
    }

    pub fn total_distance_m(&self) -> f64 {
        self.segments.iter().map(|s| s.length_m).sum()
    }

    pub fn steep_segment_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| matches!(s.grade_type(), GradeType::Steep | GradeType::Extreme))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_flat() {
        assert_eq!(GradeType::from_percent(1.0), GradeType::Flat);
    }

    #[test]
    fn test_grade_steep() {
        assert_eq!(GradeType::from_percent(12.0), GradeType::Steep);
    }

    #[test]
    fn test_max_speed_extreme() {
        assert_eq!(GradeType::Extreme.max_safe_speed_kmh(), 40.0);
    }

    #[test]
    fn test_engine_brake() {
        assert!(GradeType::Steep.engine_brake_recommended());
        assert!(!GradeType::Gentle.engine_brake_recommended());
    }

    #[test]
    fn test_uphill() {
        let s = GradeSensor::new(8.0, 500.0, 2000.0, 60.0);
        assert!(s.is_uphill());
        assert!(!s.is_downhill());
    }

    #[test]
    fn test_downhill() {
        let s = GradeSensor::new(-10.0, 1000.0, 2000.0, 50.0);
        assert!(s.is_downhill());
    }

    #[test]
    fn test_elevation_change() {
        let s = GradeSensor::new(10.0, 1000.0, 2000.0, 60.0);
        assert!((s.elevation_change_m() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_braking_distance() {
        let s = GradeSensor::new(0.0, 500.0, 1500.0, 100.0);
        assert!(s.braking_distance_m() > 0.0);
        assert!(s.braking_distance_m() < 200.0);
    }

    #[test]
    fn test_recommended_gear() {
        let s = GradeSensor::new(12.0, 500.0, 2000.0, 50.0);
        assert_eq!(s.recommended_gear(), 3);
    }

    #[test]
    fn test_hill_hold() {
        let s = GradeSensor::new(7.0, 200.0, 1500.0, 0.0);
        assert!(s.hill_hold_needed());
    }

    #[test]
    fn test_descent_control() {
        let s = GradeSensor::new(-12.0, 1000.0, 2000.0, 40.0);
        assert!(s.descent_control_needed());
    }

    #[test]
    fn test_fuel_penalty_uphill() {
        let s = GradeSensor::new(10.0, 500.0, 2000.0, 60.0);
        assert!(s.fuel_penalty_factor() > 1.0);
    }

    #[test]
    fn test_fuel_penalty_downhill() {
        let s = GradeSensor::new(-10.0, 500.0, 2000.0, 60.0);
        assert!(s.fuel_penalty_factor() < 1.0);
    }

    #[test]
    fn test_route_elevation() {
        let mut r = GradeRoute::new();
        r.add_segment(GradeSensor::new(10.0, 1000.0, 2000.0, 60.0));
        r.add_segment(GradeSensor::new(-5.0, 500.0, 2000.0, 60.0));
        assert!((r.total_elevation_gain_m() - 100.0).abs() < 0.1);
        assert!((r.total_elevation_loss_m() - 25.0).abs() < 0.1);
    }

    #[test]
    fn test_steep_count() {
        let mut r = GradeRoute::new();
        r.add_segment(GradeSensor::new(3.0, 100.0, 1500.0, 60.0));
        r.add_segment(GradeSensor::new(14.0, 200.0, 1500.0, 40.0));
        assert_eq!(r.steep_segment_count(), 1);
    }
}
