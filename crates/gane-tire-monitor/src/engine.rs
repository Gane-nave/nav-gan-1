/// Tire monitoring: pressure, temperature, tread depth, blowout prediction.
#[derive(Debug, Clone, PartialEq)]
pub enum TirePosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    Spare,
}

impl TirePosition {
    pub fn is_front(&self) -> bool {
        matches!(self, TirePosition::FrontLeft | TirePosition::FrontRight)
    }

    pub fn is_drive_axle(&self) -> bool {
        matches!(self, TirePosition::RearLeft | TirePosition::RearRight)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TireCondition {
    Excellent,
    Good,
    Fair,
    Worn,
    Critical,
    Flat,
}

impl TireCondition {
    pub fn safe_for_highway(&self) -> bool {
        matches!(
            self,
            TireCondition::Excellent | TireCondition::Good | TireCondition::Fair
        )
    }

    pub fn needs_replacement(&self) -> bool {
        matches!(self, TireCondition::Critical | TireCondition::Flat)
    }
}

#[derive(Debug, Clone)]
pub struct TireSensor {
    pub position: TirePosition,
    pub pressure_psi: f64,
    pub temperature_c: f64,
    pub tread_depth_mm: f64,
    pub recommended_psi: f64,
}

impl TireSensor {
    pub fn new(position: TirePosition) -> Self {
        Self {
            position,
            pressure_psi: 35.0,
            temperature_c: 30.0,
            tread_depth_mm: 8.0,
            recommended_psi: 35.0,
        }
    }

    pub fn pressure_deviation_pct(&self) -> f64 {
        if self.recommended_psi <= 0.0 {
            return 0.0;
        }
        ((self.pressure_psi - self.recommended_psi) / self.recommended_psi * 100.0).abs()
    }

    pub fn is_underinflated(&self) -> bool {
        self.pressure_psi < self.recommended_psi * 0.85
    }

    pub fn is_overinflated(&self) -> bool {
        self.pressure_psi > self.recommended_psi * 1.15
    }

    pub fn condition(&self) -> TireCondition {
        if self.pressure_psi < 10.0 {
            return TireCondition::Flat;
        }
        if self.tread_depth_mm < 1.6 {
            return TireCondition::Critical;
        }
        if self.tread_depth_mm < 3.0 {
            return TireCondition::Worn;
        }
        if self.tread_depth_mm < 5.0 {
            return TireCondition::Fair;
        }
        if self.tread_depth_mm < 7.0 {
            return TireCondition::Good;
        }
        TireCondition::Excellent
    }

    pub fn blowout_risk(&self) -> f64 {
        let pressure_risk = if self.is_underinflated() { 40.0 } else { 0.0 };
        let temp_risk = if self.temperature_c > 80.0 { 30.0 } else { 0.0 };
        let tread_risk = if self.tread_depth_mm < 2.0 { 30.0 } else { 0.0 };
        let total: f64 = pressure_risk + temp_risk + tread_risk;
        total.clamp(0.0, 100.0)
    }

    pub fn remaining_life_km(&self) -> f64 {
        if self.tread_depth_mm <= 1.6 {
            return 0.0;
        }
        (self.tread_depth_mm - 1.6) * 10000.0
    }

    pub fn max_safe_speed_kmh(&self) -> f64 {
        if self.condition().needs_replacement() {
            return 30.0;
        }
        if self.is_underinflated() {
            return 80.0;
        }
        if self.temperature_c > 90.0 {
            return 80.0;
        }
        200.0
    }
}

#[derive(Debug, Clone)]
pub struct TireSet {
    pub tires: Vec<TireSensor>,
}

impl Default for TireSet {
    fn default() -> Self {
        Self::new()
    }
}

impl TireSet {
    pub fn new() -> Self {
        Self { tires: Vec::new() }
    }

    pub fn add(&mut self, t: TireSensor) {
        self.tires.push(t);
    }

    pub fn all_ok(&self) -> bool {
        self.tires.iter().all(|t| t.condition().safe_for_highway())
    }

    pub fn worst_condition(&self) -> Option<TireCondition> {
        self.tires
            .iter()
            .map(|t| t.condition())
            .max_by_key(|c| match c {
                TireCondition::Flat => 5,
                TireCondition::Critical => 4,
                TireCondition::Worn => 3,
                TireCondition::Fair => 2,
                TireCondition::Good => 1,
                TireCondition::Excellent => 0,
            })
    }

    pub fn max_blowout_risk(&self) -> f64 {
        self.tires
            .iter()
            .map(|t| t.blowout_risk())
            .fold(0.0_f64, f64::max)
    }

    pub fn min_safe_speed(&self) -> f64 {
        self.tires
            .iter()
            .map(|t| t.max_safe_speed_kmh())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn pressure_balanced(&self) -> bool {
        let fronts: Vec<_> = self
            .tires
            .iter()
            .filter(|t| t.position.is_front())
            .collect();
        if fronts.len() < 2 {
            return true;
        }
        (fronts[0].pressure_psi - fronts[1].pressure_psi).abs() < 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front_position() {
        assert!(TirePosition::FrontLeft.is_front());
        assert!(!TirePosition::RearLeft.is_front());
    }

    #[test]
    fn test_condition_excellent() {
        let t = TireSensor::new(TirePosition::FrontLeft);
        assert_eq!(t.condition(), TireCondition::Excellent);
    }

    #[test]
    fn test_condition_critical() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.tread_depth_mm = 1.0;
        assert_eq!(t.condition(), TireCondition::Critical);
    }

    #[test]
    fn test_underinflated() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.pressure_psi = 25.0;
        assert!(t.is_underinflated());
    }

    #[test]
    fn test_overinflated() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.pressure_psi = 45.0;
        assert!(t.is_overinflated());
    }

    #[test]
    fn test_blowout_risk_normal() {
        let t = TireSensor::new(TirePosition::FrontLeft);
        assert!(t.blowout_risk() < 10.0);
    }

    #[test]
    fn test_blowout_risk_high() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.pressure_psi = 20.0;
        t.temperature_c = 90.0;
        t.tread_depth_mm = 1.0;
        assert!(t.blowout_risk() > 80.0);
    }

    #[test]
    fn test_remaining_life() {
        let t = TireSensor::new(TirePosition::FrontLeft);
        assert!(t.remaining_life_km() > 50000.0);
    }

    #[test]
    fn test_remaining_life_worn() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.tread_depth_mm = 1.6;
        assert!(t.remaining_life_km() < 1.0);
    }

    #[test]
    fn test_set_all_ok() {
        let mut s = TireSet::new();
        s.add(TireSensor::new(TirePosition::FrontLeft));
        s.add(TireSensor::new(TirePosition::FrontRight));
        assert!(s.all_ok());
    }

    #[test]
    fn test_set_not_ok() {
        let mut s = TireSet::new();
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.tread_depth_mm = 1.0;
        s.add(t);
        assert!(!s.all_ok());
    }

    #[test]
    fn test_pressure_balanced() {
        let mut s = TireSet::new();
        s.add(TireSensor::new(TirePosition::FrontLeft));
        s.add(TireSensor::new(TirePosition::FrontRight));
        assert!(s.pressure_balanced());
    }

    #[test]
    fn test_max_safe_speed_critical() {
        let mut t = TireSensor::new(TirePosition::FrontLeft);
        t.tread_depth_mm = 1.0;
        assert!((t.max_safe_speed_kmh() - 30.0).abs() < 0.01);
    }
}
